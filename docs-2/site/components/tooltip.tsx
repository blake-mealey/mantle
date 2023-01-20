import { ReactNode, useReducer } from 'react';
import * as RadixPopover from '@radix-ui/react-popover';

type TooltipStatus = 'closed' | 'open' | 'open_persist';
type Action =
  | 'targetEntered'
  | 'targetLeft'
  | 'targetClicked'
  | 'targetFocused'
  | 'close';

const transitions: Record<TooltipStatus, { [K in Action]?: TooltipStatus }> = {
  closed: {
    targetEntered: 'open',
    targetClicked: 'open_persist',
    targetFocused: 'open_persist',
  },
  open: {
    targetLeft: 'closed',
    targetClicked: 'open_persist',
    targetFocused: 'open_persist',
    close: 'closed',
  },
  open_persist: {
    close: 'closed',
    targetClicked: 'closed',
    targetFocused: 'open_persist',
  },
};

function tooltipStatusReducer(
  status: TooltipStatus,
  action: Action
): TooltipStatus {
  const nextStatus = transitions[status]?.[action];
  return nextStatus ?? status;
}

interface TooltipProps {
  children: ReactNode;
  header: ReactNode;
  content: ReactNode;
  defaultOpen?: boolean;
}

export function Tooltip({
  children,
  header,
  content,
  defaultOpen = false,
}: TooltipProps) {
  const [status, dispatch] = useReducer(
    tooltipStatusReducer,
    defaultOpen ? 'open' : 'closed'
  );

  return (
    <RadixPopover.Root open={status !== 'closed'}>
      <RadixPopover.Trigger
        asChild
        onMouseEnter={() => dispatch('targetEntered')}
        onMouseLeave={() => dispatch('targetLeft')}
        onFocus={() => dispatch('targetFocused')}
        onBlur={() => dispatch('close')}
        onMouseDown={() => dispatch('targetClicked')}
        onKeyDown={(e) => {
          if (e.code === 'Space' || e.code === 'Enter') {
            dispatch('targetClicked');
          }
        }}
      >
        <button className="bg-none rounded group cursor-default" type="button">
          {children}
        </button>
      </RadixPopover.Trigger>
      <RadixPopover.Portal>
        <RadixPopover.Content
          sideOffset={8}
          onEscapeKeyDown={() => dispatch('close')}
          onOpenAutoFocus={(e) => e.preventDefault()}
          onCloseAutoFocus={(e) => e.preventDefault()}
        >
          <div className="px-4 py-3 max-w-lg bg-neutral-900/80 backdrop-blur-md rounded border border-1 border-white/10 z-30">
            <header className="font-bold mb-1">{header}</header>
            {content}
          </div>
          <RadixPopover.Arrow
            className="fill-neutral-900/80 stroke-white/10 translate-y-[-1px]"
            strokeWidth="2"
            strokeDashoffset={34}
            strokeDasharray={33}
            strokeLinecap="round"
          />
        </RadixPopover.Content>
      </RadixPopover.Portal>
    </RadixPopover.Root>
  );
}
