import {
  flattenSchemaProperties,
  SchemaProperty,
} from './flatten-schema-properties';
import { getReleases } from './releases';
import { CompileMdx } from './types';

export interface ProcessedSchema {
  version: string;
  properties: SchemaProperty[];
}

export async function getLatestSchema(compileMdx: CompileMdx) {
  const releases = await getReleases();
  const latestRelease = releases[0];
  if (!latestRelease) {
    throw new Error('No latest release');
  }
  return {
    version: latestRelease.version,
    properties: await flattenSchemaProperties(
      compileMdx,
      latestRelease.configurationSchema
    ),
  };
}
