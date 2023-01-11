import { Fragment, ReactNode, useMemo } from 'react';
import { Schema } from '@lib/schemas';
import { useSSG } from 'nextra/ssg';
import { Code } from 'nextra/components';
import { useMDXComponents } from 'nextra-theme-docs';
import { MDXRemote } from 'next-mdx-remote';
import clsx from 'clsx';
import { JSONSchema7Type } from 'json-schema';
import { Tooltip } from './tooltip';
import { PropertyType } from '@lib/flatten-schema-properties';

function Heading({
  level,
  ...props
}: {
  level: number;
  children: React.ReactNode;
  id: string;
  style?: React.CSSProperties;
}) {
  const components = useMDXComponents();
  const tag = `h${level}` as keyof JSX.IntrinsicElements;
  const Tag = components[tag] ?? tag;
  return <Tag {...props} />;
}

// TODO: improve for nested properties
// note that changes will probably break links
function slugify(value: string) {
  return value
    .toLowerCase()
    .replace(/\s+/g, '-')
    .replace(/[^\w-]/g, '');
}

function TooltipToken({
  children,
  type,
}: {
  children: ReactNode;
  type: MetaTokenProps['type'];
}) {
  return (
    <MetaToken
      type={type}
      className="border-b border-dotted border-gray-500 group-hover:border-yellow-700 group-focus:border-yellow-700 group-hover:text-yellow-500 group-focus:text-yellow-500 transition-colors"
    >
      {children}
    </MetaToken>
  );
}

interface MetaTokenProps {
  type: 'optional' | 'required' | 'default' | 'type';
  children: ReactNode;
  className?: string;
}

function MetaToken({ type, className, children }: MetaTokenProps) {
  return (
    <span
      className={clsx(className, 'tracking-tighter font-medium', {
        'nx-text-gray-500': type === 'optional',
        'text-red-500': type === 'required',
        'text-cyan-500': type === 'default',
      })}
    >
      {children}
    </span>
  );
}

function PropertyTypeToken({
  propertyType,
  root = false,
  nested = false,
}: {
  propertyType: PropertyType;
  root?: boolean;
  nested?: boolean;
}) {
  if (propertyType.type === 'primitive') {
    if (root) {
      return <MetaToken type="type">{propertyType.value}</MetaToken>;
    } else {
      if (!nested) {
        return <Code className="whitespace-nowrap">{propertyType.value}</Code>;
      } else {
        return <span>{propertyType.value}</span>;
      }
    }
  }

  if (propertyType.type === 'array') {
    if (root) {
      return (
        <MetaToken type="type">
          array{'<'}
          <PropertyTypeToken propertyType={propertyType.valueType} root />
          {'>'}
        </MetaToken>
      );
    } else {
      return (
        <Code>
          array{'<'}
          <PropertyTypeToken propertyType={propertyType.valueType} nested />
          {'>'}
        </Code>
      );
    }
  }

  if (propertyType.type === 'dictionary') {
    if (root) {
      return (
        <MetaToken type="type">
          dictionary{'<'}
          <PropertyTypeToken propertyType={propertyType.valueType} root />
          {'>'}
        </MetaToken>
      );
    } else {
      return (
        <Code>
          dictionary{'<'}
          <PropertyTypeToken propertyType={propertyType.valueType} nested />
          {'>'}
        </Code>
      );
    }
  }

  if (propertyType.type === 'object') {
    if (root) {
      return (
        <Tooltip
          header="Object"
          content={
            <div className="grid grid-cols-[auto_auto] gap-x-4 gap-y-1">
              {propertyType.properties.map(([name, subPropertyType]) => (
                <Fragment key={name}>
                  <MetaToken type="type">{name}</MetaToken>
                  <span className="w-fit">
                    <PropertyTypeToken propertyType={subPropertyType} />
                  </span>
                </Fragment>
              ))}
            </div>
          }
        >
          <TooltipToken type="type">object</TooltipToken>
        </Tooltip>
      );
    }
    return (
      <PropertyTypeToken
        propertyType={{ type: 'primitive', value: 'object' }}
        nested={nested}
      />
    );
  }

  if (root) {
    return (
      <Tooltip
        header={propertyType.type === 'enum' ? 'Enum' : 'Union'}
        content={propertyType.values.map((item, i) => (
          <Fragment key={i}>
            <PropertyTypeToken key={i} propertyType={item} />
            {i !== propertyType.values.length - 1 && <span>, </span>}
          </Fragment>
        ))}
      >
        <TooltipToken type="type">
          {propertyType.type === 'enum' ? 'enum' : 'union'}
        </TooltipToken>
      </Tooltip>
    );
  } else {
    return (
      <MetaToken type="type">
        {propertyType.values.map((item, i) => (
          <Fragment key={i}>
            <PropertyTypeToken propertyType={item} nested={nested} />
            {i !== propertyType.values.length - 1 && <span>, </span>}
          </Fragment>
        ))}
      </MetaToken>
    );
  }
}

function OptionalToken({
  defaultValue,
}: {
  defaultValue: JSONSchema7Type | undefined;
}) {
  if (!defaultValue) {
    return <MetaToken type="optional">optional</MetaToken>;
  }

  let value: string;
  if (typeof defaultValue === 'object') {
    value = Array.isArray(defaultValue) ? 'array' : 'object';
  }
  value = defaultValue.toString();

  return (
    <Tooltip header="Default" content={<Code>{value}</Code>}>
      <TooltipToken type="optional">optional</TooltipToken>
    </Tooltip>
  );
}

export function LatestSchema() {
  const { schema } = useSSG() as { schema: Schema };
  const { properties } = schema;
  const components = useMDXComponents();

  return properties.map((property) => {
    return (
      <Fragment key={property.id}>
        <div className="flex items-start justify-between flex-col md:flex-row md:items-center">
          <Heading
            level={property.level}
            id={slugify(property.id)}
            style={{ maxWidth: '100%' }}
          >
            <Code>{property.id}</Code>
          </Heading>
          <div className="flex gap-2 mt-1 md:mt-8">
            {property.required ? (
              <MetaToken type="required">required</MetaToken>
            ) : (
              <OptionalToken defaultValue={property.default?.value} />
            )}
            <PropertyTypeToken propertyType={property.propertyType} root />
          </div>
        </div>
        {property.compiledContent && (
          <MDXRemote
            key={property.id}
            compiledSource={property.compiledContent}
            components={components}
          />
        )}
      </Fragment>
    );
  });
}
