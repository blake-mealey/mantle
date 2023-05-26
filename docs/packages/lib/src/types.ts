import { Plugin } from 'unified';

export type CompileMdx = (
  md: string,
  options?: { mdxOptions: { remarkPlugins: Plugin[] } }
) => Promise<{ result: string }>;
