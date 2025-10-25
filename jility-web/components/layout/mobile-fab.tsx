'use client'

import { Plus } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

interface MobileFABProps {
  onClick: () => void
  className?: string
}

export function MobileFAB({ onClick, className }: MobileFABProps) {
  return (
    <Button
      onClick={onClick}
      className={cn(
        'fixed right-4 bottom-20 z-40',
        'md:hidden', // Hide on desktop
        'w-14 h-14 rounded-full',
        'shadow-lg hover:shadow-xl',
        'transition-all duration-200',
        className
      )}
      size="icon"
    >
      <Plus className="h-6 w-6" />
    </Button>
  )
}
