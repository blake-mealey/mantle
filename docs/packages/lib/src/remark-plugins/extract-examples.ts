import { Processor } from '@mdx-js/mdx/lib/core';
import { SKIP, visit } from 'unist-util-visit';
import { Plugin } from 'unified';

// We render examples in the docs in an aside, so we need to extract the example code blocks from the markdown
// and render them separately. We look for all code blocks with a filename that includes "Example" and extract
// them using vfile data.
export const extractExamples: Plugin<[]> = function (this: Processor) {
  return (tree, file, done) => {
    visit(tree, [{ type: 'code' }], (node: any, index, parent: any) => {
      if (node.meta) {
        const match = (node.meta as string).match(/filename="(.*)"/);

        if (match && match[1]?.includes('Example')) {
          let examples = file.data.examples;
          if (!examples || !Array.isArray(examples)) {
            examples = [];
            file.data.examples = examples;
          }
          // @ts-ignore
          examples.push(node);
          parent.children.splice(index, 1);
          return [SKIP, index];
        }
      }
    });

    done();
  };
};
