import { ReactNode } from 'react';
import * as RadixTooltip from '@radix-ui/react-tooltip';

interface TooltipProps {
  children: ReactNode;
  header: ReactNode;
  content: ReactNode;
  open?: boolean;
}

export function Tooltip({ children, header, content, open }: TooltipProps) {
  return (
    <RadixTooltip.Provider>
      <RadixTooltip.Root delayDuration={0} open={open}>
        <RadixTooltip.Trigger asChild>
          <button
            className="bg-none rounded group cursor-default"
            type="button"
          >
            {children}
          </button>
        </RadixTooltip.Trigger>
        <RadixTooltip.Portal>
          <RadixTooltip.Content asChild sideOffset={8}>
            <div className="px-4 py-3 max-w-lg bg-neutral-900 border rounded border-neutral-300 dark:border-neutral-600">
              <header className="font-bold mb-1">{header}</header>
              {content}
            </div>
          </RadixTooltip.Content>
        </RadixTooltip.Portal>
      </RadixTooltip.Root>
    </RadixTooltip.Provider>
  );
}
