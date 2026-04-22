import * as React from "react";

import { cn } from "@/lib/utils";

/*
 * Per docs/design.md §6.2: 32 px tall, 1 px border --paper-300, 4 px radius,
 * --paper-200 background. Focus is the global :focus-visible outline ring
 * defined in src/index.css plus a border swap to --ink-500.
 */
const Input = React.forwardRef<
  HTMLInputElement,
  React.InputHTMLAttributes<HTMLInputElement>
>(({ className, type, ...props }, ref) => {
  return (
    <input
      type={type}
      className={cn(
        "flex h-8 w-full rounded border border-paper-300 bg-paper-200 px-3 py-1 text-sm font-sans text-paper-900 transition-colors duration-instant ease-enter file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-paper-700 placeholder:text-paper-500 focus:border-ink-500 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      ref={ref}
      {...props}
    />
  );
});
Input.displayName = "Input";

export { Input };
