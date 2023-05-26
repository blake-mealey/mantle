import type { SchemaVersionItem } from 'lib';
import { useSSG } from 'nextra/data';
import { FileText } from 'react-feather';
import { LinksList } from './links-list';

export function SchemaVersionsList() {
  const { schemaVersionsList } = useSSG() as {
    schemaVersionsList: SchemaVersionItem[];
  };

  return (
    <LinksList
      links={schemaVersionsList.map(({ version, url }) => ({
        label: version,
        url,
        Icon: FileText,
      }))}
    />
  );
}
