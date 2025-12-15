"use client"

import { useState } from "react"

import { AuthGuard } from "@/components/auth-guard"
import { Dashboard } from "@/components/dashboard"
import { HelpSupport } from "@/components/help-support"
import { MonitorEditor } from "@/components/monitor-editor"
import { ProfileManager } from "@/components/profile-manager"
import { Settings } from "@/components/settings"
import { Sidebar } from "@/components/sidebar"
import { SystemCapture } from "@/components/system-capture"
import { TopBar } from "@/components/top-bar"

export default function Home() {
  const [currentView, setCurrentView] = useState<"dashboard" | "profiles" | "monitor" | "capture" | "settings" | "help">("dashboard")
  const [currentProfile, setCurrentProfile] = useState<string | null>(null)

  return (
    <AuthGuard>
      <div className="flex h-screen bg-background text-foreground overflow-hidden">
        {/* Left Sidebar Navigation */}
        <Sidebar currentView={currentView} setCurrentView={setCurrentView} />

        {/* Main Content Area */}
        <div className="flex-1 flex flex-col">
          {/* Top Header Bar */}
          <TopBar 
            currentView={currentView} 
            onNavigateToProfiles={() => setCurrentView("profiles")} 
            onNavigateToSettings={() => setCurrentView("settings")}
          />

          {/* Content */}
          <main className="flex-1 overflow-auto">
            {currentView === "dashboard" && <Dashboard setCurrentView={setCurrentView} />}
            {currentView === "profiles" && <ProfileManager onSelectProfile={setCurrentProfile} />}
            {currentView === "monitor" && <MonitorEditor selectedProfile={currentProfile} onSelectProfile={setCurrentProfile} />}
            {currentView === "capture" && <div className="p-8"><SystemCapture /></div>}
            {currentView === "settings" && <Settings />}
            {currentView === "help" && <HelpSupport />}
          </main>
        </div>
      </div>
    </AuthGuard>
  )
}
