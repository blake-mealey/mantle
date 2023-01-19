import { Schema } from 'lib';
import { useSSG } from 'nextra/ssg';
import { SchemaReference } from './schema-reference';

export function LatestSchemaReference() {
  const { latestSchema } = useSSG() as { latestSchema: Schema };

  return <SchemaReference schema={latestSchema} />;
}
