import Link from 'next/link';
import { ReactNode } from 'react';
import { clsx } from 'clsx';

interface FeatureProps {
  children: ReactNode;
  reverse?: boolean;
}

export function Feature({ children, reverse }: FeatureProps) {
  return (
    <div
      className={clsx(
        {
          'flex-row-reverse': reverse,
        },
        'relative overflow-hidden bg-gradient-to-r to-neutral-800/20 border border-neutral-500/20 rounded-xl p-16 flex flex-wrap gap-32 mb-16'
      )}
    >
      {children}
      {reverse ? (
        <>
          <span className="-z-10 absolute w-[1000px] h-[1000px] -bottom-[500px] -right-[500px] rounded-full bg-orange-700" />
          <span className="-z-10 absolute w-[500px] h-[500px] -top-[250px] -right-[250px] rounded-full bg-orange-600" />
          <span className="-z-10 absolute w-[1000px] h-[1000px] -top-[500px] -left-[500px] rounded-full bg-orange-400" />
          <span className="-z-10 absolute w-[500px] h-[500px] -bottom-[250px] -left-[250px] rounded-full bg-orange-500" />
        </>
      ) : (
        <>
          <span className="-z-10 absolute w-[1000px] h-[1000px] -top-[500px] -left-[500px] rounded-full bg-orange-700" />
          <span className="-z-10 absolute w-[500px] h-[500px] -bottom-[250px] -left-[250px] rounded-full bg-orange-600" />
          <span className="-z-10 absolute w-[1000px] h-[1000px] -bottom-[500px] -right-[500px] rounded-full bg-orange-400" />
          <span className="-z-10 absolute w-[500px] h-[500px] -top-[250px] -right-[250px] rounded-full bg-orange-500" />
        </>
      )}
    </div>
  );
}

interface FeatureContentProps {
  children: ReactNode;
}

export function FeatureContent({ children }: FeatureContentProps) {
  return <div className="flex-1 flex flex-col gap-4">{children}</div>;
}

export function FeatureTitle({ children }: FeatureContentProps) {
  return <div className="text-3xl font-bold">{children}</div>;
}

export function FeatureDescription({ children }: FeatureContentProps) {
  return <div className="text-xl font-medium">{children}</div>;
}

interface FeatureActionProps {
  children: ReactNode;
  href: string;
}

export function FeatureAction({ children, href }: FeatureActionProps) {
  return (
    <Link
      href={href}
      className="rounded-lg bg-white text-black font-semibold px-5 py-2 text-xl w-fit block"
    >
      {children}
    </Link>
  );
}

interface FeatureDisplayProps {
  children: ReactNode;
  title: ReactNode;
}

export function FeatureDisplay({ children, title }: FeatureDisplayProps) {
  return (
    <div className="flex-1 rounded-xl border border-neutral-500/20 bg-neutral-900/50 backdrop-blur-3xl">
      <div className="border-b border-neutral-500/20 p-4 font-medium">
        {title}
      </div>
      <div className="py-4">{children}</div>
    </div>
  );
}
