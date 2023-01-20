import { Code, Pre } from 'nextra/components';
import { useSSG } from 'nextra/data';
import { useMDXComponents } from 'nextra/mdx';

export function SchemasList() {
  const { schemaVersions } = useSSG() as { schemaVersions: string[] };

  const { ul: Ul, li: Li, a: A } = useMDXComponents();

  return (
    <>
      <Pre lang="json">
        <Code lang="json">
          {JSON.stringify(
            { 'yaml.schemas': { 'https://asdf': 'mantle.yml' } },
            null,
            2
          )}
        </Code>
      </Pre>
      {/* @ts-ignore */}
      <Ul>
        {schemaVersions.map((version) => (
          // @ts-ignore
          <Li>
            {/* @ts-ignore */}
            <A href={`/schemas/${version}/schema.json`}>{version}</A>
          </Li>
        ))}
      </Ul>
    </>
  );
}
