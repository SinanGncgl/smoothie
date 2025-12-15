"use client"

import { useState } from "react"
import { AuthGuard } from "@/components/auth-guard"
import { Sidebar } from "@/components/sidebar"
import { TopBar } from "@/components/top-bar"
import { Dashboard } from "@/components/dashboard"
import { ProfileManager } from "@/components/profile-manager"
import { MonitorEditor } from "@/components/monitor-editor"
import { SystemCapture } from "@/components/system-capture"
import { Settings } from "@/components/settings"
import { HelpSupport } from "@/components/help-support"

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
