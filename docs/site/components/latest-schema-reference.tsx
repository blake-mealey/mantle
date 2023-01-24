import { ProcessedSchema } from 'lib';
import { useSSG } from 'nextra/ssg';
import { SchemaReference } from './schema-reference';

export function LatestSchemaReference() {
  const { latestSchema } = useSSG() as { latestSchema: ProcessedSchema };

  return <SchemaReference schema={latestSchema} />;
}
