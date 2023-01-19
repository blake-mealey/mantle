import { JSONSchema7 } from 'json-schema';
import { mkdir, readdir, readFile, rm, writeFile } from 'fs/promises';
import { existsSync } from 'fs';
import { join } from 'path';
import { Octokit } from '@octokit/rest';
import { isDefined } from './is-defined';
import chalk from 'chalk-template';
import { remark } from 'remark';
import remarkGfm from 'remark-gfm';
import remarkMdx from 'remark-mdx';
import { translateToNextra } from './remark-plugins/translate-to-nextra';
import { createSchemaTransformer } from './transform-schema-markdown';

export type QualifiedSchema = Exclude<JSONSchema7, boolean>;

export interface Release {
  version: string;
  configurationSchema: QualifiedSchema;
}

const CACHE_DIR = join(__dirname, '.releases-cache');
const GITHUB_OWNER = 'blake-mealey';
const GITHUB_REPO = 'mantle';

async function loadFromGitHub(): Promise<Release[]> {
  const client = new Octokit({ auth: process.env.GITHUB_TOKEN });

  const repoParams = { owner: GITHUB_OWNER, repo: GITHUB_REPO };

  const githubReleases = await client.paginate(
    client.rest.repos.listReleases,
    repoParams
  );

  const processor = remark()
    .use(remarkGfm)
    .use(translateToNextra)
    .use(remarkMdx);
  const transformSchema = createSchemaTransformer(processor);

  return (
    await Promise.all(
      githubReleases.map(async (githubRelease) => {
        const asset_id = githubRelease.assets.find(
          (asset) => asset.name === 'schema.json'
        )?.id;

        if (!asset_id) return;

        const response = await client.rest.repos.getReleaseAsset({
          ...repoParams,
          asset_id,
          headers: {
            accept: 'application/octet-stream',
          },
        });

        const configurationSchema = JSON.parse(
          Buffer.from(response.data as any).toString('utf8')
        );
        await transformSchema(configurationSchema);

        return {
          version: githubRelease.tag_name,
          configurationSchema,
        };
      })
    )
  ).filter(isDefined);
}

async function loadFromCache(): Promise<Release[] | undefined> {
  if (!existsSync(CACHE_DIR)) {
    return undefined;
  }

  const versions = await readdir(CACHE_DIR);
  if (versions.length === 0) {
    return undefined;
  }

  return Promise.all(
    versions.map(async (version) => {
      const configurationSchema = await readFile(
        join(CACHE_DIR, version, 'configurationSchema.json'),
        'utf8'
      );
      return {
        version,
        configurationSchema: JSON.parse(configurationSchema),
      };
    })
  );
}

async function saveToCache(releases: Release[]) {
  console.log(`Saving ${releases.length} releases to cache...`);

  if (existsSync(CACHE_DIR)) {
    await rm(CACHE_DIR, { recursive: true });
  }
  await mkdir(CACHE_DIR);

  await Promise.all(
    releases.map(async (release) => {
      const versionDir = join(CACHE_DIR, release.version);

      console.log(
        chalk`{grey Saving} ${release.version} {grey to} ${versionDir}`
      );

      await mkdir(versionDir, { recursive: true });
      await writeFile(
        join(versionDir, 'configurationSchema.json'),
        JSON.stringify(release.configurationSchema)
      );
    })
  );
}

export async function refreshReleasesCache() {
  const releases = await loadFromGitHub();
  saveToCache(releases);
}

export async function getReleases() {
  let releases = await loadFromCache();
  if (!releases) {
    releases = await loadFromGitHub();
    await saveToCache(releases);
  }
  return loadFromGitHub();
}
