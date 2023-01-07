import { Processor } from '@mdx-js/mdx/lib/core';
import { visit } from 'unist-util-visit';
import { Plugin } from 'unified';

interface CodeNode {
  lang?: string;
  meta?: string;
}

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

    done();
  };
};
