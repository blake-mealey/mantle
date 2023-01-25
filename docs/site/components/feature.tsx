import Link from 'next/link';
import React, { ReactNode } from 'react';
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
          'xl:flex-row-reverse': reverse,
        },
        'relative overflow-hidden border border-neutral-500/20 rounded-xl p-6 sm:p-16 flex flex-col xl:flex-row gap-8 xl:gap-32 mb-16'
      )}
    >
      {children}
      {reverse ? (
        <>
          <span className="-z-10 absolute w-[1000px] h-[1000px] -top-[500px] -left-[500px] rounded-full bg-orange-600" />
          <span className="-z-10 absolute w-[500px] h-[500px] -bottom-[250px] -left-[250px] rounded-full bg-orange-700" />
        </>
      ) : (
        <>
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
  return (
    <div className="flex-1 flex flex-col justify-center gap-2 xl:gap-4 text-black dark:text-white">
      {children}
    </div>
  );
}

export function FeatureTitle({ children }: FeatureContentProps) {
  return <div className="text-3xl sm:text-5xl font-bold">{children}</div>;
}

export function FeatureDescription({ children }: FeatureContentProps) {
  return <div className="text-lg sm:text-2xl font-medium">{children}</div>;
}

interface FeatureActionProps {
  children: ReactNode;
  href: string;
}

export function FeatureAction({ children, href }: FeatureActionProps) {
  return (
    <Link
      href={href}
      className="font-semibold text-md text-black/70 dark:text-white/70 w-fit block hover:text-black/80 focus:text-black/90 dark:hover:text-white/80 dark:focus:text-white/90 transition-colors"
    >
      {children} →
    </Link>
  );
}

interface FeatureDisplayProps {
  children: ReactNode;
  title: ReactNode;
}

export function FeatureDisplay({ children, title }: FeatureDisplayProps) {
  const mappedChildren = React.Children.map(children, (child) => {
    return React.isValidElement(child)
      ? React.cloneElement(child, { className: 'py-4 overflow-x-auto' } as any)
      : null;
  });

  return (
    <div className="flex-1 rounded-xl border border-neutral-500/20 bg-neutral-100/80 dark:bg-neutral-900/80 backdrop-blur-3xl">
      <div className="border-b border-neutral-500/20 p-4 font-medium dark:text-white">
        {title}
      </div>
      {mappedChildren}
    </div>
  );
}
