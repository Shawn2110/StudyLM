import * as React from "react";

import { cn } from "@/lib/utils";

/*
 * SMALL CAPS label component per docs/design.md §3.2 (eyebrow type style).
 * Used for panel titles ("SOURCES", "CHAT", "STUDY GUIDE"), prep-mode
 * metadata, and any label that should sit quietly above content.
 *
 * Geist 600 / 0.6875 rem / tracking 0.12em / paper-500. Never more than one
 * eyebrow per visual group.
 */

type EyebrowProps = React.HTMLAttributes<HTMLSpanElement> & {
  as?: keyof JSX.IntrinsicElements;
};

export const Eyebrow = React.forwardRef<HTMLSpanElement, EyebrowProps>(
  ({ className, as: Comp = "span", children, ...props }, ref) => {
    const Tag = Comp as React.ElementType;
    return (
      <Tag
        ref={ref}
        className={cn(
          "inline-block text-[0.6875rem] font-sans font-semibold uppercase leading-tight tracking-[0.12em] text-paper-500",
          className,
        )}
        {...props}
      >
        {children}
      </Tag>
    );
  },
);
Eyebrow.displayName = "Eyebrow";
