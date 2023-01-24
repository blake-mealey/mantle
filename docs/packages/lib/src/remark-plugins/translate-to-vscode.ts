import { Processor } from '@mdx-js/mdx/lib/core';
import { u } from 'unist-builder';
import { visit } from 'unist-util-visit';
import { Plugin } from 'unified';

const calloutTypeToEmoji: Record<string, string> = {
  default: '💡',
  info: 'ℹ️',
  warning: '⚠️',
  error: '🚫',
};

// Our docs are written for our website, which makes them hard to read in VSCode.
// We run the markdown through a remark plugin to simplify it for VSCode rendering.
export const translateToVscode: Plugin<[]> = function (this: Processor) {
  return (tree, _file, done) => {
    visit(tree, [{ type: 'code' }], (node: any, index, parent: any) => {
      // VSCode does not support code block filenames
      if (node.meta) {
        const match = (node.meta as string).match(/filename="(.*)"/);
        if (match && match[1]) {
          (parent.children as any[]).splice(
            index as number,
            0,
            u('paragraph', [u('strong', [u('text', match[1] + ':')])])
          );
        }
        delete node.meta;
      }

      // VSCode does not support fenced code blocks
      delete node.lang;
    });

    // Replace callouts with blockquotes and emoji
    visit(tree, [{ type: 'paragraph' }], (node: any, index, parent: any) => {
      const firstChild = node.children?.[0];
      if (
        firstChild?.type === 'html' &&
        firstChild.value.startsWith('<Callout')
      ) {
        const calloutType = firstChild.value.match(/type="(\w*)"/)?.[1];

        let firstTextNode = node.children[1];
        if (firstTextNode?.type === 'paragraph') {
          firstTextNode = firstTextNode.children[0];
        }
        if (
          firstTextNode?.type === 'text' &&
          firstTextNode.value &&
          calloutType
        ) {
          firstTextNode.value = [
            calloutTypeToEmoji[calloutType],
            firstTextNode.value,
          ].join(' ');
        }

        const childrenWithoutHtmlTags = node.children.slice(1, -1);
        const blockquote = u('blockquote', [
          u('paragraph', childrenWithoutHtmlTags),
        ]);

        parent.children.splice(index as number, 1, blockquote);
      }
    });

    // Make relative URLs point to the docs
    visit(tree, 'link', (node: any) => {
      const url: string = node.url;
      if (url.startsWith('/')) {
        node.url = 'https://mantledeploy.vercel.app' + url;
      } else if (url.startsWith('#')) {
        node.url =
          'https://mantledeploy.vercel.app/docs/configuration/reference' + url;
      }
    });

    done();
  };
};
