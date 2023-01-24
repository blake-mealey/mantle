import { JSONSchema7, JSONSchema7Definition } from 'json-schema';
import { Processor } from 'unified';

interface SchemaTransformerOptions {
  propagateTitle: boolean;
  descriptionKey: string;
}

export function createSchemaTransformer(
  processor: Processor,
  options?: SchemaTransformerOptions
) {
  const propagateTitle = options?.propagateTitle ?? false;
  const descriptionKey = options?.descriptionKey ?? 'description';

  const transform = async (
    schema: JSONSchema7 | JSONSchema7Definition,
    title?: string
  ) => {
    if (typeof schema === 'boolean') {
      return;
    }

    if (propagateTitle && title) {
      schema.title = title;
    }

    if (schema.description) {
      const { value } = await processor.process(schema.description);
      (schema as Record<string, any>)[descriptionKey] = value.toString();
    }

    const changes: Promise<unknown>[] = [];

    if (schema.properties) {
      changes.push(
        ...Object.entries(schema.properties).map(([name, definition]) =>
          transform(definition, name)
        )
      );
    }

    if (schema.oneOf) {
      changes.push(...schema.oneOf.map((definition) => transform(definition)));
    }

    if (schema.anyOf) {
      changes.push(...schema.anyOf.map((definition) => transform(definition)));
    }

    if (typeof schema.items === 'object') {
      const items = Array.isArray(schema.items) ? schema.items : [schema.items];
      changes.push(...items.map((definition) => transform(definition)));
    }

    if (typeof schema.additionalProperties === 'object') {
      changes.push(transform(schema.additionalProperties));
    }

    return changes;
  };

  return transform;
}
