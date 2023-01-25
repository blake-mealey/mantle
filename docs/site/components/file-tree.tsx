import { useMemo } from 'react';
import {
  Folder as FolderIcon,
  File as FileIcon,
  FileText as FileTextIcon,
  Image as ImageIcon,
  Icon,
} from 'react-feather';

interface FileTreeProps {
  children: React.ReactNode;
}

export function FileTree({ children }: FileTreeProps) {
  return (
    <div className="rounded-xl border border-black/20 dark:border-white/20 px-4 py-2 mt-6">
      {children}
    </div>
  );
}

interface FolderProps {
  name: string;
  children: React.ReactNode;
}

export function Folder({ children, name }: FolderProps) {
  return (
    <>
      <div className="flex items-center gap-2 font-medium py-1">
        <FolderIcon size={16} className="nx-text-gray-500" />
        {name}
        {'/'}
      </div>
      <div className="ml-8">{children}</div>
    </>
  );
}

const fileIcons: Record<string, Icon> = {
  png: ImageIcon,
  jpeg: ImageIcon,
  jpg: ImageIcon,
  gif: ImageIcon,
  svg: ImageIcon,
  bmp: ImageIcon,
  yml: FileTextIcon,
  yaml: FileTextIcon,
  json: FileTextIcon,
  txt: FileTextIcon,
  md: FileTextIcon,
};

function getFileExt(name: string) {
  const lastDot = name.lastIndexOf('.');
  if (lastDot !== -1) {
    return name.substring(lastDot + 1);
  }
}

interface FileProps {
  name: string;
}

export function File({ name }: FileProps) {
  const Icon = useMemo(() => {
    const ext = getFileExt(name);
    return (ext ? fileIcons[ext] : undefined) ?? FileIcon;
  }, [name]);

  return (
    <div className="flex items-center gap-2 font-medium py-1">
      <Icon size={16} className="nx-text-gray-500" />
      {name}
    </div>
  );
}
