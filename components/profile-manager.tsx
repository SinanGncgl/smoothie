"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Plus, Trash2, Save, Copy, Loader2 } from "lucide-react"
import { motion } from "framer-motion"
import { useProfiles } from "@/hooks/use-profiles"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Alert, AlertDescription } from "@/components/ui/alert"

interface ProfileManagerProps {
  onSelectProfile: (id: string) => void
}

export function ProfileManager({ onSelectProfile }: ProfileManagerProps) {
  const { 
    profiles, 
    loading, 
    error, 
    isBackendConnected,
    createProfile, 
    updateProfile, 
    deleteProfile, 
    duplicateProfile 
  } = useProfiles()
  
  const [editingId, setEditingId] = useState<string | null>(null)
  const [editingName, setEditingName] = useState("")
  const [newAppName, setNewAppName] = useState("")
  const [newTabUrl, setNewTabUrl] = useState("")
  const [newTabBrowser, setNewTabBrowser] = useState("Chrome")
  const [selectedProfileId, setSelectedProfileId] = useState<string | null>(null)
  
  // New profile dialog state
  const [dialogOpen, setDialogOpen] = useState(false)
  const [newProfileName, setNewProfileName] = useState("")
  const [newProfileDescription, setNewProfileDescription] = useState("")
  const [newProfileType, setNewProfileType] = useState<string>("Custom")
  const [isCreating, setIsCreating] = useState(false)

  const handleCreateProfile = async () => {
    if (!newProfileName.trim()) return
    setIsCreating(true)
    try {
      await createProfile({
        name: newProfileName,
        description: newProfileDescription,
        profile_type: newProfileType,
      })
      setNewProfileName("")
      setNewProfileDescription("")
      setNewProfileType("Custom")
      setDialogOpen(false)
    } catch (err) {
      console.error("Failed to create profile:", err)
    } finally {
      setIsCreating(false)
    }
  }

  const handleAddTab = (profileId: string) => {
    if (!newTabUrl.trim()) return
    // TODO: Implement with backend
    setNewTabUrl("")
  }

  const handleRemoveTab = (profileId: string, index: number) => {
    // TODO: Implement with backend
  }

  const handleAddApp = (profileId: string) => {
    if (!newAppName.trim()) return
    // TODO: Implement with backend
    setNewAppName("")
  }

  const handleRemoveApp = (profileId: string, appName: string) => {
    // TODO: Implement with backend
  }

  const handleSaveName = async (profileId: string) => {
    try {
      await updateProfile(profileId, editingName)
      setEditingId(null)
    } catch (err) {
      console.error("Failed to update profile name:", err)
    }
  }

  const handleDuplicateProfile = async (profileId: string) => {
    try {
      await duplicateProfile(profileId)
    } catch (err) {
      console.error("Failed to duplicate profile:", err)
    }
  }

  const handleDeleteProfile = async (profileId: string) => {
    try {
      await deleteProfile(profileId)
    } catch (err) {
      console.error("Failed to delete profile:", err)
    }
  }

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
    hidden: { opacity: 0, y: 12 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.3 },
    },
  }

  return (
    <div className="h-full flex flex-col p-8 gap-8">
      {/* Demo Mode Banner */}
      {!isBackendConnected && (
        <Alert>
          <AlertDescription>
            Running in demo mode. Profile changes are stored locally only.
          </AlertDescription>
        </Alert>
      )}

      {/* Header Section */}
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold text-foreground">Profile Library</h3>
          <p className="text-sm text-muted-foreground">Create and manage workspace configurations</p>
        </div>
        <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
          <DialogTrigger asChild>
            <Button className="gap-2">
              <Plus className="w-4 h-4" />
              New Profile
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create New Profile</DialogTitle>
              <DialogDescription>
                Set up a new workspace profile with your preferred configuration.
              </DialogDescription>
            </DialogHeader>
            <div className="grid gap-4 py-4">
              <div className="grid gap-2">
                <Label htmlFor="name">Name</Label>
                <Input
                  id="name"
                  placeholder="My Profile"
                  value={newProfileName}
                  onChange={(e) => setNewProfileName(e.target.value)}
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="description">Description</Label>
                <Input
                  id="description"
                  placeholder="Profile description..."
                  value={newProfileDescription}
                  onChange={(e) => setNewProfileDescription(e.target.value)}
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="type">Type</Label>
                <Select value={newProfileType} onValueChange={setNewProfileType}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="Work">Work</SelectItem>
                    <SelectItem value="Gaming">Gaming</SelectItem>
                    <SelectItem value="Research">Research</SelectItem>
                    <SelectItem value="Custom">Custom</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setDialogOpen(false)}>
                Cancel
              </Button>
              <Button onClick={handleCreateProfile} disabled={isCreating || !newProfileName.trim()}>
                {isCreating && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
                Create Profile
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      {/* Loading State */}
      {loading && (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center space-y-4">
            <Loader2 className="w-8 h-8 animate-spin mx-auto text-primary" />
            <p className="text-sm text-muted-foreground">Loading profiles...</p>
          </div>
        </div>
      )}

      {/* Profiles Grid */}
      {!loading && (
        <motion.div
          className="flex-1 overflow-y-auto space-y-4"
          variants={containerVariants}
          initial="hidden"
          animate="visible"
        >
          {profiles.length === 0 ? (
            <div className="flex-1 flex items-center justify-center py-12">
              <div className="text-center space-y-4">
                <p className="text-muted-foreground">No profiles yet. Create your first profile!</p>
                <Button onClick={() => setDialogOpen(true)} className="gap-2">
                  <Plus className="w-4 h-4" />
                  Create Profile
                </Button>
              </div>
            </div>
          ) : (
            profiles.map((profile) => (
              <motion.div key={profile.id} variants={itemVariants}>
                <Card
                  className={`cursor-pointer transition-all duration-300 overflow-hidden ${
                    selectedProfileId === profile.id
                      ? "border-primary/50 bg-primary/5 ring-1 ring-primary/30"
                      : "hover:border-border/80 hover:bg-card/50"
                  }`}
                  onClick={() => {
                    setSelectedProfileId(profile.id)
                    onSelectProfile(profile.id)
                  }}
                >
                  <CardHeader>
                    <div className="flex items-start justify-between gap-4">
                      <div className="flex-1 min-w-0">
                        {editingId === profile.id ? (
                          <div className="flex gap-2 items-center mb-2">
                            <Input
                              value={editingName}
                              onChange={(e) => setEditingName(e.target.value)}
                              placeholder="Profile name"
                              autoFocus
                              className="max-w-sm"
                            />
                            <Button size="sm" onClick={() => handleSaveName(profile.id)} variant="default">
                              <Save className="w-4 h-4" />
                            </Button>
                          </div>
                        ) : (
                          <div>
                            <div className="flex items-center gap-3 mb-1">
                              <CardTitle
                                className="cursor-pointer hover:text-primary transition-colors"
                                onClick={(e) => {
                                  e.stopPropagation()
                                  setEditingId(profile.id)
                                  setEditingName(profile.name)
                                }}
                              >
                                {profile.name}
                              </CardTitle>
                              <span className="text-xs bg-primary/10 text-primary px-2 py-1 rounded-full font-medium">
                                {profile.profile_type}
                              </span>
                              {profile.is_active && (
                                <span className="text-xs bg-green-500/10 text-green-500 px-2 py-1 rounded-full font-medium">
                                  Active
                                </span>
                              )}
                            </div>
                            <CardDescription>{profile.description}</CardDescription>
                          </div>
                        )}
                      </div>

                      {/* Action Buttons */}
                      <div className="flex items-center gap-2">
                        <Button
                          variant="ghost"
                          size="sm"
                          className="gap-1"
                          onClick={(e) => {
                            e.stopPropagation()
                            handleDuplicateProfile(profile.id)
                          }}
                        >
                          <Copy className="w-4 h-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="text-destructive hover:text-destructive"
                          onClick={(e) => {
                            e.stopPropagation()
                            handleDeleteProfile(profile.id)
                          }}
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </div>
                    </div>
                  </CardHeader>

                  <CardContent>
                    <div className="space-y-6">
                      {/* Quick Stats */}
                      <div className="grid grid-cols-3 gap-4">
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                            <span className="text-xs font-bold text-primary">{profile.monitors?.length || 0}</span>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">Monitors</p>
                            <p className="font-semibold text-sm">Configured</p>
                          </div>
                        </div>
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center">
                            <span className="text-xs font-bold text-accent">{profile.apps?.length || 0}</span>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">Applications</p>
                            <p className="font-semibold text-sm">Managed</p>
                          </div>
                        </div>
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 rounded-lg bg-chart-1/10 flex items-center justify-center">
                            <span className="text-xs font-bold text-chart-1">{profile.browser_tabs?.length || 0}</span>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">Browser Tabs</p>
                            <p className="font-semibold text-sm">Saved</p>
                          </div>
                        </div>
                      </div>

                      {/* Applications Section */}
                      <div className="border-t border-border pt-6">
                        <p className="text-sm font-semibold mb-3 text-foreground">Applications ({profile.apps?.length || 0})</p>
                        <div className="flex flex-wrap gap-2 mb-4">
                          {(profile.apps || []).map((app) => (
                            <motion.div
                              key={app.id || app.name}
                              initial={{ opacity: 0, scale: 0.8 }}
                              animate={{ opacity: 1, scale: 1 }}
                              className="bg-primary/10 text-primary px-3 py-1.5 rounded-lg text-xs font-medium flex items-center gap-2 hover:bg-primary/20 transition-colors"
                            >
                              {typeof app === 'string' ? app : app.name}
                              <button
                                onClick={(e) => {
                                  e.stopPropagation()
                                  handleRemoveApp(profile.id, typeof app === 'string' ? app : app.name)
                                }}
                                className="hover:text-destructive transition-colors ml-1"
                              >
                                ×
                              </button>
                            </motion.div>
                          ))}
                        </div>

                        <div className="flex gap-2">
                          <Input
                            placeholder="Add application..."
                            value={newAppName}
                            onChange={(e) => setNewAppName(e.target.value)}
                            onKeyDown={(e) => {
                              if (e.key === "Enter") handleAddApp(profile.id)
                            }}
                            className="flex-1 text-sm"
                            onClick={(e) => e.stopPropagation()}
                          />
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={(e) => {
                              e.stopPropagation()
                              handleAddApp(profile.id)
                            }}
                            className="gap-1"
                          >
                            <Plus className="w-3 h-3" />
                          </Button>
                        </div>
                      </div>

                      {/* Browser Tabs Section */}
                      <div className="border-t border-border pt-6">
                        <p className="text-sm font-semibold mb-3 text-foreground">Browser Tabs ({profile.browser_tabs?.length || 0})</p>
                        <div className="space-y-2 mb-4">
                          {(profile.browser_tabs || []).map((tab, idx) => (
                            <motion.div
                              key={tab.id || idx}
                              initial={{ opacity: 0, x: -10 }}
                              animate={{ opacity: 1, x: 0 }}
                              className="flex items-center justify-between gap-3 bg-card/50 p-2 rounded-lg text-sm"
                            >
                              <div className="flex items-center gap-2 flex-1 min-w-0">
                                <span className="text-xs bg-muted text-muted-foreground px-2 py-1 rounded font-mono">
                                  {tab.browser || "Browser"}
                                </span>
                                <span className="text-muted-foreground truncate">{tab.url}</span>
                              </div>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation()
                                  handleRemoveTab(profile.id, idx)
                                }}
                                className="text-destructive hover:text-destructive transition-colors"
                              >
                                ×
                              </button>
                            </motion.div>
                          ))}
                        </div>

                        <div className="flex gap-2">
                          <Input
                            placeholder="Enter tab URL..."
                            value={newTabUrl}
                            onChange={(e) => setNewTabUrl(e.target.value)}
                            onKeyDown={(e) => {
                              if (e.key === "Enter") handleAddTab(profile.id)
                            }}
                            className="flex-1 text-sm"
                            onClick={(e) => e.stopPropagation()}
                          />
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={(e) => {
                              e.stopPropagation()
                              handleAddTab(profile.id)
                            }}
                            className="gap-1"
                          >
                            <Plus className="w-3 h-3" />
                          </Button>
                        </div>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </motion.div>
            ))
          )}
        </motion.div>
      )}
    </div>
  )
}
