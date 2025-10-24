'use client'

import { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { useRouter } from 'next/navigation'

export interface User {
  id: string
  email: string
  username: string
  full_name?: string
  avatar_url?: string
}

export interface RegisterData {
  email: string
  username: string
  password: string
  full_name?: string
}

interface AuthResponse {
  token: string
  expires_at: number
  user: User
}

interface AuthContextType {
  user: User | null
  login: (email: string, password: string) => Promise<void>
  logout: () => Promise<void>
  register: (data: RegisterData) => Promise<void>
  isLoading: boolean
  isAuthenticated: boolean
}

const AuthContext = createContext<AuthContextType | null>(null)

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api'

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const router = useRouter()

  useEffect(() => {
    // Check for existing token on mount
    const checkAuth = async () => {
      const token = localStorage.getItem('jility_token')
      if (token) {
        try {
          // Validate token by fetching current user
          const response = await fetch(`${API_BASE}/auth/me`, {
            headers: {
              Authorization: `Bearer ${token}`,
            },
          })

          if (response.ok) {
            const userData = await response.json()
            setUser(userData)
          } else {
            // Token is invalid, clear it
            localStorage.removeItem('jility_token')
          }
        } catch (error) {
          console.error('Error checking auth:', error)
          localStorage.removeItem('jility_token')
        }
      }
      setIsLoading(false)
    }

    checkAuth()
  }, [])

  const login = async (email: string, password: string) => {
    try {
      const response = await fetch(`${API_BASE}/auth/login`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
      })

      if (!response.ok) {
        const error = await response.json()
        throw new Error(error.message || 'Login failed')
      }

      const data: AuthResponse = await response.json()
      localStorage.setItem('jility_token', data.token)
      setUser(data.user)
      router.push('/')
    } catch (error) {
      throw error
    }
  }

  const logout = async () => {
    const token = localStorage.getItem('jility_token')

    if (token) {
      try {
        // Call logout endpoint to revoke session
        await fetch(`${API_BASE}/auth/logout`, {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${token}`,
          },
        })
      } catch (error) {
        console.error('Error during logout:', error)
      }
    }

    localStorage.removeItem('jility_token')
    setUser(null)
    router.push('/login')
  }

  const register = async (data: RegisterData) => {
    try {
      const response = await fetch(`${API_BASE}/auth/register`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      })

      if (!response.ok) {
        const error = await response.json()
        throw new Error(error.message || 'Registration failed')
      }

      const authData: AuthResponse = await response.json()
      localStorage.setItem('jility_token', authData.token)
      setUser(authData.user)
      router.push('/')
    } catch (error) {
      throw error
    }
  }

  return (
    <AuthContext.Provider
      value={{
        user,
        login,
        logout,
        register,
        isLoading,
        isAuthenticated: !!user,
      }}
    >
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
