import { JSONSchema7, JSONSchema7TypeName } from 'json-schema';
import { compileMdx } from 'nextra/compile';
import { translateToNextra } from './remark-plugins/translate-to-nextra';

export interface SchemaProperty {
  id: string;
  required: boolean;
  level: number;
  compiledContent: string;
  type: string;
}

export async function flattenSchemaProperties(
  schema: JSONSchema7,
  parentId?: string
) {
  if ((schema as Record<string, any>)['x-skip-properties']) {
    return [];
  }

  let properties: SchemaProperty[] = [];

  const requiredProps = schema.required ?? [];

  if (isType(schema, 'object')) {
    if (schema.properties) {
      await Promise.all(
        Object.entries(schema.properties).map(async ([id, definition]) => {
          if (typeof definition === 'boolean') {
            return;
          }
          const formattedId = formatId(id, parentId);
          const level = getLevel(formattedId);
          const content = [
            `${'#'.repeat(level)} \`${formattedId}\``,
            definition.description,
          ]
            .filter(Boolean)
            .join('\n\n');

          properties.push({
            id: formattedId,
            level,
            required: requiredProps.includes(id),
            compiledContent: (
              await compileMdx(content, {
                mdxOptions: { remarkPlugins: [translateToNextra] },
              })
            ).result,
            type: getType(definition),
          });
          properties.push(
            ...(await flattenSchemaProperties(definition, formattedId))
          );
        })
      );
    }
    if (
      schema.additionalProperties &&
      typeof schema.additionalProperties !== 'boolean'
    ) {
      properties.push(
        ...(await flattenSchemaProperties(
          schema.additionalProperties,
          formatId('<label>', parentId)
        ))
      );
    }
  } else if (isType(schema, 'array')) {
    const items = Array.isArray(schema.items) ? schema.items : [schema.items];
    await Promise.all(
      items.map(async (definition) => {
        if (!definition || typeof definition === 'boolean') {
          return;
        }
        properties.push(
          ...(await flattenSchemaProperties(
            definition,
            formatId('*', parentId)
          ))
        );
      })
    );
  } else if (schema.oneOf) {
    await Promise.all(
      schema.oneOf.map(async (definition) => {
        if (typeof definition === 'boolean') {
          return;
        }
        properties.push(
          ...(await flattenSchemaProperties(definition, parentId))
        );
      })
    );
  } else if (schema.anyOf) {
    await Promise.all(
      schema.anyOf.map(async (definition) => {
        if (typeof definition === 'boolean') {
          return;
        }
        properties.push(
          ...(await flattenSchemaProperties(definition, parentId))
        );
      })
    );
  } else {
    if (
      schema.type !== 'string' &&
      schema.type !== 'integer' &&
      schema.type !== 'boolean' &&
      schema.type !== 'number'
    )
      console.log(schema.type, Object.keys(schema));
  }

  return properties;
}

function isType(schema: JSONSchema7, type: JSONSchema7TypeName) {
  if (!schema.type) {
    return false;
  }
  if (Array.isArray(schema.type)) {
    return schema.type.includes(type);
  } else {
    return schema.type === type;
  }
}

function getType(schema: JSONSchema7): string {
  let type: string | undefined;
  if (Array.isArray(schema.type)) {
    type = schema.type.find((x) => x !== 'null');
  } else {
    type = schema.type;
  }

  if (schema.enum) {
    let values = schema.enum;
    if (type === 'string') {
      values = values.map((value) => `'${value}'`);
    }
    return values.join(' | ');
  }

  if (type === 'number' || type === 'integer') {
    return schema.format ?? type;
  }

  if (
    type === 'array' &&
    schema.items &&
    typeof schema.items !== 'boolean' &&
    !Array.isArray(schema.items)
  ) {
    return `[${getType(schema.items)}]`;
  }

  if (type === 'object' && schema.additionalProperties) {
    return 'dictionary';
  }

  if (schema.oneOf) {
    return schema.oneOf
      .filter(
        (definition): definition is JSONSchema7 =>
          typeof definition !== 'boolean'
      )
      .map(getType)
      .join(' | ');
  }

  if (schema.anyOf) {
    return schema.anyOf
      .filter(
        (definition): definition is JSONSchema7 =>
          typeof definition !== 'boolean'
      )
      .map(getType)
      .join(' | ');
  }

  return type ?? 'unknown';
}

function formatId(id: string, parentId?: string) {
  return (parentId ? `${parentId}.` : '') + id;
}

function getLevel(id: string) {
  // flat after first level
  return id.includes('.') ? 4 : 3;

  // flat after 6th level
  // return Math.min(
  //   6,
  //   Array.from(id).reduce(
  //     (n, c) => (c === '.' ? n + 1 : c === '*' ? n - 1 : n),
  //     3
  //   )
  // );
}
