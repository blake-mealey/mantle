import { refreshReleasesCache } from 'lib';
import { config } from 'dotenv';

config();

refreshReleasesCache().catch(console.error);
