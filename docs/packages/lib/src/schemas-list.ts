import { getReleases } from './releases';
import { CompileMdx } from './types';

export interface SchemaVersionItem {
  version: string;
  url: string;
}

export async function getSchemaVersionsList(): Promise<SchemaVersionItem[]> {
  const releases = await getReleases();
  return releases.map((release) => ({
    version: release.version,
    url: `/schemas/${release.version}/schema.json`,
  }));
}

export async function getSchemasSnippetContent(compileMdx: CompileMdx) {
  const releases = await getReleases();
  const vscodeSnippet = `\`\`\`json
"yaml.schemas": {
  "https://mantledeploy.vercel.app/schemas/${releases[0]?.version}/schema.json": "mantle.yml"
}
\`\`\``;
  return (await compileMdx(vscodeSnippet)).result;
}
