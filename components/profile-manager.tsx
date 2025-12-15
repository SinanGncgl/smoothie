"use client";

import { useState, useEffect, useCallback } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Plus,
  Trash2,
  Save,
  Copy,
  Loader2,
  Play,
  Globe,
  AppWindow,
  X,
  Camera,
  Crown,
} from "lucide-react";
import { motion } from "framer-motion";
import { useProfiles } from "@/hooks/use-profiles";
import { useSystemDetection } from "@/hooks/use-system-detection";
import { useToast } from "@/hooks/use-toast";
import { useSubscription } from "@/contexts/subscription-context";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Alert, AlertDescription } from "@/components/ui/alert";
import {
  appApi,
  browserTabApi,
  profileApi,
  isTauri,
  type App,
  type BrowserTab,
  type RunningApp,
  type InstalledApp,
} from "@/lib/tauri";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Checkbox } from "@/components/ui/checkbox";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface ProfileManagerProps {
  onSelectProfile: (id: string) => void;
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
    duplicateProfile,
    fetchProfiles,
    captureMonitorsToProfile,
  } = useProfiles();

  const {
    runningApps = [],
    installedApps = [],
    refreshRunningApps,
    refreshInstalledApps,
  } = useSystemDetection();
  const { toast } = useToast();
  const { canCreateProfile, isPro, tier } = useSubscription();

  // OSS version: unlimited profiles
  const canCreateMore = true;
  const maxProfiles = Infinity;

  const [editingId, setEditingId] = useState<string | null>(null);
  const [editingName, setEditingName] = useState("");
  const [newTabUrl, setNewTabUrl] = useState("");
  const [newTabBrowser, setNewTabBrowser] = useState("Chrome");
  const [selectedProfileId, setSelectedProfileId] = useState<string | null>(
    null,
  );

  // Profile apps and tabs state
  const [profileApps, setProfileApps] = useState<Record<string, App[]>>({});
  const [profileTabs, setProfileTabs] = useState<Record<string, BrowserTab[]>>(
    {},
  );

  // App picker dialog state
  const [appPickerOpen, setAppPickerOpen] = useState(false);
  const [appPickerProfileId, setAppPickerProfileId] = useState<string | null>(
    null,
  );
  const [selectedApps, setSelectedApps] = useState<Set<string>>(new Set());
  const [appPickerTab, setAppPickerTab] = useState<"running" | "installed">(
    "running",
  );
  const [appSearchQuery, setAppSearchQuery] = useState("");

  // Starting profile state
  const [startingProfileId, setStartingProfileId] = useState<string | null>(
    null,
  );

  // New profile dialog state
  const [dialogOpen, setDialogOpen] = useState(false);
  const [newProfileName, setNewProfileName] = useState("");
  const [newProfileDescription, setNewProfileDescription] = useState("");
  const [newProfileType, setNewProfileType] = useState<string>("Custom");
  const [captureMonitorsOnCreate, setCaptureMonitorsOnCreate] = useState(true);
  const [isCreating, setIsCreating] = useState(false);
  const [capturingProfileId, setCapturingProfileId] = useState<string | null>(
    null,
  );

  // Load apps and tabs for all profiles
  const loadProfileData = useCallback(async (profileId: string) => {
    if (!isTauri()) return;
    try {
      const [apps, tabs] = await Promise.all([
        appApi.getApps(profileId),
        browserTabApi.getBrowserTabs(profileId),
      ]);
      setProfileApps((prev) => ({ ...prev, [profileId]: apps }));
      setProfileTabs((prev) => ({ ...prev, [profileId]: tabs }));
    } catch {
      // Silent failure for profile data loading
    }
  }, []);

  // Load data for all profiles on mount
  useEffect(() => {
    profiles.forEach((p) => loadProfileData(p.id));
  }, [profiles, loadProfileData]);

  const handleCreateProfile = async () => {
    if (!newProfileName.trim()) return;
    setIsCreating(true);
    try {
      await createProfile(
        {
          name: newProfileName,
          description: newProfileDescription,
          profileType: newProfileType,
        },
        captureMonitorsOnCreate,
      );
      setNewProfileName("");
      setNewProfileDescription("");
      setNewProfileType("Custom");
      setCaptureMonitorsOnCreate(true);
      setDialogOpen(false);
      toast({
        title: "Profile created",
        description: captureMonitorsOnCreate
          ? "Profile created with current monitor layout"
          : "Profile created successfully",
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to create profile",
        variant: "destructive",
      });
    } finally {
      setIsCreating(false);
    }
  };

  const handleCaptureMonitors = async (profileId: string) => {
    setCapturingProfileId(profileId);
    try {
      await captureMonitorsToProfile(profileId);
      toast({
        title: "Monitors captured",
        description: "Current monitor layout saved to profile",
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to capture monitors",
        variant: "destructive",
      });
    } finally {
      setCapturingProfileId(null);
    }
  };

  const handleAddTab = async (profileId: string) => {
    if (!newTabUrl.trim() || !isTauri()) return;
    try {
      const existingTabs = profileTabs[profileId] || [];
      const tabOrder = existingTabs.length;

      await browserTabApi.createBrowserTab({
        profileId,
        url: newTabUrl,
        browser: newTabBrowser,
        tabOrder,
      });
      setNewTabUrl("");
      await loadProfileData(profileId);
      await fetchProfiles();
      toast({
        title: "Tab added",
        description: `Added ${newTabUrl} to profile`,
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to add tab",
        variant: "destructive",
      });
    }
  };

  const handleRemoveTab = async (profileId: string, tabId: string) => {
    if (!isTauri()) return;
    try {
      await browserTabApi.deleteBrowserTab(tabId);
      await loadProfileData(profileId);
      await fetchProfiles();
      toast({
        title: "Tab removed",
        description: "Browser tab removed from profile",
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to remove tab",
        variant: "destructive",
      });
    }
  };

  const openAppPicker = (profileId: string) => {
    setAppPickerProfileId(profileId);
    // Pre-select already added apps
    const existingApps = profileApps[profileId] || [];
    setSelectedApps(new Set(existingApps.map((a) => a.bundleId)));
    setAppSearchQuery("");
    setAppPickerTab("running");
    refreshRunningApps();
    refreshInstalledApps();
    setAppPickerOpen(true);
  };

  const handleAddSelectedApps = async () => {
    if (!appPickerProfileId || !isTauri()) return;
    try {
      const existingApps = profileApps[appPickerProfileId] || [];
      const existingBundleIds = new Set(existingApps.map((a) => a.bundleId));

      // Combine running and installed apps to find the selected ones
      const allApps = [
        ...runningApps.map((app) => ({
          name: app.name,
          bundleId: app.bundleId,
          path: app.path,
        })),
        ...installedApps.map((app) => ({
          name: app.name,
          bundleId: app.bundleId,
          path: app.path,
        })),
      ];

      // Add new apps that aren't already in the profile
      for (const app of allApps) {
        if (
          selectedApps.has(app.bundleId) &&
          !existingBundleIds.has(app.bundleId)
        ) {
          await appApi.createApp({
            profileId: appPickerProfileId,
            name: app.name,
            bundleId: app.bundleId,
            exePath: app.path,
            launchOnActivate: true,
            startupDelayMs: 0,
            orderIndex: existingBundleIds.size,
          });
          // Add to existing set to avoid duplicates
          existingBundleIds.add(app.bundleId);
        }
      }

      setAppPickerOpen(false);
      await loadProfileData(appPickerProfileId);
      await fetchProfiles();
      toast({
        title: "Apps added",
        description: "Applications added to profile",
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to add apps",
        variant: "destructive",
      });
    }
  };

  const handleRemoveApp = async (profileId: string, appId: string) => {
    if (!isTauri()) return;
    try {
      await appApi.deleteApp(appId);
      await loadProfileData(profileId);
      await fetchProfiles();
      toast({
        title: "App removed",
        description: "Application removed from profile",
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to remove app",
        variant: "destructive",
      });
    }
  };

  const handleStartProfile = async (profileId: string) => {
    if (!isTauri()) return;
    setStartingProfileId(profileId);
    try {
      const result = await profileApi.startProfile(profileId);
      const successApps = result.appsLaunched.filter((a) => a.success).length;
      const successTabs = result.tabsOpened.filter((t) => t.success).length;
      const monitorStatus = result.monitorLayout.applied
        ? `${result.monitorLayout.monitorCount} monitors arranged`
        : result.monitorLayout.monitorCount > 0
          ? result.monitorLayout.message
          : "No monitor layout";
      toast({
        title: "Profile Started",
        description: `${monitorStatus}, launched ${successApps} apps, opened ${successTabs} tabs`,
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to start profile",
        variant: "destructive",
      });
    } finally {
      setStartingProfileId(null);
    }
  };

  const handleSaveName = async (profileId: string) => {
    try {
      await updateProfile(profileId, { name: editingName });
      setEditingId(null);
    } catch {
      // Error handled by updateProfile
    }
  };

  const handleDuplicateProfile = async (profileId: string) => {
    try {
      await duplicateProfile(profileId);
    } catch {
      // Error handled by duplicateProfile
    }
  };

  const handleDeleteProfile = async (profileId: string) => {
    try {
      await deleteProfile(profileId);
    } catch {
      // Error handled by deleteProfile
    }
  };

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.08,
        delayChildren: 0.1,
      },
    },
  };

  const itemVariants = {
    hidden: { opacity: 0, y: 12 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.3 },
    },
  };

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
          <h3 className="text-lg font-semibold text-foreground">
            Profile Library
          </h3>
          <p className="text-sm text-muted-foreground">
            Create and manage workspace configurations
          </p>
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
                Set up a new workspace profile with your preferred
                configuration.
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
                <Select
                  value={newProfileType}
                  onValueChange={setNewProfileType}
                >
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
              <div className="flex items-center space-x-3 pt-2">
                <Checkbox
                  id="captureMonitors"
                  checked={captureMonitorsOnCreate}
                  onCheckedChange={(checked) =>
                    setCaptureMonitorsOnCreate(checked === true)
                  }
                />
                <div className="flex flex-col">
                  <Label
                    htmlFor="captureMonitors"
                    className="text-sm font-medium cursor-pointer flex items-center gap-2"
                  >
                    <Camera className="w-4 h-4" />
                    Capture current monitor layout
                  </Label>
                  <p className="text-xs text-muted-foreground">
                    Save your current monitor arrangement to this profile
                  </p>
                </div>
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setDialogOpen(false)}>
                Cancel
              </Button>
              <Button
                onClick={handleCreateProfile}
                disabled={isCreating || !newProfileName.trim()}
              >
                {isCreating && (
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                )}
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
                <p className="text-muted-foreground">
                  No profiles yet. Create your first profile!
                </p>
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
                    setSelectedProfileId(profile.id);
                    onSelectProfile(profile.id);
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
                            <Button
                              size="sm"
                              onClick={() => handleSaveName(profile.id)}
                              variant="default"
                            >
                              <Save className="w-4 h-4" />
                            </Button>
                          </div>
                        ) : (
                          <div>
                            <div className="flex items-center gap-3 mb-1">
                              <CardTitle
                                className="cursor-pointer hover:text-primary transition-colors"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  setEditingId(profile.id);
                                  setEditingName(profile.name);
                                }}
                              >
                                {profile.name}
                              </CardTitle>
                              <span className="text-xs bg-primary/10 text-primary px-2 py-1 rounded-full font-medium">
                                {profile.profileType}
                              </span>
                              {profile.isActive && (
                                <span className="text-xs bg-green-500/10 text-green-500 px-2 py-1 rounded-full font-medium">
                                  Active
                                </span>
                              )}
                            </div>
                            <CardDescription>
                              {profile.description}
                            </CardDescription>
                          </div>
                        )}
                      </div>

                      {/* Action Buttons */}
                      <div className="flex items-center gap-2">
                        <Button
                          variant="default"
                          size="sm"
                          className="gap-1"
                          disabled={startingProfileId === profile.id}
                          onClick={(e) => {
                            e.stopPropagation();
                            handleStartProfile(profile.id);
                          }}
                        >
                          {startingProfileId === profile.id ? (
                            <Loader2 className="w-4 h-4 animate-spin" />
                          ) : (
                            <Play className="w-4 h-4" />
                          )}
                          Start
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          className="gap-1"
                          disabled={capturingProfileId === profile.id}
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCaptureMonitors(profile.id);
                          }}
                          title="Capture current monitor layout"
                        >
                          {capturingProfileId === profile.id ? (
                            <Loader2 className="w-4 h-4 animate-spin" />
                          ) : (
                            <Camera className="w-4 h-4" />
                          )}
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="gap-1"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleDuplicateProfile(profile.id);
                          }}
                        >
                          <Copy className="w-4 h-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="text-destructive hover:text-destructive"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleDeleteProfile(profile.id);
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
                            <span className="text-xs font-bold text-primary">
                              {profile.monitorCount || 0}
                            </span>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">
                              Monitors
                            </p>
                            <p className="font-semibold text-sm">Configured</p>
                          </div>
                        </div>
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center">
                            <span className="text-xs font-bold text-accent">
                              {profile.appCount || 0}
                            </span>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">
                              Applications
                            </p>
                            <p className="font-semibold text-sm">Managed</p>
                          </div>
                        </div>
                        <div className="flex items-center gap-3">
                          <div className="w-10 h-10 rounded-lg bg-chart-1/10 flex items-center justify-center">
                            <span className="text-xs font-bold text-chart-1">
                              {profile.browserTabCount || 0}
                            </span>
                          </div>
                          <div>
                            <p className="text-xs text-muted-foreground">
                              Browser Tabs
                            </p>
                            <p className="font-semibold text-sm">Saved</p>
                          </div>
                        </div>
                      </div>

                      {/* Applications Section */}
                      <div className="border-t border-border pt-6">
                        <div className="flex items-center justify-between mb-3">
                          <p className="text-sm font-semibold text-foreground flex items-center gap-2">
                            <AppWindow className="w-4 h-4" />
                            Applications (
                            {(profileApps[profile.id] || []).length})
                          </p>
                        </div>
                        {(profileApps[profile.id] || []).length > 0 ? (
                          <div className="flex flex-wrap gap-2 mb-4">
                            {(profileApps[profile.id] || []).map((app) => (
                              <motion.div
                                key={app.id}
                                initial={{ opacity: 0, scale: 0.8 }}
                                animate={{ opacity: 1, scale: 1 }}
                                className="bg-primary/10 text-primary px-3 py-1.5 rounded-lg text-xs font-medium flex items-center gap-2 hover:bg-primary/20 transition-colors"
                              >
                                {app.name}
                                <button
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    handleRemoveApp(profile.id, app.id);
                                  }}
                                  className="hover:text-destructive transition-colors ml-1"
                                >
                                  <X className="w-3 h-3" />
                                </button>
                              </motion.div>
                            ))}
                          </div>
                        ) : (
                          <p className="text-xs text-muted-foreground mb-4">
                            No applications configured
                          </p>
                        )}
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={(e) => {
                            e.stopPropagation();
                            openAppPicker(profile.id);
                          }}
                          className="gap-1"
                        >
                          <Plus className="w-3 h-3" />
                          Add Applications
                        </Button>
                      </div>

                      {/* Browser Tabs Section */}
                      <div className="border-t border-border pt-6">
                        <p className="text-sm font-semibold mb-3 text-foreground flex items-center gap-2">
                          <Globe className="w-4 h-4" />
                          Browser Tabs ({(profileTabs[profile.id] || []).length}
                          )
                        </p>
                        {(profileTabs[profile.id] || []).length > 0 ? (
                          <div className="space-y-2 mb-4">
                            {(profileTabs[profile.id] || []).map((tab) => (
                              <motion.div
                                key={tab.id}
                                initial={{ opacity: 0, x: -10 }}
                                animate={{ opacity: 1, x: 0 }}
                                className="flex items-center justify-between gap-3 bg-card/50 p-2 rounded-lg text-sm"
                              >
                                <div className="flex items-center gap-2 flex-1 min-w-0">
                                  <span className="text-xs bg-muted text-muted-foreground px-2 py-1 rounded font-mono">
                                    {tab.browser}
                                  </span>
                                  <span className="text-muted-foreground truncate">
                                    {tab.url}
                                  </span>
                                </div>
                                <button
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    handleRemoveTab(profile.id, tab.id);
                                  }}
                                  className="text-destructive hover:text-destructive transition-colors"
                                >
                                  <X className="w-3 h-3" />
                                </button>
                              </motion.div>
                            ))}
                          </div>
                        ) : (
                          <p className="text-xs text-muted-foreground mb-4">
                            No browser tabs configured
                          </p>
                        )}
                        <div className="flex gap-2">
                          <Input
                            placeholder="Enter tab URL..."
                            value={newTabUrl}
                            onChange={(e) => setNewTabUrl(e.target.value)}
                            onKeyDown={(e) => {
                              if (e.key === "Enter") handleAddTab(profile.id);
                            }}
                            className="flex-1 text-sm"
                            onClick={(e) => e.stopPropagation()}
                          />
                          <Select
                            value={newTabBrowser}
                            onValueChange={setNewTabBrowser}
                          >
                            <SelectTrigger
                              className="w-28"
                              onClick={(e) => e.stopPropagation()}
                            >
                              <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                              <SelectItem value="Chrome">Chrome</SelectItem>
                              <SelectItem value="Safari">Safari</SelectItem>
                              <SelectItem value="Firefox">Firefox</SelectItem>
                              <SelectItem value="Arc">Arc</SelectItem>
                              <SelectItem value="Brave">Brave</SelectItem>
                              <SelectItem value="Edge">Edge</SelectItem>
                            </SelectContent>
                          </Select>
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleAddTab(profile.id);
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

      {/* App Picker Dialog */}
      <Dialog open={appPickerOpen} onOpenChange={setAppPickerOpen}>
        <DialogContent className="max-w-lg">
          <DialogHeader>
            <DialogTitle>Add Applications</DialogTitle>
            <DialogDescription>
              Select applications to add to this profile. You can choose from
              running apps or browse all installed apps.
            </DialogDescription>
          </DialogHeader>

          {/* Search Input */}
          <div className="mb-2">
            <Input
              placeholder="Search applications..."
              value={appSearchQuery}
              onChange={(e) => setAppSearchQuery(e.target.value)}
              className="w-full"
            />
          </div>

          <Tabs
            value={appPickerTab}
            onValueChange={(v) => setAppPickerTab(v as "running" | "installed")}
          >
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="running">
                Running ({runningApps.length})
              </TabsTrigger>
              <TabsTrigger value="installed">
                All Apps ({installedApps.length})
              </TabsTrigger>
            </TabsList>

            <TabsContent value="running" className="mt-2">
              <ScrollArea className="h-[300px] pr-4">
                <div className="space-y-2">
                  {runningApps.length === 0 ? (
                    <p className="text-sm text-muted-foreground text-center py-4">
                      No running applications detected
                    </p>
                  ) : (
                    runningApps
                      .filter(
                        (app) =>
                          appSearchQuery === "" ||
                          app.name
                            .toLowerCase()
                            .includes(appSearchQuery.toLowerCase()) ||
                          app.bundleId
                            .toLowerCase()
                            .includes(appSearchQuery.toLowerCase()),
                      )
                      .map((app) => (
                        <div
                          key={app.bundleId}
                          className="flex items-center space-x-3 p-2 rounded-lg hover:bg-muted/50 cursor-pointer"
                          onClick={() => {
                            const newSelected = new Set(selectedApps);
                            if (newSelected.has(app.bundleId)) {
                              newSelected.delete(app.bundleId);
                            } else {
                              newSelected.add(app.bundleId);
                            }
                            setSelectedApps(newSelected);
                          }}
                        >
                          <Checkbox
                            checked={selectedApps.has(app.bundleId)}
                            onCheckedChange={(checked) => {
                              const newSelected = new Set(selectedApps);
                              if (checked) {
                                newSelected.add(app.bundleId);
                              } else {
                                newSelected.delete(app.bundleId);
                              }
                              setSelectedApps(newSelected);
                            }}
                          />
                          <div className="flex-1 min-w-0">
                            <p className="text-sm font-medium truncate">
                              {app.name}
                            </p>
                            <p className="text-xs text-muted-foreground truncate">
                              {app.bundleId}
                            </p>
                          </div>
                          {app.windowCount > 0 && (
                            <span className="text-xs text-muted-foreground bg-muted px-2 py-0.5 rounded">
                              {app.windowCount} window
                              {app.windowCount !== 1 ? "s" : ""}
                            </span>
                          )}
                        </div>
                      ))
                  )}
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent value="installed" className="mt-2">
              <ScrollArea className="h-[300px] pr-4">
                <div className="space-y-2">
                  {installedApps.length === 0 ? (
                    <p className="text-sm text-muted-foreground text-center py-4">
                      Loading installed applications...
                    </p>
                  ) : (
                    installedApps
                      .filter(
                        (app) =>
                          appSearchQuery === "" ||
                          app.name
                            .toLowerCase()
                            .includes(appSearchQuery.toLowerCase()) ||
                          app.bundleId
                            .toLowerCase()
                            .includes(appSearchQuery.toLowerCase()),
                      )
                      .map((app) => (
                        <div
                          key={app.bundleId}
                          className="flex items-center space-x-3 p-2 rounded-lg hover:bg-muted/50 cursor-pointer"
                          onClick={() => {
                            const newSelected = new Set(selectedApps);
                            if (newSelected.has(app.bundleId)) {
                              newSelected.delete(app.bundleId);
                            } else {
                              newSelected.add(app.bundleId);
                            }
                            setSelectedApps(newSelected);
                          }}
                        >
                          <Checkbox
                            checked={selectedApps.has(app.bundleId)}
                            onCheckedChange={(checked) => {
                              const newSelected = new Set(selectedApps);
                              if (checked) {
                                newSelected.add(app.bundleId);
                              } else {
                                newSelected.delete(app.bundleId);
                              }
                              setSelectedApps(newSelected);
                            }}
                          />
                          <div className="flex-1 min-w-0">
                            <p className="text-sm font-medium truncate">
                              {app.name}
                            </p>
                            <p className="text-xs text-muted-foreground truncate">
                              {app.bundleId}
                            </p>
                          </div>
                          {app.version && (
                            <span className="text-xs text-muted-foreground bg-muted px-2 py-0.5 rounded">
                              v{app.version}
                            </span>
                          )}
                        </div>
                      ))
                  )}
                </div>
              </ScrollArea>
            </TabsContent>
          </Tabs>

          <DialogFooter>
            <Button variant="outline" onClick={() => setAppPickerOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleAddSelectedApps}
              disabled={selectedApps.size === 0}
            >
              Add Selected ({selectedApps.size})
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
