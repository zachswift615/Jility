'use client'

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { Project } from './types'
import { api } from './api'

interface ProjectContextType {
  projects: Project[]
  currentProject: Project | null
  isLoading: boolean
  error: string | null
  setCurrentProject: (project: Project | null) => void
  refreshProjects: () => Promise<void>
  createProject: (data: {
    workspace_id: string
    name: string
    description?: string
    key?: string
    color?: string
    ai_planning_enabled?: boolean
    auto_link_git?: boolean
    require_story_points?: boolean
  }) => Promise<Project>
  updateProject: (id: string, data: Partial<Project>) => Promise<Project>
  deleteProject: (id: string) => Promise<void>
}

const ProjectContext = createContext<ProjectContextType | undefined>(undefined)

const CURRENT_PROJECT_KEY = 'jility_current_project_id'

export function ProjectProvider({ children }: { children: ReactNode }) {
  const [projects, setProjects] = useState<Project[]>([])
  const [currentProject, setCurrentProjectState] = useState<Project | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Load projects and restore current project from localStorage
  useEffect(() => {
    loadProjects()
  }, [])

  // Save current project to localStorage when it changes
  useEffect(() => {
    if (currentProject) {
      localStorage.setItem(CURRENT_PROJECT_KEY, currentProject.id)
    } else {
      localStorage.removeItem(CURRENT_PROJECT_KEY)
    }
  }, [currentProject])

  const loadProjects = async () => {
    try {
      setIsLoading(true)
      setError(null)
      const fetchedProjects = await api.listProjects()
      setProjects(fetchedProjects)

      // Restore current project from localStorage or default to first project
      const savedProjectId = localStorage.getItem(CURRENT_PROJECT_KEY)
      if (savedProjectId) {
        const savedProject = fetchedProjects.find(p => p.id === savedProjectId)
        if (savedProject) {
          setCurrentProjectState(savedProject)
        } else if (fetchedProjects.length > 0) {
          setCurrentProjectState(fetchedProjects[0])
        }
      } else if (fetchedProjects.length > 0) {
        setCurrentProjectState(fetchedProjects[0])
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load projects')
    } finally {
      setIsLoading(false)
    }
  }

  const refreshProjects = async () => {
    await loadProjects()
  }

  const setCurrentProject = (project: Project | null) => {
    setCurrentProjectState(project)
  }

  const createProject = async (data: {
    workspace_id: string
    name: string
    description?: string
    key?: string
    color?: string
    ai_planning_enabled?: boolean
    auto_link_git?: boolean
    require_story_points?: boolean
  }): Promise<Project> => {
    try {
      const newProject = await api.createProject(data)
      setProjects(prev => [...prev, newProject])
      // Automatically set as current project
      setCurrentProject(newProject)
      return newProject
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to create project'
      setError(message)
      throw new Error(message)
    }
  }

  const updateProject = async (
    id: string,
    data: Partial<Project>
  ): Promise<Project> => {
    try {
      const updatedProject = await api.updateProject(id, data)
      setProjects(prev => prev.map(p => (p.id === id ? updatedProject : p)))
      // Update current project if it's the one being updated
      if (currentProject?.id === id) {
        setCurrentProject(updatedProject)
      }
      return updatedProject
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to update project'
      setError(message)
      throw new Error(message)
    }
  }

  const deleteProject = async (id: string): Promise<void> => {
    try {
      await api.deleteProject(id)
      setProjects(prev => prev.filter(p => p.id !== id))
      // If deleting current project, switch to first available project
      if (currentProject?.id === id) {
        const remainingProjects = projects.filter(p => p.id !== id)
        setCurrentProject(remainingProjects.length > 0 ? remainingProjects[0] : null)
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to delete project'
      setError(message)
      throw new Error(message)
    }
  }

  return (
    <ProjectContext.Provider
      value={{
        projects,
        currentProject,
        isLoading,
        error,
        setCurrentProject,
        refreshProjects,
        createProject,
        updateProject,
        deleteProject,
      }}
    >
      {children}
    </ProjectContext.Provider>
  )
}

export function useProject() {
  const context = useContext(ProjectContext)
  if (context === undefined) {
    throw new Error('useProject must be used within a ProjectProvider')
  }
  return context
}
