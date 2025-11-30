"use client"

import { Button } from "@/components/ui/button"
import { LayoutGrid, Settings, Monitor, FileText } from "lucide-react"

interface NavbarProps {
  currentView: string
  setCurrentView: (view: "dashboard" | "profiles" | "monitor" | "settings") => void
}

export function Navbar({ currentView, setCurrentView }: NavbarProps) {
  const navItems = [
    { id: "dashboard", label: "Dashboard", icon: LayoutGrid },
    { id: "profiles", label: "Profiles", icon: FileText },
    { id: "monitor", label: "Monitor Editor", icon: Monitor },
    { id: "settings", label: "Settings", icon: Settings },
  ]

  return (
    <nav className="border-b border-border bg-card">
      <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-gradient-to-br from-blue-400 to-cyan-400 rounded-lg flex items-center justify-center">
            <span className="text-white font-bold text-sm">SF</span>
          </div>
          <h1 className="text-xl font-bold text-balance">Smoothie</h1>
        </div>

        <div className="flex items-center gap-2">
          {navItems.map((item) => {
            const Icon = item.icon
            const isActive = currentView === item.id
            return (
              <Button
                key={item.id}
                variant={isActive ? "default" : "ghost"}
                size="sm"
                onClick={() => setCurrentView(item.id as any)}
                className="gap-2"
              >
                <Icon className="w-4 h-4" />
                <span className="hidden sm:inline">{item.label}</span>
              </Button>
            )
          })}
        </div>
      </div>
    </nav>
  )
}
