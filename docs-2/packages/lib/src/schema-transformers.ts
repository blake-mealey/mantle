import { remark } from 'remark';
import remarkGfm from 'remark-gfm';
import remarkMdx from 'remark-mdx';
import { translateToNextra } from './remark-plugins/translate-to-nextra';
import { translateToVscode } from './remark-plugins/translate-to-vscode';
import { createSchemaTransformer } from './transform-schema-markdown';

export function getTranslateToNextraTransformer() {
  const processor = remark()
    .use(remarkGfm)
    .use(translateToNextra)
    .use(remarkMdx);
  return createSchemaTransformer(processor);
}

export function getTranslateToVscodeTransformer() {
  const processor = remark()
    .use(remarkGfm)
    .use(remarkMdx)
    .use(translateToVscode);
  return createSchemaTransformer(processor, {
    descriptionKey: 'markdownDescription',
    propagateTitle: true,
  });
}
