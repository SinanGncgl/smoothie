"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Switch } from "@/components/ui/switch"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Cloud, Zap, Clock, Shield, Download, Upload, RotateCcw, Info } from "lucide-react"
import { motion } from "framer-motion"

export function Settings() {
  const [settings, setSettings] = useState({
    cloudSync: true,
    autoRestore: true,
    monitorDetection: true,
    animationsEnabled: true,
    autoActivateTime: "never",
    keyboardShortcut: "Cmd+Shift+1",
    theme: "dark",
  })

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.08,
        delayChildren: 0.1,
      },
    },
  }

  const itemVariants = {
    hidden: { opacity: 0, x: -12 },
    visible: {
      opacity: 1,
      x: 0,
      transition: { duration: 0.3 },
    },
  }

  const settingGroups = [
    {
      title: "General Settings",
      description: "Core functionality and behavior",
      items: [
        {
          id: "autoRestore",
          icon: Zap,
          title: "Auto-Restore on Startup",
          description: "Automatically restore last used profile on launch",
          type: "toggle",
        },
        {
          id: "monitorDetection",
          icon: Shield,
          title: "Monitor Change Detection",
          description: "Adapt layout when monitors are connected/disconnected",
          type: "toggle",
        },
        {
          id: "animationsEnabled",
          icon: Clock,
          title: "Enable Animations",
          description: "Smooth transitions and animations throughout the app",
          type: "toggle",
        },
      ],
    },
    {
      title: "Automation Rules",
      description: "Set up automatic profile activation",
      items: [
        {
          id: "autoActivateTime",
          icon: Clock,
          title: "Auto-Activate Profile",
          description: "Activate a profile at specific times",
          type: "select",
        },
        {
          id: "keyboardShortcut",
          icon: Zap,
          title: "Keyboard Shortcut",
          description: "Quick profile activation",
          type: "text",
        },
      ],
    },
    {
      title: "Cloud Sync",
      description: "Manage your profile data across devices",
      items: [
        {
          id: "cloudSync",
          icon: Cloud,
          title: "Enable Cloud Sync",
          description: "Sync profiles across all your devices",
          type: "toggle",
        },
      ],
    },
  ]

  return (
    <div className="h-full flex flex-col p-8 gap-8 overflow-y-auto">
      {/* Header */}
      <motion.div initial={{ opacity: 0, y: -10 }} animate={{ opacity: 1, y: 0 }}>
        <h3 className="text-lg font-semibold text-foreground mb-1">Application Settings</h3>
        <p className="text-sm text-muted-foreground">Customize Smoothie behavior and preferences</p>
      </motion.div>

      {/* Settings Sections */}
      <motion.div className="space-y-6 flex-1" variants={containerVariants} initial="hidden" animate="visible">
        {/* General Settings */}
        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle className="text-base">General Settings</CardTitle>
              <CardDescription>Core functionality and behavior</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              {settingGroups[0].items.map((item) => {
                const Icon = item.icon
                return (
                  <div
                    key={item.id}
                    className="flex items-center justify-between py-3 border-b border-border last:border-0"
                  >
                    <div className="flex items-start gap-4">
                      <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center mt-1 flex-shrink-0">
                        <Icon className="w-5 h-5 text-primary" />
                      </div>
                      <div>
                        <p className="font-medium text-foreground">{item.title}</p>
                        <p className="text-sm text-muted-foreground">{item.description}</p>
                      </div>
                    </div>
                    <Switch
                      checked={settings[item.id as keyof typeof settings] as boolean}
                      onCheckedChange={(checked) =>
                        setSettings({
                          ...settings,
                          [item.id]: checked,
                        })
                      }
                    />
                  </div>
                )
              })}
            </CardContent>
          </Card>
        </motion.div>

        {/* Automation */}
        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Automation Rules</CardTitle>
              <CardDescription>Set up automatic profile activation</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="space-y-2">
                <label className="block text-sm font-medium text-foreground">Auto-Activate Profile</label>
                <p className="text-xs text-muted-foreground mb-3">
                  Automatically switch to a profile at specific times
                </p>
                <Select
                  value={settings.autoActivateTime}
                  onValueChange={(value) => setSettings({ ...settings, autoActivateTime: value })}
                >
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select option" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="never">Never</SelectItem>
                    <SelectItem value="morning">Morning (9:00 AM) → Work</SelectItem>
                    <SelectItem value="afternoon">Afternoon (12:00 PM) → Break</SelectItem>
                    <SelectItem value="evening">Evening (6:00 PM) → Personal</SelectItem>
                    <SelectItem value="custom">Custom Time...</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2 border-t border-border pt-6">
                <label className="block text-sm font-medium text-foreground">Keyboard Shortcut</label>
                <p className="text-xs text-muted-foreground mb-3">Quick access to profile activation</p>
                <div className="flex gap-2">
                  <div className="flex-1 font-mono bg-muted border border-border px-4 py-2 rounded-lg text-sm text-foreground flex items-center">
                    {settings.keyboardShortcut}
                  </div>
                  <Button variant="outline">Record</Button>
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        {/* Cloud Sync */}
        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Cloud Sync & Backup</CardTitle>
              <CardDescription>Manage your profile data</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between py-4 px-4 bg-primary/5 border border-primary/20 rounded-lg">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                    <Cloud className="w-5 h-5 text-primary" />
                  </div>
                  <div>
                    <p className="font-medium text-foreground text-sm">Cloud Sync Status</p>
                    <p className="text-xs text-muted-foreground">Last synced: 2 minutes ago</p>
                  </div>
                </div>
                <Switch checked={settings.cloudSync} />
              </div>

              <div className="grid grid-cols-2 gap-3">
                <Button variant="outline" className="gap-2 bg-transparent">
                  <Download className="w-4 h-4" />
                  Download Backup
                </Button>
                <Button variant="outline" className="gap-2 bg-transparent">
                  <Upload className="w-4 h-4" />
                  Upload Backup
                </Button>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        {/* Preferences */}
        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Preferences</CardTitle>
              <CardDescription>Customize your experience</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <label className="block text-sm font-medium text-foreground">Theme</label>
                <Select value={settings.theme} onValueChange={(value) => setSettings({ ...settings, theme: value })}>
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select theme" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="dark">Dark</SelectItem>
                    <SelectItem value="light">Light</SelectItem>
                    <SelectItem value="system">System</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        {/* About & Advanced */}
        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle className="text-base flex items-center gap-2">
                <Info className="w-4 h-4" />
                About & Advanced
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2 pb-4 border-b border-border">
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Version</span>
                  <span className="text-sm font-semibold">1.0.0</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Last Updated</span>
                  <span className="text-sm font-semibold">Nov 30, 2025</span>
                </div>
              </div>

              <div className="flex gap-2">
                <Button variant="outline" className="flex-1 gap-2 bg-transparent">
                  <RotateCcw className="w-4 h-4" />
                  Reset to Defaults
                </Button>
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </motion.div>
    </div>
  )
}
