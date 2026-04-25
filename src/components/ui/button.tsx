import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

/*
 * Per docs/design.md §6.1 (v2): 36 px tall, 6 px radius, Geist 500.
 * Three variants — primary (coral), secondary (surface-alt), ghost.
 * Pills are reserved for chips/badges and the active sidebar item.
 */
const buttonVariants = cva(
  "inline-flex h-9 items-center justify-center gap-2 whitespace-nowrap rounded-md px-4 text-sm font-medium font-sans transition-all duration-instant ease-enter focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-[18px] [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        primary:
          "bg-accent text-accent-on shadow-sm hover:bg-accent-hover active:bg-accent-active",
        secondary:
          "bg-surface-alt text-text border border-border-default hover:border-border-strong",
        ghost:
          "bg-transparent text-muted-foreground hover:bg-surface-alt hover:text-text",
        destructive:
          "bg-surface-alt text-danger border border-border-default hover:border-border-strong",
        link:
          "text-accent underline-offset-4 hover:underline hover:text-accent-hover",
      },
      size: {
        default: "h-9 px-4",
        sm: "h-8 px-3 text-xs",
        lg: "h-10 px-5",
        icon: "h-9 w-9 p-0",
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
