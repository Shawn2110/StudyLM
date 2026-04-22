import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

/*
 * Per docs/design.md §6.1: rectangular (not pill), 4 px radius, 32 px tall,
 * UI font weight 500. Pills are reserved for semantic signals (status,
 * prep mode, citations).
 */
const buttonVariants = cva(
  "inline-flex h-8 items-center justify-center gap-2 whitespace-nowrap rounded text-sm font-medium font-sans transition-colors duration-instant ease-enter focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-[18px] [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        primary:
          "bg-ink-500 text-paper-50 hover:bg-ink-600 active:bg-ink-700",
        secondary:
          "bg-paper-200 text-paper-900 border border-paper-300 hover:bg-paper-300",
        ghost:
          "bg-transparent text-paper-700 hover:bg-paper-100",
        destructive:
          "bg-paper-200 text-danger border border-paper-300 hover:bg-paper-300",
        link:
          "text-ink-500 underline-offset-4 hover:underline hover:text-ink-600",
      },
      size: {
        default: "px-3.5 py-2",
        sm: "h-7 px-3 text-xs",
        lg: "h-10 px-5",
        icon: "h-8 w-8 p-0",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "default",
    },
  },
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  },
);
Button.displayName = "Button";

export { Button, buttonVariants };
