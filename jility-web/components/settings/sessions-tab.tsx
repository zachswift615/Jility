'use client'

interface Session {
  id: string
  ip_address: string | null
  user_agent: string | null
  created_at: string
  expires_at: string
}

interface SessionsTabProps {
  sessions: Session[]
}

export function SessionsTab({ sessions }: SessionsTabProps) {
  return (
    <div>
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
    </div>
  )
}
