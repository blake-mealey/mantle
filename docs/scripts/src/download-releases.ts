import { refreshReleasesCache } from 'lib';
import { config } from 'dotenv';

config();

async function main() {
  await refreshReleasesCache();
}

main().catch(console.error);
