"use client"

import { motion } from "framer-motion"
import { LayoutGrid, Settings, Monitor, FileText, HelpCircle, LogOut } from "lucide-react"
import { cn } from "@/lib/utils"

interface SidebarProps {
  currentView: string
  setCurrentView: (view: "dashboard" | "profiles" | "monitor" | "settings") => void
}

export function Sidebar({ currentView, setCurrentView }: SidebarProps) {
  const mainItems = [
    { id: "dashboard", label: "Dashboard", icon: LayoutGrid },
    { id: "profiles", label: "Profiles", icon: FileText },
    { id: "monitor", label: "Monitor Editor", icon: Monitor },
    { id: "settings", label: "Settings", icon: Settings },
  ]

  const bottomItems = [
    { id: "help", label: "Help & Support", icon: HelpCircle },
    { id: "logout", label: "Quit", icon: LogOut },
  ]

  return (
    <div className="w-64 bg-sidebar border-r border-sidebar-border flex flex-col">
      {/* Logo Section */}
      <div className="px-6 py-8 border-b border-sidebar-border">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 bg-gradient-to-br from-cyan-400 to-blue-500 rounded-lg flex items-center justify-center shadow-lg">
            <span className="text-white font-bold text-lg">SF</span>
          </div>
          <div className="flex-1">
            <h1 className="text-lg font-bold text-sidebar-foreground">Smoothie</h1>
            <p className="text-xs text-sidebar-accent">Desktop</p>
          </div>
        </div>
      </div>

      {/* Main Navigation */}
      <nav className="flex-1 px-3 py-6 flex flex-col gap-2 overflow-y-auto">
        <div className="text-xs font-semibold text-sidebar-accent px-3 mb-2 uppercase tracking-wider">Main</div>
        {mainItems.map((item) => {
          const Icon = item.icon
          const isActive = currentView === item.id
          return (
            <motion.button
              key={item.id}
              onClick={() => setCurrentView(item.id as any)}
              className={cn(
                "w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200 text-left text-sm font-medium",
                isActive
                  ? "bg-sidebar-primary text-sidebar-primary-foreground shadow-lg"
                  : "text-sidebar-foreground hover:bg-sidebar-accent/20",
              )}
              whileHover={{ x: 4 }}
              whileTap={{ scale: 0.98 }}
            >
              <Icon className="w-5 h-5 flex-shrink-0" />
              <span>{item.label}</span>
              {isActive && <div className="ml-auto w-2 h-2 rounded-full bg-sidebar-primary-foreground" />}
            </motion.button>
          )
        })}
      </nav>

      {/* Bottom Actions */}
      <div className="px-3 py-4 border-t border-sidebar-border space-y-2">
        <div className="text-xs font-semibold text-sidebar-accent px-3 mb-2 uppercase tracking-wider">System</div>
        {bottomItems.map((item) => {
          const Icon = item.icon
          return (
            <motion.button
              key={item.id}
              className="w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200 text-left text-sm font-medium text-sidebar-foreground hover:bg-sidebar-accent/20"
              whileHover={{ x: 4 }}
              whileTap={{ scale: 0.98 }}
            >
              <Icon className="w-5 h-5 flex-shrink-0" />
              <span>{item.label}</span>
            </motion.button>
          )
        })}
      </div>
    </div>
  )
}
