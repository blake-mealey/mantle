import { Octokit } from '@octokit/rest';
import { JSONSchema7 } from 'json-schema';
import {
  flattenSchemaProperties,
  SchemaProperty,
} from './flatten-schema-properties';

interface Schemas {
  schemas: Record<string, JSONSchema7>;
  latestVersion: string | undefined;
}

export async function getSchemas(): Promise<Schemas> {
  const client = new Octokit({ auth: process.env.GITHUB_TOKEN });

  const repoParams = { owner: 'blake-mealey', repo: 'mantle' };

  const releases = await client.paginate(
    client.rest.repos.listReleases,
    repoParams
  );

  const schemas: Record<string, JSONSchema7> = {};

  await Promise.all(
    releases.map(async (release) => {
      const asset_id = release.assets.find(
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

      schemas[release.tag_name] = JSON.parse(
        Buffer.from(response.data as any).toString('utf8')
      );
    })
  );

  return { schemas, latestVersion: releases[0]?.tag_name };
}

export interface Schema {
  version: string;
  properties: SchemaProperty[];
}

export async function processSchema({
  version,
  schema,
}: {
  version: string;
  schema: JSONSchema7;
}): Promise<Schema> {
  return {
    version,
    properties: await flattenSchemaProperties(schema),
  };
}
