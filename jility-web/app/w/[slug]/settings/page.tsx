'use client'

import { useEffect, useState } from 'react'
import { useParams, useSearchParams } from 'next/navigation'
import { useWorkspace } from '@/lib/workspace-context'
import { withAuth, WithAuthProps } from '@/lib/with-auth'
import { api } from '@/lib/api'
import type { WorkspaceMember, PendingInvite, Project } from '@/lib/types'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { ProfileTab } from '@/components/settings/profile-tab'
import { WorkspaceTab } from '@/components/settings/workspace-tab'
import { ProjectsTab } from '@/components/settings/projects-tab'
import { ApiKeysTab } from '@/components/settings/api-keys-tab'
import { SessionsTab } from '@/components/settings/sessions-tab'

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

function SettingsPage({ user }: WithAuthProps) {
  const params = useParams()
  const searchParams = useSearchParams()
  const slug = params.slug as string
  const { currentWorkspace } = useWorkspace()

  // Get tab from URL query param, default to 'profile'
  const initialTab = searchParams.get('tab') || 'profile'
  const [activeTab, setActiveTab] = useState(initialTab)

  // Workspace data
  const [members, setMembers] = useState<WorkspaceMember[]>([])
  const [pendingInvites, setPendingInvites] = useState<PendingInvite[]>([])

  // Projects data
  const [projects, setProjects] = useState<Project[]>([])
  const [isLoadingProjects, setIsLoadingProjects] = useState(true)

  // Profile data
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([])
  const [sessions, setSessions] = useState<Session[]>([])

  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    if (slug) {
      loadData()
    }
  }, [slug])

  // Update active tab when URL param changes
  useEffect(() => {
    const tab = searchParams.get('tab')
    if (tab) {
      setActiveTab(tab)
    }
  }, [searchParams])

  const loadData = async () => {
    try {
      setIsLoading(true)
      setIsLoadingProjects(true)

      // Load all data in parallel
      const [
        membersData,
        invitesData,
        projectsData,
        keysData,
        sessionsData,
      ] = await Promise.all([
        api.listWorkspaceMembers(slug),
        api.listPendingInvites(slug).catch(() => []), // Gracefully handle if user is not admin
        api.listProjects().catch(() => []),
        api.listApiKeys(),
        api.listSessions(),
      ])

      setMembers(membersData)
      setPendingInvites(invitesData)
      setProjects(projectsData)
      setApiKeys(keysData)
      setSessions(sessionsData)
    } catch (error) {
      console.error('Failed to load data:', error)
    } finally {
      setIsLoading(false)
      setIsLoadingProjects(false)
    }
  }

  const isAdmin = currentWorkspace?.role === 'admin'

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

  return (
    <div className="container max-w-5xl py-8 pb-24 md:pb-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Settings</h1>
        <p className="text-muted-foreground">
          Manage your account, workspace, projects, and API access
        </p>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
        <TabsList className="grid w-full grid-cols-5 mb-8">
          <TabsTrigger value="profile">Profile</TabsTrigger>
          <TabsTrigger value="workspace">Workspace</TabsTrigger>
          <TabsTrigger value="projects">Projects</TabsTrigger>
          <TabsTrigger value="api-keys">API Keys</TabsTrigger>
          <TabsTrigger value="sessions">Sessions</TabsTrigger>
        </TabsList>

        <TabsContent value="profile">
          <ProfileTab user={user} />
        </TabsContent>

        <TabsContent value="workspace">
          <WorkspaceTab
            slug={slug}
            members={members}
            pendingInvites={pendingInvites}
            isAdmin={isAdmin}
            onUpdate={loadData}
          />
        </TabsContent>

        <TabsContent value="projects">
          <ProjectsTab projects={projects} isLoading={isLoadingProjects} />
        </TabsContent>

        <TabsContent value="api-keys">
          <ApiKeysTab apiKeys={apiKeys} onUpdate={loadData} />
        </TabsContent>

        <TabsContent value="sessions">
          <SessionsTab sessions={sessions} />
        </TabsContent>
      </Tabs>
    </div>
  )
}

export default withAuth(SettingsPage)
