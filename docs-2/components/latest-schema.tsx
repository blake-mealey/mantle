import { Fragment } from 'react';
import { Schema } from '@lib/schemas';
import { useSSG } from 'nextra/ssg';
import { Code } from 'nextra/components';
import { useMDXComponents } from 'nextra-theme-docs';
import { MDXRemote } from 'next-mdx-remote';

function Heading({
  level,
  children,
}: {
  level: number;
  children: React.ReactNode;
}) {
  const components = useMDXComponents();
  const tag = `h${level}` as keyof JSX.IntrinsicElements;
  const Tag = components[tag] ?? tag;
  return <Tag id="">{children}</Tag>;
}

export function LatestSchema() {
  const { schema } = useSSG() as { schema: Schema };
  const { properties } = schema;
  const components = useMDXComponents();

  return properties.map((property) => {
    return (
      <MDXRemote
        key={property.id}
        compiledSource={property.compiledContent}
        components={components}
      />
    );
  });
}

// {{#each properties}}
// {{#repeat this.level}}#{{/repeat}} `{{this.id}}`{{> heading-info }}{{anchor this.id}}

// {{this.schema.description}}

// {{/each}}
