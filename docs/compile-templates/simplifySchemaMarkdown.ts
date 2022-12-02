import { JSONSchema7 } from 'json-schema';
import remark, { RemarkOptions } from 'remark';
import visit from 'unist-util-visit';
import remarkGfm from 'remark-gfm';
import { paragraph, text, strong, blockquote } from 'mdast-builder';

const admonitionTypeToEmoji = {
  note: '📝',
  tip: '💡',
  info: 'ℹ️',
  caution: '⚠️',
  danger: '🔥',
};

const simplify = () => {
  return (tree: any, _file: any) => {
    visit(tree, 'paragraph', (node: any, index: number, parent: any) => {
      // VSCode does not support admonitions. Note the "correct" way to do this
      // would be with the `remark-admonitions` package but I could not get it
      // to work (always threw an error immediately after being used)
      if (node.children[0].value.startsWith(':::')) {
        const match = node.children[0].value.match(/^:::(\w*)\s*\n/);
        const type = match[1];

        let lines = node.children[0].value.split('\n');
        lines.splice(
          0,
          1,
          admonitionTypeToEmoji[type] + ' ' + type.toUpperCase(),
          ''
        );
        node.children[0].value = lines.join('\n');

        lines = node.children[node.children.length - 1].value.split('\n');
        lines.splice(-1, 1);
        node.children[node.children.length - 1].value = lines.join('\n');

        parent.children[index] = blockquote([node]);
      }
    });

    visit(tree, 'code', (node: any, index: number, parent: any) => {
      // VSCode does not support code block titles
      if (node.meta) {
        const match = (node.meta as string).match(/title="(.*)"/);
        if (match && match[1]) {
          (parent.children as any[]).splice(
            index,
            0,
            paragraph([strong(text(match[1] + ':'))])
          );
        }
        delete node.meta;
      }

      // VSCode does not support fenced code blocks
      delete node.lang;
    });

    visit(tree, 'link', (node: any) => {
      // Make relative URLs point to the docs
      const url: string = node.url;
      if (url.startsWith('/')) {
        node.url = 'https://mantledeploy.vercel.app' + url;
      } else if (url.startsWith('#')) {
        node.url = 'https://mantledeploy.vercel.app/docs/configuration' + url;
      }
    });
  };
};

const processor = remark()
  .data('settings', {} as RemarkOptions)
  .use(remarkGfm)
  .use(simplify);

async function simplifyMarkdown(md: string) {
  const { contents } = await processor.process(md);
  return contents.toString();
}

export async function simplifySchemaMarkdown(
  schema: JSONSchema7,
  title?: string
) {
  schema.title = title;

  if (schema.description) {
    // VSCode interprets the `description` property as plaintext. To render as
    // markdown, you need to use the `markdownDescription` property.
    (schema as any).markdownDescription = await simplifyMarkdown(
      schema.description
    );
    delete schema.description;
  }

  if (schema.properties) {
    for (const [name, definition] of Object.entries(schema.properties)) {
      if (typeof definition !== 'boolean') {
        await simplifySchemaMarkdown(definition, name);
      }
    }
  }

  if (schema.oneOf) {
    for (const definition of Object.values(schema.oneOf)) {
      if (typeof definition !== 'boolean') {
        await simplifySchemaMarkdown(definition);
      }
    }
  }

  if (schema.anyOf) {
    for (const definition of Object.values(schema.anyOf)) {
      if (typeof definition !== 'boolean') {
        await simplifySchemaMarkdown(definition);
      }
    }
  }

  if (typeof schema.items === 'object') {
    const items = Array.isArray(schema.items) ? schema.items : [schema.items];
    for (const definition of items) {
      if (typeof definition !== 'boolean') {
        await simplifySchemaMarkdown(definition);
      }
    }
  }

  if (typeof schema.additionalProperties === 'object') {
    await simplifySchemaMarkdown(schema.additionalProperties);
  }
}
