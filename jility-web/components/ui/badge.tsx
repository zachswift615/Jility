import * as React from 'react'
import { cva, type VariantProps } from 'class-variance-authority'
import { cn } from '@/lib/utils'

const badgeVariants = cva(
  'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
  {
    variants: {
      variant: {
        default: 'border-transparent bg-primary text-primary-foreground hover:bg-primary/80',
        secondary: 'border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80',
        destructive: 'border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80',
        outline: 'text-foreground',
        backlog: 'border-transparent bg-status-backlog/10 text-status-backlog border-status-backlog/20',
        todo: 'border-transparent bg-status-todo/10 text-status-todo border-status-todo/20',
        'in-progress': 'border-transparent bg-status-in-progress/10 text-status-in-progress border-status-in-progress/20',
        review: 'border-transparent bg-status-review/10 text-status-review border-status-review/20',
        done: 'border-transparent bg-status-done/10 text-status-done border-status-done/20',
        blocked: 'border-transparent bg-status-blocked/10 text-status-blocked border-status-blocked/20',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
)

export interface BadgeProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof badgeVariants> {}

function Badge({ className, variant, ...props }: BadgeProps) {
  return <div className={cn(badgeVariants({ variant }), className)} {...props} />
}

export { Badge, badgeVariants }
