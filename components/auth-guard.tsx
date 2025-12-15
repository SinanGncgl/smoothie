"use client"

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/contexts/auth-context'
import { Loader2, Monitor } from 'lucide-react'

interface AuthGuardProps {
  children: React.ReactNode
  fallback?: React.ReactNode
}

export function AuthGuard({ children, fallback }: AuthGuardProps) {
  const router = useRouter()
  const { isAuthenticated, isLoading, isConfigured } = useAuth()

  useEffect(() => {
    // Only redirect to login if Supabase is configured and user is not authenticated
    if (!isLoading && isConfigured && !isAuthenticated) {
      router.push('/login')
    }
  }, [isAuthenticated, isLoading, isConfigured, router])

  // Show loading state
  if (isLoading) {
    return fallback ?? (
      <div className="min-h-screen flex items-center justify-center bg-background">
        <div className="flex flex-col items-center gap-4">
          <div className="w-16 h-16 bg-primary/10 rounded-2xl flex items-center justify-center animate-pulse">
            <Monitor className="w-8 h-8 text-primary" />
          </div>
          <div className="flex items-center gap-2 text-muted-foreground">
            <Loader2 className="w-4 h-4 animate-spin" />
            <span>Loading...</span>
          </div>
        </div>
      </div>
    )
  }

  // If Supabase is not configured, allow access (local-only mode)
  if (!isConfigured) {
    return <>{children}</>
  }

  // If authenticated, render children
  if (isAuthenticated) {
    return <>{children}</>
  }

  // Not authenticated - show loading while redirecting
  return fallback ?? (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <div className="flex flex-col items-center gap-4">
        <div className="w-16 h-16 bg-primary/10 rounded-2xl flex items-center justify-center">
          <Monitor className="w-8 h-8 text-primary" />
        </div>
        <div className="flex items-center gap-2 text-muted-foreground">
          <Loader2 className="w-4 h-4 animate-spin" />
          <span>Redirecting to login...</span>
        </div>
      </div>
    </div>
  )
}

// Higher-order component version
export function withAuthGuard<P extends object>(
  WrappedComponent: React.ComponentType<P>
) {
  return function AuthGuardedComponent(props: P) {
    return (
      <AuthGuard>
        <WrappedComponent {...props} />
      </AuthGuard>
    )
  }
}
