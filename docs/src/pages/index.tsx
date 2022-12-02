import React, { useEffect, useRef } from 'react';
import clsx from 'clsx';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import styles from './index.module.css';
import CodeBlock from '@theme/CodeBlock';
import ThemedImage from '@theme/ThemedImage';

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <ThemedImage
          alt="Mantle Logo"
          sources={{
            light: '/img/mantle-logo-white.svg',
            dark: '/img/mantle-logo-black.svg',
          }}
        />
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/getting-started"
          >
            Get Started
          </Link>
        </div>
      </div>
    </header>
  );
}

const features = [
  {
    label: 'Declarative',
    title: 'Declarative configuration',
    description:
      'Describe the Roblox infrastructure you want and Mantle will take care of the rest.',
    learnMoreLink: '/docs/configuration',
    example: {
      title: 'mantle.yml',
      language: 'yml',
      content: `environments:
  - label: dev
    targetNamePrefix: environmentLabel
  - label: prod
    targetAccess: public

target:
  experience:
    configuration:
      genre: building
    places:
      start:
        file: game.rbxlx
        configuration:
          name: Getting Started with Mantle
          description: |-
            Made with Mantle`,
    },
  },
  {
    label: 'Smart',
    title: 'Smart deployments',
    description:
      'Mantle makes the minimum required changes to keep your deployments fast and stable.',
    learnMoreLink: '/docs/getting-started',
    example: {
      title: 'mantle deploy',
      language: 'ansi',
      content: `Deploying resources:
  ╷
  │  [33m~[0m Updating: placeConfiguration_start
  │    ╷
  │    │  Dependencies:
  │    │      [2m- place:[0m
  │    │      [2m    assetId: 8635420754[0m
  │    │  Inputs:
  │    │      [2mplaceConfiguration:[0m
  │    │    [31m-[0m [31m  name: Getting Started with Mantle[0m
  │    │    [32m+[0m [32m  name: Going to the moon with Mantle[0m
  │    │      [2m  description: Made with Mantle[0m
  │    │      [2m  maxPlayerCount: 50[0m
  │    │      [2m  allowCopying: false[0m
  │    │      [2m  socialSlotType: Automatic[0m
  │    │      [2m  customSocialSlotsCount: ~[0m
  │    │  
  │    ╰─ Succeeded with outputs:
  │           [2mplaceConfiguration[0m
  │  
  │  
  ╰─ Succeeded with 0 create(s), 1 update(s), 0 delete(s), 5 noop(s), 0 skip(s)`,
    },
  },
  {
    label: 'Integrated',
    title: 'Integrated with Roblox tooling',
    description: (
      <>
        Mantle integrates with the best of the Roblox OSS community. Install
        with <a href="https://github.com/roblox/foreman">Foreman</a> and build
        with <a href="https://rojo.space">Rojo</a>.
      </>
    ),
    learnMoreLink: '/docs/installation',
    example: {
      title: 'foreman.toml',
      language: 'toml',
      content: `[tools]
mantle = { source = "blake-mealey/mantle", version = "0.11" }`,
    },
  },
  {
    label: 'Automated',
    title: 'Automate with continuous deployments',
    description:
      'Mantle is designed for continuous deployment environments like GitHub Actions.',
    learnMoreLink: '/docs/continuous-deployment',
    example: {
      title: '.github/workflows/deploy.yml',
      language: 'yml',
      content: `name: Deploy
on: push
jobs:
  build-and-deploy:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: Roblox/setup-foreman@v1
        with:
          token: $\{\{ secrets.GITHUB_TOKEN }}
      - name: Build project
        run: rojo build --output game.rbxlx
      - name: Deploy project
        run: mantle deploy
        env:
          ROBLOSECURITY: $\{\{ secrets.ROBLOSECURITY }}
          AWS_ACCESS_KEY_ID: $\{\{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: $\{\{ secrets.AWS_SECRET_ACCESS_KEY }}`,
    },
  },
];

function Demo() {
  const ref = useRef<HTMLDivElement>();

  useEffect(() => {
    const { current: parent } = ref;
    if (!parent) {
      return;
    }

    const script = document.createElement('script');
    script.src = 'https://asciinema.org/a/A2RTzQTSLem3npwpcyGn35DPy.js';
    script.id = 'asciicast-A2RTzQTSLem3npwpcyGn35DPy';
    script.async = true;
    script.dataset.autoplay = '1';

    parent.appendChild(script);
    return () => {
      parent.removeChild(script);
    };
  }, [ref]);

  return <div ref={ref} className={styles.demo}></div>;
}

export default function Home(): JSX.Element {
  return (
    <Layout title="Features" description="Mantle features">
      <HomepageHeader />
      <main className={styles.main}>
        <section className={styles.divider}>
          <div>
            {features.map((feature, index) => (
              <a
                key={feature.label}
                href={`#feature-${feature.label.toLowerCase()}`}
              >
                <h3>
                  <span className={styles.featureNum}>{index + 1}</span>
                  {feature.label}
                </h3>
              </a>
            ))}
          </div>
        </section>

        <Demo />

        {features.map((feature, index) => (
          <section
            key={feature.label}
            id={`feature-${feature.label.toLowerCase()}`}
            className={styles.feature}
          >
            <aside>
              <CodeBlock
                className={`language-${feature.example.language}`}
                title={feature.example.title}
              >
                {feature.example.content}
              </CodeBlock>
            </aside>
            <div className={styles.content}>
              <h2>
                <span className={styles.featureNum}>{index + 1}</span>
                {feature.title}
              </h2>
              <p>{feature.description}</p>
              <div>
                <Link
                  className="button button--secondary button--lg"
                  to={feature.learnMoreLink}
                >
                  Learn More
                </Link>
              </div>
            </div>
          </section>
        ))}
      </main>
    </Layout>
  );
}
