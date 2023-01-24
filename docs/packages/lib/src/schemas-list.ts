import { getReleases } from './releases';
import { CompileMdx } from './types';

export async function getSchemasListContent(compileMdx: CompileMdx) {
  const releases = await getReleases();
  const schemasList = releases
    .map(
      (release) =>
        `- [${release.version}](/schemas/${release.version}/schema.json)`
    )
    .join('\n');
  return (await compileMdx(schemasList)).result;
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
