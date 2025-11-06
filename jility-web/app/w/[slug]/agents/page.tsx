'use client'

import { useEffect, useState } from 'react'
import { useParams } from 'next/navigation'
import { api } from '@/lib/api'
import type { Ticket } from '@/lib/types'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { getStatusLabel, formatDate } from '@/lib/utils'
import { Bot, TrendingUp, CheckCircle2, Clock } from 'lucide-react'
import Link from 'next/link'

export default function AgentsPage() {
  const params = useParams()
  const slug = params.slug as string
  const [tickets, setTickets] = useState<Ticket[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadAgentTickets()
  }, [])

  const loadAgentTickets = async () => {
    try {
      const allTickets = await api.listTickets()
      // Filter tickets assigned to agents (assignees starting with "agent-")
      const agentTickets = allTickets.filter((ticket) =>
        ticket.assignees.some((assignee) => assignee.startsWith('agent-'))
      )
      setTickets(agentTickets)
    } catch (error) {
      console.error('Failed to load agent tickets:', error)
    } finally {
      setLoading(false)
    }
  }

  // Get unique agents
  const agents = Array.from(
    new Set(
      tickets.flatMap((ticket) =>
        ticket.assignees.filter((assignee) => assignee.startsWith('agent-'))
      )
    )
  )

  // Calculate metrics
  const getAgentMetrics = (agentName: string) => {
    const agentTickets = tickets.filter((ticket) =>
      ticket.assignees.includes(agentName)
    )

    return {
      total: agentTickets.length,
      completed: agentTickets.filter((t) => t.status === 'done').length,
      inProgress: agentTickets.filter((t) => t.status === 'in_progress').length,
      avgStoryPoints:
        agentTickets.reduce((sum, t) => sum + (t.story_points || 0), 0) /
        (agentTickets.length || 1),
    }
  }

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-muted-foreground">Loading agent activity...</div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Agent Dashboard</h1>
        <p className="text-muted-foreground">
          Track AI agent activity and productivity
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <Bot className="h-4 w-4 text-primary" />
              Active Agents
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{agents.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <TrendingUp className="h-4 w-4 text-primary" />
              Total Tickets
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{tickets.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <CheckCircle2 className="h-4 w-4 text-status-done" />
              Completed
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {tickets.filter((t) => t.status === 'done').length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium flex items-center gap-2">
              <Clock className="h-4 w-4 text-status-in-progress" />
              In Progress
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {tickets.filter((t) => t.status === 'in_progress').length}
            </div>
          </CardContent>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="space-y-6">
          <h2 className="text-2xl font-bold">Agents</h2>

          {agents.length === 0 ? (
            <Card>
              <CardContent className="py-8 text-center text-muted-foreground">
                No agent activity yet
              </CardContent>
            </Card>
          ) : (
            <div className="space-y-4">
              {agents.map((agent) => {
                const metrics = getAgentMetrics(agent)
                return (
                  <Card key={agent}>
                    <CardHeader>
                      <div className="flex items-center gap-3">
                        <Avatar>
                          <AvatarFallback className="bg-primary text-primary-foreground">
                            <Bot className="h-5 w-5" />
                          </AvatarFallback>
                        </Avatar>
                        <div>
                          <CardTitle>{agent}</CardTitle>
                          <CardDescription>
                            {metrics.total} tickets assigned
                          </CardDescription>
                        </div>
                      </div>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-3 gap-4 text-sm">
                        <div>
                          <div className="text-muted-foreground">Completed</div>
                          <div className="text-lg font-semibold text-status-done">
                            {metrics.completed}
                          </div>
                        </div>
                        <div>
                          <div className="text-muted-foreground">In Progress</div>
                          <div className="text-lg font-semibold text-status-in-progress">
                            {metrics.inProgress}
                          </div>
                        </div>
                        <div>
                          <div className="text-muted-foreground">Avg Points</div>
                          <div className="text-lg font-semibold">
                            {metrics.avgStoryPoints.toFixed(1)}
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                )
              })}
            </div>
          )}
        </div>

        <div className="space-y-6">
          <h2 className="text-2xl font-bold">Recent Agent Activity</h2>

          {tickets.length === 0 ? (
            <Card>
              <CardContent className="py-8 text-center text-muted-foreground">
                No tickets assigned to agents yet
              </CardContent>
            </Card>
          ) : (
            <div className="space-y-3">
              {tickets.slice(0, 10).map((ticket) => (
                <Link key={ticket.id} href={`/w/${slug}/ticket/${ticket.id}`}>
                  <Card className="hover:border-primary/50 transition-colors cursor-pointer">
                    <CardContent className="pt-6">
                      <div className="flex items-start justify-between gap-4">
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-2">
                            <span className="text-xs text-muted-foreground font-mono">
                              {ticket.number}
                            </span>
                            <Badge variant={ticket.status as any}>
                              {getStatusLabel(ticket.status)}
                            </Badge>
                          </div>
                          <h3 className="font-medium mb-2 line-clamp-2">
                            {ticket.title}
                          </h3>
                          <div className="flex items-center gap-2 flex-wrap">
                            {ticket.assignees
                              .filter((a) => a.startsWith('agent-'))
                              .map((agent) => (
                                <div
                                  key={agent}
                                  className="flex items-center gap-1 text-xs text-muted-foreground"
                                >
                                  <Bot className="h-3 w-3" />
                                  {agent}
                                </div>
                              ))}
                          </div>
                        </div>
                        {ticket.story_points && (
                          <div className="text-sm text-muted-foreground">
                            {ticket.story_points} pts
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>
                </Link>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
