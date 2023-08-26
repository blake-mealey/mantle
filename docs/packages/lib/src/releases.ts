import { JSONSchema7 } from 'json-schema';
import { mkdir, readdir, readFile, rm, writeFile } from 'fs/promises';
import { existsSync } from 'fs';
import { join } from 'path';
import { exec } from 'child_process';
import { Octokit } from '@octokit/rest';
import { isDefined } from './is-defined';
import chalk from 'chalk-template';
import { getTranslateToNextraTransformer } from './schema-transformers';
import { getWorkspaceDir } from './get-workspace-dir';
import { gt } from 'semver';

export type QualifiedSchema = Exclude<JSONSchema7, boolean>;

export interface Release {
  version: string;
  configurationSchema: QualifiedSchema;
}

const CACHE_DIR = '.releases-cache';
const GITHUB_OWNER = 'blake-mealey';
const GITHUB_REPO = 'mantle';

async function loadFromGitHub(): Promise<Release[]> {
  const client = new Octokit({ auth: process.env.GITHUB_TOKEN });

  const repoParams = { owner: GITHUB_OWNER, repo: GITHUB_REPO };

  const githubReleases = await client.paginate(
    client.rest.repos.listReleases,
    repoParams
  );

  // transformer to translate old schemas to new md syntax
  const transformSchema = getTranslateToNextraTransformer();

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
  const cacheDir = await getWorkspaceDir(CACHE_DIR);
  if (!existsSync(cacheDir)) {
    return undefined;
  }

  const versions = await readdir(cacheDir);
  if (versions.length === 0) {
    return undefined;
  }

  const releases = await Promise.all(
    versions.map(async (version) => {
      const versionDir = join(cacheDir, version);
      console.log(chalk`{grey Loading} ${version} {grey from} ${versionDir}`);

      const configurationSchema = await readFile(
        join(versionDir, 'configurationSchema.json'),
        'utf8'
      );
      return {
        version,
        configurationSchema: JSON.parse(configurationSchema),
      };
    })
  );

  console.log(`Loaded ${releases.length} releases from cache`);

  return releases;
}

async function saveToCache(releases: Release[]) {
  console.log(`Saving ${releases.length} releases to cache...`);

  const cacheDir = await getWorkspaceDir(CACHE_DIR);
  if (existsSync(cacheDir)) {
    await rm(cacheDir, { recursive: true });
  }
  await mkdir(cacheDir);

  await Promise.all(
    releases.map(async (release) => {
      const versionDir = join(cacheDir, release.version);

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

// let releases: Release[] | undefined;

export async function refreshReleasesCache() {
  const releases = await loadFromGitHub();
  saveToCache(releases);
}

export async function getReleases() {
  let nextRelease: Release | undefined;
  if (process.env.NODE_ENV === 'development') {
    nextRelease = await getNextRelease();
  }

  console.log('Loading releases from cache...');
  let releases = await loadFromCache();
  if (!releases) {
    console.log('Cache miss');
    releases = await loadFromGitHub();
    await saveToCache(releases);
  }
  releases.sort((a, b) => {
    return gt(a.version, b.version) ? -1 : 1;
  });

  if (nextRelease) {
    releases.unshift(nextRelease);
  }
  return releases;
}

async function getNextRelease(): Promise<Release> {
  const workspaceDir = await getWorkspaceDir();
  const mantleDir = join(workspaceDir, '..', 'mantle');

  console.log(chalk`{grey Generating} next {grey from} ${mantleDir}`);

  const rawSchema = await new Promise<string>((resolve, reject) => {
    exec('cargo run --bin gen_schema', { cwd: mantleDir }, (err, stdout) => {
      if (err) {
        reject(err);
      }
      resolve(stdout);
    });
  });

  const configurationSchema = JSON.parse(rawSchema);

  const transformSchema = getTranslateToNextraTransformer();
  await transformSchema(configurationSchema);

  return {
    version: 'next',
    configurationSchema,
  };
}
