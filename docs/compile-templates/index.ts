import {
  readFileSync,
  writeFileSync,
  readdirSync,
  mkdirSync,
  existsSync,
} from 'node:fs';
import path from 'node:path';
import Handlebars from 'handlebars';
import { JSONSchema7 } from 'json-schema';
import flattenProperties from './flattenProperties';
import { createSlugger } from '@docusaurus/utils';
import { Octokit } from '@octokit/rest';
import semver from 'semver';
import { simplifySchemaMarkdown } from './simplifySchemaMarkdown';

const docsDir = path.join(__dirname, '../docs');

function stringifyJson(data: any) {
  const env = process.env.NODE_ENV?.toLowerCase();

  if (env === 'production') {
    return JSON.stringify(data);
  } else {
    return JSON.stringify(data, null, 2);
  }
}

function registerIncludes() {
  const includesDir = path.join(docsDir, 'includes');
  const includes = readdirSync(includesDir);
  includes.forEach((file) => {
    const contents = readFileSync(path.join(includesDir, file), 'utf-8');
    Handlebars.registerPartial(path.parse(file).name, contents);
  });
}

function registerHelpers() {
  Handlebars.registerHelper('repeat', require('handlebars-helper-repeat'));

  const slugs = createSlugger();
  Handlebars.registerHelper('anchor', (id: string) => {
    return `{#${slugs.slug(id.replace(/\./g, '-'))}}`;
  });
}

function compileTemplates(context: any) {
  readdirSync(docsDir)
    .filter((file) => file.endsWith('.hbs'))
    .map((file) => path.join(docsDir, file))
    .forEach((file) => {
      console.log('Compiling:\t', file);

      const source = readFileSync(file, 'utf-8');
      const template = Handlebars.compile(source, { noEscape: true });
      const markdown = template({
        ...context,
        sourceFile: path.relative(docsDir, file),
      });

      const outFile = path.format({
        ...path.parse(file),
        base: '',
        ext: '.md',
      });
      console.log('Writing:\t', outFile);
      writeFileSync(outFile, markdown, 'utf-8');
    });
}

const cacheDir = path.join(
  __dirname,
  '../node_modules/.cache/compile-templates'
);

function readCache<T>(key: string) {
  try {
    const contents = readFileSync(path.join(cacheDir, `${key}.json`), 'utf-8');
    console.log(
      `Loaded ${key} from cache. Run \`rm -r node_modules/.cache/compile-templates\` to clear cache.`
    );

    return JSON.parse(contents) as T;
  } catch {}
}

function writeCache(key: string, data: any) {
  mkdirSync(cacheDir, { recursive: true });
  writeFileSync(
    path.join(cacheDir, `${key}.json`),
    stringifyJson(data),
    'utf-8'
  );
}

interface SchemasCache {
  schemas: Record<string, JSONSchema7>;
  latestVersion: string;
}

function withNextVersion(schemasCache: SchemasCache) {
  const schemaPath = path.join(__dirname, '../schema.json');
  if (existsSync(schemaPath)) {
    console.log('Adding `next` schema version.');

    const nextSchema = JSON.parse(
      readFileSync(schemaPath, 'utf-8')
    ) as JSONSchema7;

    schemasCache.schemas['next'] = nextSchema;
    schemasCache.latestVersion = 'next';
  }
  return schemasCache;
}

async function getSchemas() {
  const cached = readCache<SchemasCache>('schemas');
  if (cached) {
    return withNextVersion(cached);
  }

  const client = new Octokit();
  const repoParams = { owner: 'blake-mealey', repo: 'mantle' };

  const { data: releases } = await client.rest.repos.listReleases({
    ...repoParams,
  });

  const schemas: Record<string, JSONSchema7> = {};

  for (const release of releases) {
    // schemas are only available starting with v0.11.0
    if (semver.lt(release.name, '0.11.0')) {
      break;
    }

    const asset_id = release.assets.find(
      (asset) => asset.name === 'schema.json'
    )?.id;

    const response = await client.rest.repos.getReleaseAsset({
      ...repoParams,
      asset_id,
      headers: {
        accept: 'application/octet-stream',
      },
    });

    schemas[release.name] = JSON.parse(
      Buffer.from(response.data as any).toString('utf8')
    );
  }

  const result: SchemasCache = { schemas, latestVersion: releases[0].name };

  console.log('Loaded schemas from API.');
  writeCache('schemas', result);

  return withNextVersion(result);
}

function createContext(
  schemas: Record<string, JSONSchema7>,
  latestVersion: string
) {
  return {
    properties: flattenProperties(schemas[latestVersion]),
    schemaVersion: latestVersion,
  };
}

async function saveSchemas(schemas: Record<string, JSONSchema7>) {
  const schemasDir = path.join(__dirname, '../static/schemas');

  for (const [version, schema] of Object.entries(schemas)) {
    const versionDir = path.join(schemasDir, version);
    mkdirSync(versionDir, { recursive: true });

    await simplifySchemaMarkdown(schema);

    const file = path.join(versionDir, 'schema.json');
    console.log('Writing:\t', file);
    writeFileSync(file, stringifyJson(schema), 'utf-8');
  }
}

async function main() {
  registerIncludes();
  registerHelpers();

  const { latestVersion, schemas } = await getSchemas();

  const context = createContext(schemas, latestVersion);
  compileTemplates(context);

  saveSchemas(schemas);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
