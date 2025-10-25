import type { Metadata } from 'next'
import './globals.css'
import { Providers } from './providers'
import { Navbar } from '@/components/layout/navbar'
import { MobileBottomNav } from '@/components/layout/mobile-bottom-nav'

export const metadata: Metadata = {
  title: 'Jility - AI-Native Project Management',
  description: 'Beautiful, Linear-inspired project management with AI agents',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className="font-sans antialiased">
        <Providers>
          <div className="min-h-screen flex flex-col">
            <Navbar />
            <main className="flex-1 mb-mobile-nav md:mb-0">
              {children}
            </main>
            <MobileBottomNav />
          </div>
        </Providers>
      </body>
    </html>
  )
}
