"use client"

import { motion, AnimatePresence } from "framer-motion"
import { Bell, User, Search, AlertCircle, Info, CheckCircle2, AlertTriangle, X, Loader2, LogOut, Settings } from "lucide-react"
import { useState, useEffect, useCallback, useRef } from "react"

import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Button } from "@/components/ui/button"
import { useAuth } from "@/contexts/auth-context"
import { notificationApi, profileApi, isTauri, type SystemEvent } from "@/lib/tauri"

interface Profile {
  id: string
  name: string
  description?: string
  icon?: string
  color?: string
}

interface TopBarProps {
  currentView: string
  onNavigateToProfiles?: () => void
  onNavigateToSettings?: () => void
}

const viewTitles: Record<string, { title: string; description: string }> = {
  dashboard: {
    title: "Dashboard",
    description: "View and activate your workspace profiles",
  },
  profiles: {
    title: "Profiles",
    description: "Create and manage workspace layouts",
  },
  monitor: {
    title: "Monitor Editor",
    description: "Configure monitor arrangements and window positions",
  },
  capture: {
    title: "System Capture",
    description: "Detect and capture your current monitor and window layout",
  },
  settings: {
    title: "Settings",
    description: "Customize automation rules and preferences",
  },
}

export function TopBar({ currentView, onNavigateToProfiles, onNavigateToSettings }: TopBarProps) {
  const view = viewTitles[currentView] || viewTitles.dashboard
  const { user, isAuthenticated, isConfigured, signOut } = useAuth()
  const [showNotifications, setShowNotifications] = useState(false)
  const [showUserMenu, setShowUserMenu] = useState(false)
  const [notifications, setNotifications] = useState<SystemEvent[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [hasUnread, setHasUnread] = useState(true)
  
  // Search state
  const [searchQuery, setSearchQuery] = useState("")
  const [searchResults, setSearchResults] = useState<Profile[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)
  const [isSearching, setIsSearching] = useState(false)
  const [allProfiles, setAllProfiles] = useState<Profile[]>([])
  const searchInputRef = useRef<HTMLInputElement>(null)

  // Fetch profiles for search
  useEffect(() => {
    const fetchProfiles = async () => {
      if (!isTauri()) {
        // Mock data for development
        setAllProfiles([
          { id: "1", name: "Work Setup", description: "Development environment", color: "#3b82f6" },
          { id: "2", name: "Gaming", description: "Gaming configuration", color: "#10b981" },
          { id: "3", name: "Streaming", description: "OBS and streaming setup", color: "#8b5cf6" },
        ])
        return
      }
      try {
        const profiles = await profileApi.getProfiles()
        setAllProfiles(profiles || [])
      } catch (error) {
        // Silently fail - search will just be empty
      }
    }
    fetchProfiles()
  }, [])

  // Search profiles
  useEffect(() => {
    if (!searchQuery.trim()) {
      setSearchResults([])
      setShowSearchResults(false)
      return
    }

    setIsSearching(true)
    const query = searchQuery.toLowerCase()
    const results = allProfiles.filter(
      (profile) =>
        profile.name.toLowerCase().includes(query) ||
        (profile.description?.toLowerCase().includes(query) ?? false)
    )
    setSearchResults(results)
    setShowSearchResults(true)
    setIsSearching(false)
  }, [searchQuery, allProfiles])

  const handleProfileSelect = async (profile: Profile) => {
    setSearchQuery("")
    setShowSearchResults(false)
    
    if (isTauri()) {
      try {
        await profileApi.activateProfile(profile.id)
      } catch (error) {
        // Silently fail - will still navigate
      }
    }
    
    // Navigate to profiles view
    if (onNavigateToProfiles) {
      onNavigateToProfiles()
    }
  }

  const fetchNotifications = useCallback(async () => {
    if (!isTauri()) {
      // Mock data for development
      setNotifications([
        {
          id: "1",
          eventType: "profile_activated",
          severity: "info",
          source: "ProfileService",
          message: "Profile 'Work' activated successfully",
          createdAt: new Date().toISOString(),
        },
        {
          id: "2",
          eventType: "monitor_detected",
          severity: "info",
          source: "SystemService",
          message: "External monitor connected",
          createdAt: new Date(Date.now() - 3600000).toISOString(),
        },
      ])
      return
    }

    setIsLoading(true)
    try {
      const events = await notificationApi.getRecentNotifications(10)
      setNotifications(events || [])
    } catch (error) {
      setNotifications([])
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    if (showNotifications) {
      fetchNotifications()
      setHasUnread(false)
    }
  }, [showNotifications, fetchNotifications])

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case "error":
        return <AlertCircle className="w-4 h-4 text-red-500" />
      case "warning":
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />
      case "success":
        return <CheckCircle2 className="w-4 h-4 text-green-500" />
      default:
        return <Info className="w-4 h-4 text-blue-500" />
    }
  }

  const formatTime = (dateStr: string) => {
    const date = new Date(dateStr)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    const diffHours = Math.floor(diffMs / 3600000)
    const diffDays = Math.floor(diffMs / 86400000)

    if (diffMins < 1) return "Just now"
    if (diffMins < 60) return `${diffMins}m ago`
    if (diffHours < 24) return `${diffHours}h ago`
    return `${diffDays}d ago`
  }

  return (
    <div className="border-b border-border bg-card/50 backdrop-blur-sm relative z-50">
      <div className="flex items-center justify-between h-16 px-8 gap-6">
        {/* Title Section */}
        <motion.div initial={{ opacity: 0, x: -20 }} animate={{ opacity: 1, x: 0 }} key={currentView}>
          <h2 className="text-xl font-bold text-foreground">{view.title}</h2>
          <p className="text-sm text-muted-foreground">{view.description}</p>
        </motion.div>

        {/* Right Section */}
        <div className="flex items-center gap-4 ml-auto">
          {/* Search */}
          <div className="relative hidden md:flex items-center">
            <Search className="w-4 h-4 absolute left-3 text-muted-foreground z-10" />
            <input
              ref={searchInputRef}
              type="text"
              placeholder="Search profiles..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onFocus={() => searchQuery && setShowSearchResults(true)}
              onBlur={() => setTimeout(() => setShowSearchResults(false), 200)}
              className="pl-9 pr-4 py-2 w-56 bg-input border border-border rounded-lg text-sm text-foreground placeholder-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            />
            
            {/* Search Results Dropdown */}
            <AnimatePresence>
              {showSearchResults && (
                <motion.div
                  initial={{ opacity: 0, y: -10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  className="absolute top-full left-0 right-0 mt-2 bg-card border border-border rounded-lg shadow-xl z-100 overflow-hidden"
                >
                  {isSearching ? (
                    <div className="flex items-center justify-center py-4">
                      <Loader2 className="w-4 h-4 animate-spin text-muted-foreground" />
                    </div>
                  ) : searchResults.length === 0 ? (
                    <div className="py-4 px-3 text-center text-sm text-muted-foreground">
                      No profiles found
                    </div>
                  ) : (
                    <div className="max-h-64 overflow-y-auto">
                      {searchResults.map((profile) => (
                        <button
                          key={profile.id}
                          onClick={() => handleProfileSelect(profile)}
                          className="w-full px-3 py-2 flex items-center gap-3 hover:bg-muted/50 transition-colors text-left"
                        >
                          <div 
                            className="w-8 h-8 rounded-lg flex items-center justify-center text-white text-sm font-medium"
                            style={{ backgroundColor: profile.color || "#6366f1" }}
                          >
                            {profile.icon || profile.name.charAt(0).toUpperCase()}
                          </div>
                          <div className="flex-1 min-w-0">
                            <p className="text-sm font-medium text-foreground truncate">{profile.name}</p>
                            {profile.description && (
                              <p className="text-xs text-muted-foreground truncate">{profile.description}</p>
                            )}
                          </div>
                        </button>
                      ))}
                    </div>
                  )}
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          {/* Notifications */}
          <div className="relative">
            <Button 
              variant="ghost" 
              size="icon" 
              className="relative"
              onClick={() => setShowNotifications(!showNotifications)}
            >
              <Bell className="w-5 h-5" />
              {hasUnread && (notifications?.length ?? 0) > 0 && (
                <span className="absolute top-1 right-1 w-2 h-2 bg-primary rounded-full" />
              )}
            </Button>

            <AnimatePresence>
              {showNotifications && (
                <>
                  {/* Backdrop */}
                  <div 
                    className="fixed inset-0 z-40" 
                    onClick={() => setShowNotifications(false)}
                  />
                  
                  {/* Dropdown */}
                  <motion.div
                    initial={{ opacity: 0, y: -10, scale: 0.95 }}
                    animate={{ opacity: 1, y: 0, scale: 1 }}
                    exit={{ opacity: 0, y: -10, scale: 0.95 }}
                    transition={{ duration: 0.15 }}
                    className="absolute right-0 top-full mt-2 w-80 bg-card border border-border rounded-lg shadow-xl z-100 overflow-hidden"
                  >
                    {/* Header */}
                    <div className="flex items-center justify-between px-4 py-3 border-b border-border">
                      <h3 className="font-semibold text-foreground">Notifications</h3>
                      <Button 
                        variant="ghost" 
                        size="icon" 
                        className="h-6 w-6"
                        onClick={() => setShowNotifications(false)}
                      >
                        <X className="w-4 h-4" />
                      </Button>
                    </div>

                    {/* Content */}
                    <div className="max-h-80 overflow-y-auto">
                      {isLoading ? (
                        <div className="flex items-center justify-center py-8">
                          <Loader2 className="w-5 h-5 animate-spin text-muted-foreground" />
                        </div>
                      ) : !notifications || notifications.length === 0 ? (
                        <div className="py-8 text-center text-muted-foreground">
                          <Bell className="w-8 h-8 mx-auto mb-2 opacity-50" />
                          <p className="text-sm">No notifications yet</p>
                        </div>
                      ) : (
                        <div className="divide-y divide-border">
                          {notifications.map((notification) => (
                            <div
                              key={notification.id}
                              className="px-4 py-3 hover:bg-muted/50 transition-colors"
                            >
                              <div className="flex items-start gap-3">
                                {getSeverityIcon(notification.severity)}
                                <div className="flex-1 min-w-0">
                                  <p className="text-sm text-foreground line-clamp-2">
                                    {notification.message}
                                  </p>
                                  <div className="flex items-center gap-2 mt-1">
                                    <span className="text-xs text-muted-foreground">
                                      {notification.source}
                                    </span>
                                    <span className="text-xs text-muted-foreground">•</span>
                                    <span className="text-xs text-muted-foreground">
                                      {formatTime(notification.createdAt)}
                                    </span>
                                  </div>
                                </div>
                              </div>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>

                    {/* Footer */}
                    {notifications && notifications.length > 0 && (
                      <div className="px-4 py-2 border-t border-border bg-card">
                        <Button 
                          variant="ghost" 
                          size="sm" 
                          className="w-full text-xs text-muted-foreground hover:text-foreground gap-2"
                          onClick={() => {
                            setNotifications([])
                            setHasUnread(false)
                          }}
                        >
                          <X className="w-3 h-3" />
                          Clear all notifications
                        </Button>
                      </div>
                    )}
                  </motion.div>
                </>
              )}
            </AnimatePresence>
          </div>

          {/* User Profile */}
          <div className="relative">
            <Button 
              variant="ghost" 
              size="icon"
              onClick={() => setShowUserMenu(!showUserMenu)}
              className="relative"
            >
              {isConfigured && isAuthenticated && user ? (
                <Avatar className="h-8 w-8">
                  <AvatarImage src={user.user_metadata?.avatar_url} alt={user.email || "User"} />
                  <AvatarFallback className="bg-primary/10 text-primary text-xs">
                    {user.email?.charAt(0).toUpperCase() || "U"}
                  </AvatarFallback>
                </Avatar>
              ) : (
                <User className="w-5 h-5" />
              )}
            </Button>

            <AnimatePresence>
              {showUserMenu && (
                <>
                  {/* Backdrop */}
                  <div 
                    className="fixed inset-0 z-40" 
                    onClick={() => setShowUserMenu(false)}
                  />
                  
                  {/* Dropdown */}
                  <motion.div
                    initial={{ opacity: 0, y: -10, scale: 0.95 }}
                    animate={{ opacity: 1, y: 0, scale: 1 }}
                    exit={{ opacity: 0, y: -10, scale: 0.95 }}
                    transition={{ duration: 0.15 }}
                    className="absolute right-0 top-full mt-2 w-56 bg-card border border-border rounded-lg shadow-xl z-100 overflow-hidden"
                  >
                    {isConfigured && isAuthenticated && user ? (
                      <>
                        {/* User Info */}
                        <div className="px-4 py-3 border-b border-border">
                          <p className="text-sm font-medium text-foreground truncate">
                            {user.user_metadata?.full_name || user.email?.split("@")[0] || "User"}
                          </p>
                          <p className="text-xs text-muted-foreground truncate">
                            {user.email}
                          </p>
                        </div>
                        
                        {/* Menu Items */}
                        <div className="py-1">
                          <button
                            onClick={() => {
                              setShowUserMenu(false)
                              onNavigateToSettings?.()
                            }}
                            className="w-full px-4 py-2 text-left text-sm text-foreground hover:bg-muted/50 flex items-center gap-2"
                          >
                            <Settings className="w-4 h-4" />
                            Account Settings
                          </button>
                          <button
                            onClick={async () => {
                              setShowUserMenu(false)
                              await signOut()
                            }}
                            className="w-full px-4 py-2 text-left text-sm text-destructive hover:bg-destructive/10 flex items-center gap-2"
                          >
                            <LogOut className="w-4 h-4" />
                            Sign Out
                          </button>
                        </div>
                      </>
                    ) : (
                      <div className="px-4 py-3">
                        <p className="text-sm text-muted-foreground">
                          {isConfigured ? "Not signed in" : "Local mode"}
                        </p>
                        <p className="text-xs text-muted-foreground mt-1">
                          {isConfigured 
                            ? "Sign in to sync your profiles" 
                            : "Data stored locally only"}
                        </p>
                      </div>
                    )}
                  </motion.div>
                </>
              )}
            </AnimatePresence>
          </div>
        </div>
      </div>
    </div>
  )
}
