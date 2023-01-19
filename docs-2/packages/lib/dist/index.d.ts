import { JSONSchema7Type, JSONSchema7 } from 'json-schema';
import { Plugin } from 'unified';

interface SchemaProperty {
    id: string;
    required: boolean;
    level: number;
    compiledContent: string | null;
    propertyType: PropertyType;
    default: {
        value: JSONSchema7Type;
    } | null;
}
declare function flattenSchemaProperties(compileMdx: CompileMdx, schema: JSONSchema7, parentId?: string): Promise<SchemaProperty[]>;
type PropertyType = {
    type: 'primitive';
    value: string;
} | {
    type: 'enum';
    values: PropertyType[];
} | {
    type: 'array';
    valueType: PropertyType;
} | {
    type: 'dictionary';
    valueType: PropertyType;
} | {
    type: 'object';
    properties: [string, PropertyType][];
} | {
    type: 'oneOf';
    values: PropertyType[];
} | {
    type: 'anyOf';
    values: PropertyType[];
};

interface Schemas {
    schemas: Record<string, JSONSchema7>;
    latestVersion: string | undefined;
}
declare function getSchemas(): Promise<Schemas>;
interface Schema {
    version: string;
    properties: SchemaProperty[];
}
type CompileMdx = (md: string, options: {
    mdxOptions: {
        remarkPlugins: Plugin[];
    };
}) => Promise<{
    result: string;
}>;
declare function processSchema(compileMdx: CompileMdx, { version, schema, }: {
    version: string;
    schema: JSONSchema7;
}): Promise<Schema>;

declare function isDefined<T>(value: T | undefined | null): value is T;

declare const translateToNextra: Plugin<[]>;

export { CompileMdx, PropertyType, Schema, SchemaProperty, flattenSchemaProperties, getSchemas, isDefined, processSchema, translateToNextra };
