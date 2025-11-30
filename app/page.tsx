"use client"

import { useState } from "react"
import { Sidebar } from "@/components/sidebar"
import { TopBar } from "@/components/top-bar"
import { Dashboard } from "@/components/dashboard"
import { ProfileManager } from "@/components/profile-manager"
import { MonitorEditor } from "@/components/monitor-editor"
import { Settings } from "@/components/settings"

export default function Home() {
  const [currentView, setCurrentView] = useState<"dashboard" | "profiles" | "monitor" | "settings">("dashboard")
  const [currentProfile, setCurrentProfile] = useState<string | null>(null)

  return (
    <div className="flex h-screen bg-background text-foreground overflow-hidden">
      {/* Left Sidebar Navigation */}
      <Sidebar currentView={currentView} setCurrentView={setCurrentView} />

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col">
        {/* Top Header Bar */}
        <TopBar currentView={currentView} />

        {/* Content */}
        <main className="flex-1 overflow-auto">
          {currentView === "dashboard" && <Dashboard setCurrentView={setCurrentView} />}
          {currentView === "profiles" && <ProfileManager onSelectProfile={setCurrentProfile} />}
          {currentView === "monitor" && <MonitorEditor selectedProfile={currentProfile} />}
          {currentView === "settings" && <Settings />}
        </main>
      </div>
    </div>
  )
}
