import * as React from "react";

import { cn } from "@/lib/utils";

/*
 * Per docs/design.md §6.2 (v2): 36 px tall, 6 px radius, surface-alt bg.
 * Focus: border becomes accent + 3 px outer ring (handled by global
 * :focus-visible style).
 */
const Input = React.forwardRef<
  HTMLInputElement,
  React.InputHTMLAttributes<HTMLInputElement>
>(({ className, type, ...props }, ref) => {
  return (
    <input
      type={type}
      className={cn(
        "flex h-9 w-full rounded-md border border-border-default bg-surface-alt px-3 text-sm font-sans text-text-strong transition-colors duration-instant ease-enter file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-text placeholder:text-muted-foreground focus:border-accent focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      ref={ref}
      {...props}
    />
  );
});
Input.displayName = "Input";

export { Input };
