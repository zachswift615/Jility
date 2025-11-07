'use client'

import { useEffect, useState } from 'react'
import { useParams, useRouter } from 'next/navigation'
import { api } from '@/lib/api'
import type { InviteDetails } from '@/lib/types'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Loader2, UserPlus, AlertCircle } from 'lucide-react'
import Link from 'next/link'

export default function InviteAcceptPage() {
  const params = useParams()
  const router = useRouter()
  const token = params.token as string

  const [inviteDetails, setInviteDetails] = useState<InviteDetails | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [isAccepting, setIsAccepting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [isLoggedIn, setIsLoggedIn] = useState(false)

  useEffect(() => {
    // Check if user is logged in
    const token = localStorage.getItem('jility_token')
    setIsLoggedIn(!!token)

    // Load invite details
    loadInviteDetails()
  }, [])

  const loadInviteDetails = async () => {
    try {
      setIsLoading(true)
      const details = await api.getInviteDetails(token)
      setInviteDetails(details)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load invite details')
    } finally {
      setIsLoading(false)
    }
  }

  const handleAccept = async () => {
    if (!isLoggedIn) {
      // Redirect to login with return URL
      router.push(`/login?redirect=/invite/${token}`)
      return
    }

    try {
      setIsAccepting(true)
      setError(null)
      const workspace = await api.acceptInvite(token)
      // Redirect to workspace after successful acceptance
      router.push(`/w/${workspace.slug}`)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to accept invite')
      setIsAccepting(false)
    }
  }

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  if (error && !inviteDetails) {
    return (
      <div className="min-h-screen flex items-center justify-center p-4">
        <Card className="max-w-md w-full">
          <CardHeader>
            <div className="flex items-center gap-2 text-destructive">
              <AlertCircle className="h-5 w-5" />
              <CardTitle>Invite Not Found</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">{error}</p>
          </CardContent>
          <CardFooter>
            <Link href="/login" className="w-full">
              <Button className="w-full">Go to Login</Button>
            </Link>
          </CardFooter>
        </Card>
      </div>
    )
  }

  if (!inviteDetails) {
    return null
  }

  const isExpired = inviteDetails.is_expired

  return (
    <div className="min-h-screen flex items-center justify-center p-4">
      <Card className="max-w-md w-full">
        <CardHeader>
          <div className="flex items-center gap-2">
            <UserPlus className="h-5 w-5" />
            <CardTitle>Workspace Invitation</CardTitle>
          </div>
          <CardDescription>
            You've been invited to join a workspace
          </CardDescription>
        </CardHeader>

        <CardContent className="space-y-4">
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Workspace:</span>
              <span className="font-medium">{inviteDetails.workspace_name}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Invited by:</span>
              <span className="font-medium">{inviteDetails.invited_by_email}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Role:</span>
              <Badge variant={inviteDetails.role === 'admin' ? 'default' : 'secondary'}>
                {inviteDetails.role}
              </Badge>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">Expires:</span>
              <span className={`text-sm ${isExpired ? 'text-destructive' : ''}`}>
                {new Date(inviteDetails.expires_at).toLocaleDateString()}
                {isExpired && ' (Expired)'}
              </span>
            </div>
          </div>

          {isExpired && (
            <div className="bg-destructive/10 border border-destructive/20 rounded-md p-3">
              <p className="text-sm text-destructive">
                This invitation has expired. Please contact the workspace admin for a new invitation.
              </p>
            </div>
          )}

          {error && (
            <div className="bg-destructive/10 border border-destructive/20 rounded-md p-3">
              <p className="text-sm text-destructive">{error}</p>
            </div>
          )}
        </CardContent>

        <CardFooter className="flex flex-col gap-2">
          {isLoggedIn ? (
            <Button
              className="w-full"
              onClick={handleAccept}
              disabled={isExpired || isAccepting}
            >
              {isAccepting && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Accept Invitation
            </Button>
          ) : (
            <>
              <Link href={`/login?redirect=/invite/${token}`} className="w-full">
                <Button className="w-full">
                  Log In to Accept
                </Button>
              </Link>
              <Link href={`/register?redirect=/invite/${token}`} className="w-full">
                <Button variant="outline" className="w-full">
                  Create Account
                </Button>
              </Link>
            </>
          )}
        </CardFooter>
      </Card>
    </div>
  )
}
