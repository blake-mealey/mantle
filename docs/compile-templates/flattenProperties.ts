import { JSONSchema7, JSONSchema7TypeName } from 'json-schema';

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

function getType(schema: JSONSchema7) {
  let type: string;
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
    return schema.format;
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
      .filter((definition) => typeof definition !== 'boolean')
      .map(getType)
      .join(' | ');
  }

  if (schema.anyOf) {
    return schema.anyOf
      .filter((definition) => typeof definition !== 'boolean')
      .map(getType)
      .join(' | ');
  }

  return type;
}

export default function flattenProperties(
  schema: JSONSchema7,
  parentId?: string
) {
  if (schema['x-skip-properties']) {
    return [];
  }

  let properties: {
    id: string;
    required: boolean;
    level: number;
    schema: JSONSchema7;
    type: string;
  }[] = [];

  const requiredProps = schema.required ?? [];

  if (isType(schema, 'object')) {
    if (schema.properties) {
      Object.entries(schema.properties).forEach(([id, definition]) => {
        if (typeof definition === 'boolean') {
          return;
        }
        const formattedId = formatId(id, parentId);
        properties.push({
          id: formattedId,
          level: getLevel(formattedId),
          required: requiredProps.includes(id),
          schema: definition,
          type: getType(definition),
        });
        properties.push(...flattenProperties(definition, formattedId));
      });
    }
    if (
      schema.additionalProperties &&
      typeof schema.additionalProperties !== 'boolean'
    ) {
      properties.push(
        ...flattenProperties(
          schema.additionalProperties,
          formatId('<label>', parentId)
        )
      );
    }
  } else if (isType(schema, 'array')) {
    const items = Array.isArray(schema.items) ? schema.items : [schema.items];
    items.forEach((definition) => {
      if (typeof definition === 'boolean') {
        return;
      }
      properties.push(
        ...flattenProperties(definition, formatId('*', parentId))
      );
    });
  } else if (schema.oneOf) {
    schema.oneOf.forEach((definition) => {
      if (typeof definition === 'boolean') {
        return;
      }
      properties.push(...flattenProperties(definition, parentId));
    });
  } else if (schema.anyOf) {
    schema.anyOf.forEach((definition) => {
      if (typeof definition === 'boolean') {
        return;
      }
      properties.push(...flattenProperties(definition, parentId));
    });
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
