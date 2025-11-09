'use client'

import { useEffect } from 'react'
import { useParams, useRouter } from 'next/navigation'
import { withAuth } from '@/lib/with-auth'

function ProfilePage() {
  const params = useParams()
  const router = useRouter()
  const slug = params.slug as string

  useEffect(() => {
    // Redirect to settings page with profile tab
    router.replace(`/w/${slug}/settings?tab=profile`)
  }, [slug, router])

  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="text-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
        <p className="mt-4 text-muted-foreground">Redirecting to settings...</p>
      </div>
    </div>
  )
}

export default withAuth(ProfilePage)
