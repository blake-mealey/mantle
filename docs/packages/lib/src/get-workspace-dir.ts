import { findWorkspaceDir } from '@pnpm/find-workspace-dir';
import { join } from 'path';

let workspaceDir: string | undefined;

export async function getWorkspaceDir(dirName?: string) {
  if (!workspaceDir) {
    workspaceDir = await findWorkspaceDir('');
  }
  if (workspaceDir === undefined) {
    throw new Error('Could not find workspace dir');
  }
  return dirName ? join(workspaceDir, dirName) : workspaceDir;
}
