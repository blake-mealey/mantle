import { JSONSchema7, JSONSchema7TypeName, JSONSchema7Type } from 'json-schema';
import { compileMdx } from 'nextra/compile';
import { isDefined } from './is-defined';
import { translateToNextra } from './remark-plugins/translate-to-nextra';

export interface SchemaProperty {
  id: string;
  required: boolean;
  level: number;
  compiledContent: string | null;
  type: string;
  propertyType: PropertyType;
  default: { value: JSONSchema7Type } | null;
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
      for (const [id, definition] of Object.entries(schema.properties)) {
        if (typeof definition === 'boolean') {
          continue;
        }
        const formattedId = formatId(id, parentId);

        properties.push({
          id: formattedId,
          level: getLevel(formattedId),
          required: requiredProps.includes(id),
          compiledContent: definition.description
            ? (
                await compileMdx(definition.description, {
                  mdxOptions: { remarkPlugins: [translateToNextra] },
                })
              ).result
            : null,
          type: getType(definition),
          propertyType: getSchemaPropertyType(definition),
          default:
            definition.default === undefined
              ? null
              : { value: definition.default },
        });
        properties.push(
          ...(await flattenSchemaProperties(definition, formattedId))
        );
      }
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
    for (const definition of items) {
      if (!definition || typeof definition === 'boolean') {
        continue;
      }
      properties.push(
        ...(await flattenSchemaProperties(definition, formatId('*', parentId)))
      );
    }
  } else if (schema.oneOf) {
    for (const definition of schema.oneOf) {
      if (typeof definition === 'boolean') {
        continue;
      }
      properties.push(...(await flattenSchemaProperties(definition, parentId)));
    }
  } else if (schema.anyOf) {
    for (const definition of schema.anyOf) {
      if (typeof definition === 'boolean') {
        continue;
      }
      properties.push(...(await flattenSchemaProperties(definition, parentId)));
    }
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

export type PropertyType =
  | { type: 'primitive'; value: string }
  | { type: 'enum'; values: PropertyType[] }
  | { type: 'array'; valueType: PropertyType }
  | { type: 'dictionary'; valueType: PropertyType }
  | { type: 'object'; properties: [string, PropertyType][] }
  | { type: 'oneOf'; values: PropertyType[] }
  | { type: 'anyOf'; values: PropertyType[] };

function getValuePropertyType(type: JSONSchema7Type): PropertyType {
  if (typeof type === 'object') {
    if (Array.isArray(type)) {
      const item = type[0];
      return {
        type: 'array',
        valueType: item
          ? getValuePropertyType(item)
          : { type: 'primitive', value: 'unknown' },
      };
    }
    // TODO: include object properties?
    return {
      type: 'primitive',
      value: 'object',
    };
  }
  return {
    type: 'primitive',
    value: type.toString(),
  };
}

function getSchemaPropertyType(schema: JSONSchema7): PropertyType {
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
    return {
      type: 'enum',
      values: values.map(getValuePropertyType),
    };
  }

  if (type === 'number' || type === 'integer') {
    return {
      type: 'primitive',
      value: schema.format ?? type,
    };
  }

  if (
    type === 'array' &&
    schema.items &&
    typeof schema.items !== 'boolean' &&
    !Array.isArray(schema.items)
  ) {
    return {
      type: 'array',
      valueType: getSchemaPropertyType(schema.items),
    };
  }

  if (type === 'object') {
    if (schema.additionalProperties) {
      return {
        type: 'dictionary',
        valueType:
          typeof schema.additionalProperties !== 'boolean'
            ? getSchemaPropertyType(schema.additionalProperties)
            : { type: 'primitive', value: 'unknown' },
      };
    }
    return {
      type: 'object',
      properties: Object.entries(schema.properties ?? {})
        .map(([name, property]): [string, PropertyType] | undefined =>
          typeof property !== 'boolean'
            ? [name, getSchemaPropertyType(property)]
            : undefined
        )
        .filter(isDefined),
    };
  }

  if (schema.oneOf) {
    const oneOf = {
      type: 'oneOf',
      values: schema.oneOf
        .filter(
          (definition): definition is JSONSchema7 =>
            typeof definition !== 'boolean'
        )
        .map(getSchemaPropertyType),
    } as const;
    if (oneOf.values.length === 1) {
      return oneOf.values[0]!;
    }
    return oneOf;
  }

  if (schema.anyOf) {
    const anyOf = {
      type: 'anyOf',
      values: schema.anyOf
        .filter(
          (definition): definition is JSONSchema7 =>
            typeof definition !== 'boolean'
        )
        .map(getSchemaPropertyType),
    } as const;
    if (anyOf.values.length === 1) {
      return anyOf.values[0]!;
    }
    return anyOf;
  }

  return {
    type: 'primitive',
    value: type ?? 'unknown',
  };
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
    return `enum(${values.join(', ')})`;
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
    return `${getType(schema.items)}[]`;
  }

  if (type === 'object' && schema.additionalProperties) {
    return 'dictionary';
  }

  if (schema.oneOf) {
    return `oneOf(${schema.oneOf
      .filter(
        (definition): definition is JSONSchema7 =>
          typeof definition !== 'boolean'
      )
      .map(getType)
      .join(', ')})`;
  }

  if (schema.anyOf) {
    return `anyOf(${schema.anyOf
      .filter(
        (definition): definition is JSONSchema7 =>
          typeof definition !== 'boolean'
      )
      .map(getType)
      .join(', ')})`;
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
