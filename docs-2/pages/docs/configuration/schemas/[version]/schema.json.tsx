import { GetStaticPaths, GetStaticProps } from 'next';

type Params = {
  version: string;
};

type Props = {
  schema: any;
};

export const getStaticPaths: GetStaticPaths<Params> = async () => {
  return {
    paths: [{ params: { version: 'v0.0.1' } }],
    fallback: false,
  };
};

export const getStaticProps: GetStaticProps<Props, Params> = async ({
  params,
}) => {
  const version = params?.version;
  if (!version) {
    throw new Error('Missing required param: version');
  }

  return {
    props: {
      schema: {
        version,
      },
    },
  };
};

export default function Schema({ schema }: Props) {
  return <pre>{JSON.stringify(schema, null, 2)}</pre>;
}
