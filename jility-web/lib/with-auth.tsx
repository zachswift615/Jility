'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth, User } from './auth-context'

export interface WithAuthProps {
  user: User
}

export function withAuth<P extends object>(
  Component: React.ComponentType<P & WithAuthProps>
) {
  return function AuthenticatedComponent(props: P) {
    const { user, isLoading } = useAuth()
    const router = useRouter()

    useEffect(() => {
      if (!isLoading && !user) {
        router.push('/login')
      }
    }, [user, isLoading, router])

    if (isLoading) {
      return (
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
            <p className="mt-4 text-muted-foreground">Loading...</p>
          </div>
        </div>
      )
    }

    if (!user) {
      return null
    }

    return <Component {...props} user={user} />
  }
}
