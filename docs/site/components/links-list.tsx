import Link from 'next/link';
import { Icon } from 'react-feather';

interface LinksListProps {
  links: { label: string; url: string; Icon?: Icon }[];
}

export function LinksList({ links }: LinksListProps) {
  return (
    <ul className="grid grid-cols-2 gap-y-2 gap-x-4">
      {links.map((link) => (
        <li>
          <Link
            className="group flex items-center gap-2 w-full rounded-xl border border-neutral-200 dark:border-neutral-800 hover:border-neutral-300 dark:hover:border-neutral-600 transition-colors p-4 text-lg font-medium hover:bg-neutral-50 dark:hover:bg-neutral-900"
            href={link.url}
          >
            {link.Icon && (
              <link.Icon
                size={24}
                className="text-neutral-300 dark:text-neutral-600 group-hover:text-inherit transition-colors"
              />
            )}
            {link.label}
          </Link>
        </li>
      ))}
    </ul>
  );
}
