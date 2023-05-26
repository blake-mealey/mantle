import {
  getReleases,
  getTranslateToVscodeTransformer,
  getWorkspaceDir,
} from 'lib';
import { config } from 'dotenv';
import { writeFile, mkdir } from 'node:fs/promises';
import { join } from 'node:path';

config();

async function main() {
  const releases = await getReleases();

  const transformSchema = getTranslateToVscodeTransformer();

  const publicDir = await getWorkspaceDir(join('site', 'public'));

  return Promise.all(
    releases.map(async (release) => {
      await transformSchema(release.configurationSchema);
      const versionDir = join(publicDir, 'schemas', release.version);
      await mkdir(versionDir, { recursive: true });
      return writeFile(
        join(versionDir, 'schema.json'),
        JSON.stringify(release.configurationSchema),
        'utf8'
      );
    })
  );
}

main().catch(console.error);
