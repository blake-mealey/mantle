import { Processor } from '@mdx-js/mdx/lib/core';
import { u } from 'unist-builder';
import { visit } from 'unist-util-visit';
import { Plugin } from 'unified';

interface CodeNode {
  lang?: string;
  meta?: string;
}

interface ParagraphNode {
  children: ParagraphChildNode[];
}

interface ParagraphChildNode {
  value: string;
}

const admonitionTypeToCalloutType: Record<string, string> = {
  note: 'default',
  tip: 'default',
  info: 'info',
  caution: 'warning',
  danger: 'error',
};

// Our docs were originally written for Docusaurus, which has different
// syntax for some features. We run the markdown through a remark plugin
// to translate to Nextra's syntax.
export const translateToNextra: Plugin<[]> = function (this: Processor) {
  return (tree, _file, done) => {
    visit(tree, [{ type: 'code' }], (node) => {
      const codeNode = node as CodeNode;

      // Shiki currently only supports the "yaml" file extension for YAML
      // See PR to fix this here: https://github.com/shikijs/shiki/pull/399
      if (codeNode.lang === 'yml') {
        codeNode.lang = 'yaml';
      }

      // Docusaurus uses the `title` attribute to specify the filename
      if (codeNode.meta) {
        codeNode.meta = codeNode.meta.replace(/title="(.*)"/, 'filename="$1"');
      }
    });

    // Docusaurus uses `:::type` admonitions syntax, while Nextra uses the
    // `<Callout type="">` component.
    visit(tree, [{ type: 'paragraph' }], (node, index, parent: any) => {
      const paragraphNode = node as unknown as ParagraphNode;
      const firstChild = paragraphNode.children[0] as
        | ParagraphChildNode
        | undefined;

      if (firstChild?.value.startsWith(':::')) {
        const match = firstChild.value.match(/^:::(\w*)\s*\n/);
        const type = match?.[1];

        firstChild.value = firstChild?.value.replace(/^:::.*\n/, '');

        const lastChild =
          paragraphNode.children[paragraphNode.children.length - 1];
        lastChild!.value = lastChild!.value.split('\n').slice(0, -1).join('\n');

        const calloutType =
          admonitionTypeToCalloutType[type ?? 'note'] ?? 'default';
        const callout = u('mdxJsxFlowElement', {
          name: 'Callout',
          attributes: [
            { type: 'mdxJsxAttribute', name: 'type', value: calloutType },
          ],
          children: paragraphNode.children,
          data: { _mdxExplicitJsx: true },
        });

        parent.children.splice(index as number, 1, callout);
      }
    });

    done();
  };
};
