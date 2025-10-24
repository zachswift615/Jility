'use client'

import { ThemeProvider as NextThemesProvider } from 'next-themes'
import { type ThemeProviderProps } from 'next-themes/dist/types'
import { AuthProvider } from '@/lib/auth-context'
import { ProjectProvider } from '@/lib/project-context'

export function Providers({ children, ...props }: ThemeProviderProps) {
  return (
    <NextThemesProvider
      attribute="class"
      defaultTheme="light"
      enableSystem
      disableTransitionOnChange
      {...props}
    >
      <AuthProvider>
        <ProjectProvider>
          {children}
        </ProjectProvider>
      </AuthProvider>
    </NextThemesProvider>
  )
}
