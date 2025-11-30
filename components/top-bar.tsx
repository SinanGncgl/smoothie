"use client"

import { motion } from "framer-motion"
import { Bell, User, Search } from "lucide-react"
import { Button } from "@/components/ui/button"

interface TopBarProps {
  currentView: string
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
  settings: {
    title: "Settings",
    description: "Customize automation rules and preferences",
  },
}

export function TopBar({ currentView }: TopBarProps) {
  const view = viewTitles[currentView] || viewTitles.dashboard

  return (
    <div className="border-b border-border bg-card/50 backdrop-blur-sm">
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
            <Search className="w-4 h-4 absolute left-3 text-muted-foreground" />
            <input
              type="text"
              placeholder="Search..."
              className="pl-9 pr-4 py-2 bg-input border border-border rounded-lg text-sm text-foreground placeholder-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            />
          </div>

          {/* Notifications */}
          <Button variant="ghost" size="icon" className="relative">
            <Bell className="w-5 h-5" />
            <span className="absolute top-1 right-1 w-2 h-2 bg-primary rounded-full" />
          </Button>

          {/* User Profile */}
          <Button variant="ghost" size="icon">
            <User className="w-5 h-5" />
          </Button>
        </div>
      </div>
    </div>
  )
}
