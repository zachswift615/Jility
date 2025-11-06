'use client'

import { useEffect, useState } from 'react'
import { useAuth, User } from '@/lib/auth-context'
import { api } from '@/lib/api'
import { withAuth, WithAuthProps } from '@/lib/with-auth'
import { ThemeSwitcher } from '@/components/theme-switcher'

interface ApiKey {
  id: string
  name: string
  prefix: string
  scopes: string[]
  created_at: string
  expires_at: string | null
  last_used_at: string | null
}

interface Session {
  id: string
  ip_address: string | null
  user_agent: string | null
  created_at: string
  expires_at: string
}

function ProfilePage({ user }: WithAuthProps) {
  const { logout } = useAuth()
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([])
  const [sessions, setSessions] = useState<Session[]>([])
  const [loading, setLoading] = useState(true)
  const [showNewKeyModal, setShowNewKeyModal] = useState(false)
  const [newKeyName, setNewKeyName] = useState('')
  const [newKeyData, setNewKeyData] = useState<{ key: string; api_key: ApiKey } | null>(null)

  useEffect(() => {
    loadData()
  }, [])

  const loadData = async () => {
    try {
      const [keys, sessions] = await Promise.all([
        api.listApiKeys(),
        api.listSessions(),
      ])
      setApiKeys(keys)
      setSessions(sessions)
    } catch (error) {
      console.error('Failed to load data:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleCreateApiKey = async () => {
    if (!newKeyName.trim()) {
      alert('Please enter a key name')
      return
    }

    try {
      const result = await api.createApiKey({
        name: newKeyName,
        scopes: ['tickets:read', 'tickets:write'],
        expires_in_days: 365,
      })
      setNewKeyData(result)
      setNewKeyName('')
      await loadData()
    } catch (error) {
      alert('Failed to create API key: ' + (error instanceof Error ? error.message : 'Unknown error'))
    }
  }

  const handleRevokeApiKey = async (id: string) => {
    if (!confirm('Are you sure you want to revoke this API key? This action cannot be undone.')) {
      return
    }

    try {
      await api.revokeApiKey(id)
      await loadData()
    } catch (error) {
      alert('Failed to revoke API key: ' + (error instanceof Error ? error.message : 'Unknown error'))
    }
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    alert('Copied to clipboard!')
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
          <p className="mt-4 text-muted-foreground">Loading...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8 max-w-4xl pb-24 md:pb-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Settings</h1>
        <p className="text-muted-foreground">Manage your account, preferences, and API access</p>
      </div>

      {/* Preferences */}
      <div className="bg-card rounded-lg border p-6 mb-8">
        <h2 className="text-xl font-semibold mb-4">Preferences</h2>
        <div className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-3 block">Theme</label>
            <ThemeSwitcher />
          </div>
        </div>
      </div>

      {/* User Info */}
      <div className="bg-card rounded-lg border p-6 mb-8">
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

      {/* API Keys */}
      <div className="bg-card rounded-lg border p-6 mb-8">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold">API Keys</h2>
          <button
            onClick={() => setShowNewKeyModal(true)}
            className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
          >
            Create API Key
          </button>
        </div>

        {apiKeys.length === 0 ? (
          <p className="text-muted-foreground text-center py-8">
            No API keys yet. Create one to access the API programmatically.
          </p>
        ) : (
          <div className="space-y-4">
            {apiKeys.map((key) => (
              <div key={key.id} className="border rounded-md p-4">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <h3 className="font-semibold">{key.name}</h3>
                    <p className="text-sm text-muted-foreground font-mono mt-1">
                      {key.prefix}...
                    </p>
                    <div className="flex gap-4 mt-2 text-xs text-muted-foreground">
                      <span>Created: {new Date(key.created_at).toLocaleDateString()}</span>
                      {key.last_used_at && (
                        <span>Last used: {new Date(key.last_used_at).toLocaleDateString()}</span>
                      )}
                      {key.expires_at && (
                        <span>Expires: {new Date(key.expires_at).toLocaleDateString()}</span>
                      )}
                    </div>
                    <div className="mt-2 flex flex-wrap gap-1">
                      {key.scopes.map((scope) => (
                        <span
                          key={scope}
                          className="px-2 py-1 text-xs bg-muted rounded"
                        >
                          {scope}
                        </span>
                      ))}
                    </div>
                  </div>
                  <button
                    onClick={() => handleRevokeApiKey(key.id)}
                    className="px-3 py-1 text-sm bg-destructive text-destructive-foreground rounded hover:bg-destructive/90"
                  >
                    Revoke
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Sessions */}
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-4">Active Sessions</h2>
        {sessions.length === 0 ? (
          <p className="text-muted-foreground text-center py-8">No active sessions</p>
        ) : (
          <div className="space-y-4">
            {sessions.map((session) => (
              <div key={session.id} className="border rounded-md p-4">
                <div className="flex items-start justify-between">
                  <div>
                    <p className="text-sm font-medium">
                      {session.user_agent || 'Unknown device'}
                    </p>
                    <p className="text-xs text-muted-foreground mt-1">
                      IP: {session.ip_address || 'Unknown'}
                    </p>
                    <p className="text-xs text-muted-foreground mt-1">
                      Created: {new Date(session.created_at).toLocaleString()}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      Expires: {new Date(session.expires_at).toLocaleString()}
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* New API Key Modal */}
      {showNewKeyModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-card rounded-lg border p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold mb-4">Create API Key</h3>

            {newKeyData ? (
              <div className="space-y-4">
                <div className="p-4 bg-yellow-500/10 border border-yellow-500/20 rounded-md">
                  <p className="text-sm font-medium text-yellow-600 dark:text-yellow-500 mb-2">
                    Important: Copy this key now!
                  </p>
                  <p className="text-xs text-muted-foreground">
                    You won't be able to see it again.
                  </p>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">API Key</label>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={newKeyData.key}
                      readOnly
                      className="flex-1 px-3 py-2 border rounded-md bg-muted font-mono text-sm"
                    />
                    <button
                      onClick={() => copyToClipboard(newKeyData.key)}
                      className="px-3 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                    >
                      Copy
                    </button>
                  </div>
                </div>

                <button
                  onClick={() => {
                    setNewKeyData(null)
                    setShowNewKeyModal(false)
                  }}
                  className="w-full px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                >
                  Done
                </button>
              </div>
            ) : (
              <div className="space-y-4">
                <div>
                  <label htmlFor="keyName" className="block text-sm font-medium mb-2">
                    Key Name
                  </label>
                  <input
                    id="keyName"
                    type="text"
                    value={newKeyName}
                    onChange={(e) => setNewKeyName(e.target.value)}
                    className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary bg-background"
                    placeholder="My API Key"
                  />
                </div>

                <div className="flex gap-2">
                  <button
                    onClick={handleCreateApiKey}
                    className="flex-1 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                  >
                    Create
                  </button>
                  <button
                    onClick={() => {
                      setShowNewKeyModal(false)
                      setNewKeyName('')
                    }}
                    className="flex-1 px-4 py-2 border rounded-md hover:bg-muted"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}

export default withAuth(ProfilePage)
