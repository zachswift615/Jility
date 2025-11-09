'use client'

import { useAuth, User } from '@/lib/auth-context'
import { ThemeSwitcher } from '@/components/theme-switcher'

interface ProfileTabProps {
  user: User
}

export function ProfileTab({ user }: ProfileTabProps) {
  const { logout } = useAuth()

  return (
    <div className="space-y-6">
      {/* Preferences */}
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-4">Preferences</h2>
        <div className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-3 block">Theme</label>
            <ThemeSwitcher />
          </div>
        </div>
      </div>

      {/* User Info */}
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-4">Account Information</h2>
        <dl className="space-y-3">
          <div>
            <dt className="text-sm font-medium text-muted-foreground">Email</dt>
            <dd className="text-base">{user.email}</dd>
          </div>
          <div>
            <dt className="text-sm font-medium text-muted-foreground">Username</dt>
            <dd className="text-base">{user.username}</dd>
          </div>
          {user.full_name && (
            <div>
              <dt className="text-sm font-medium text-muted-foreground">Full Name</dt>
              <dd className="text-base">{user.full_name}</dd>
            </div>
          )}
          <div>
            <dt className="text-sm font-medium text-muted-foreground">User ID</dt>
            <dd className="text-base font-mono text-sm">{user.id}</dd>
          </div>
        </dl>
        <div className="mt-6">
          <button
            onClick={logout}
            className="px-4 py-2 bg-destructive text-destructive-foreground rounded-md hover:bg-destructive/90"
          >
            Sign Out
          </button>
        </div>
      </div>
    </div>
  )
}
