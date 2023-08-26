import type { SchemaVersionItem } from 'lib';
import { useSSG } from 'nextra/data';
import { FileText } from 'react-feather';
import { Card, Cards } from 'nextra/components';

export function SchemaVersionsList() {
  const { schemaVersionsList } = useSSG() as {
    schemaVersionsList: SchemaVersionItem[];
  };

  return (
    <Cards>
      {schemaVersionsList.map(({ version, url }) => (
        <Card
          icon={<FileText />}
          title={version}
          href={url}
          children={undefined}
        />
      ))}
    </Cards>
  );
}
