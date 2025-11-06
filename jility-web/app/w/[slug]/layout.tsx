'use client'

import { ReactNode } from 'react'
import { useParams } from 'next/navigation'

interface WorkspaceLayoutProps {
  children: ReactNode
}

export default function WorkspaceLayout({ children }: WorkspaceLayoutProps) {
  const params = useParams()
  const slug = params.slug as string

  // The WorkspaceProvider in root layout will handle loading workspace data
  // This layout just ensures we're in a workspace context
  return <>{children}</>
}
