"use client";

import { motion } from "framer-motion";
import { Zap, Clock, Shield, RotateCcw, Info, Loader2 } from "lucide-react";
import { useTheme } from "next-themes";
import { useState, useEffect, useCallback } from "react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { useToast } from "@/hooks/use-toast";
import {
  userApi,
  isTauri,
  type UserSettings,
  profileApi,
  type Profile,
} from "@/lib/tauri";


const DEFAULT_SETTINGS: UserSettings = {
  id: "",
  userId: "",
  theme: "dark",
  autoRestore: true,
  monitorDetection: true,
  animationsEnabled: true,
  autoActivateTime: "never",
  keyboardShortcut: "Cmd+Shift+1",
  notificationsEnabled: true,
  onboardingCompleted: false,
  onboardingStep: 0,
  createdAt: "",
  updatedAt: "",
};

export function Settings() {
  const [settings, setSettings] = useState<UserSettings>(DEFAULT_SETTINGS);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);
  const [isRecordingShortcut, setIsRecordingShortcut] = useState(false);
  const [recordedKeys, setRecordedKeys] = useState<string[]>([]);
  const [customTime, setCustomTime] = useState("09:00");
  const { toast } = useToast();
  const { setTheme } = useTheme();

  // Load settings from database
  const loadSettings = useCallback(async () => {
    if (!isTauri()) {
      setIsLoading(false);
      return;
    }

    try {
      const [settingsData, profilesData] = await Promise.all([
        userApi.getSettings(),
        profileApi.getProfiles(),
      ]);

      // Handle custom time parsing
      let processedSettings = settingsData;
      if (settingsData.autoActivateTime?.startsWith("custom:")) {
        const time = settingsData.autoActivateTime
          .split(":")
          .slice(1)
          .join(":");
        setCustomTime(time);
        processedSettings = { ...settingsData, autoActivateTime: "custom" };
      }

      setSettings(processedSettings);
      setProfiles(profilesData);
    } catch (error) {
      toast({
        title: "Error",
        description: "Failed to load settings from database",
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  }, [toast]);

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  // Save a single setting
  const updateSetting = async <K extends keyof UserSettings>(
    key: K,
    value: UserSettings[K],
  ) => {
    // Update local state immediately
    setSettings((prev) => ({ ...prev, [key]: value }));

    // Apply theme immediately if it's a theme change
    if (key === "theme") {
      setTheme(value as string);
    }

    if (!isTauri()) return;

    setIsSaving(true);
    try {
      const updated = await userApi.updateSettings(undefined, { [key]: value });
      setSettings(updated);
    } catch (error) {
      toast({
        title: "Error",
        description: `Failed to save ${key}`,
        variant: "destructive",
      });
      // Revert on error
      loadSettings();
    } finally {
      setIsSaving(false);
    }
  };

  // Keyboard shortcut recording
  useEffect(() => {
    if (!isRecordingShortcut) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const keys: string[] = [];
      if (e.ctrlKey || e.metaKey) keys.push(e.metaKey ? "Cmd" : "Ctrl");
      if (e.altKey) keys.push("Alt");
      if (e.shiftKey) keys.push("Shift");
      if (!["Control", "Alt", "Shift", "Meta"].includes(e.key)) {
        keys.push(e.key.toUpperCase());
      }

      if (keys.length > 0) {
        const shortcut = keys.join("+");
        setRecordedKeys(keys);

        // Auto-save after a short delay
        setTimeout(() => {
          updateSetting("keyboardShortcut", shortcut);
          setIsRecordingShortcut(false);
          setRecordedKeys([]);
          toast({
            title: "Shortcut Recorded",
            description: `New shortcut: ${shortcut}`,
          });
        }, 500);
      }
    };

    const handleKeyUp = () => {
      if (recordedKeys.length === 0) {
        setIsRecordingShortcut(false);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  }, [isRecordingShortcut, recordedKeys.length, updateSetting, toast]);
  const resetToDefaults = async () => {
    if (!isTauri()) {
      setSettings(DEFAULT_SETTINGS);
      return;
    }

    setIsSaving(true);
    try {
      const updated = await userApi.updateSettings(undefined, {
        theme: "dark",
        autoRestore: true,
        monitorDetection: true,
        animationsEnabled: true,
        autoActivateTime: "never",
        keyboardShortcut: "Cmd+Shift+1",
        notificationsEnabled: true,
      });
      setSettings(updated);
      toast({
        title: "Settings Reset",
        description: "All settings have been reset to defaults",
      });
    } catch {
      toast({
        title: "Error",
        description: "Failed to reset settings",
        variant: "destructive",
      });
    } finally {
      setIsSaving(false);
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
    hidden: { opacity: 0, x: -12 },
    visible: {
      opacity: 1,
      x: 0,
      transition: { duration: 0.3 },
    },
  };

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
          id: "notificationsEnabled",
          icon: Info,
          title: "Enable Notifications",
          description: "Show notifications for profile changes and errors",
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
  ];

  const mainContent = isLoading ? (
    <div className="flex-1 flex items-center justify-center">
      <div className="flex flex-col items-center gap-4">
        <Loader2 className="w-8 h-8 animate-spin text-primary" />
        <p className="text-muted-foreground">Loading settings...</p>
      </div>
    </div>
  ) : (
    <motion.div
      className="space-y-6 flex-1"
      variants={containerVariants}
      initial="hidden"
      animate="visible"
    >
      {/* General Settings */}
      <motion.div variants={itemVariants}>
        <Card>
          <CardHeader>
            <CardTitle className="text-base">General Settings</CardTitle>
            <CardDescription>Core functionality and behavior</CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {settingGroups[0].items.map((item) => {
              const Icon = item.icon;
              const key = item.id as keyof UserSettings;
              return (
                <div
                  key={item.id}
                  className="flex items-center justify-between py-3 border-b border-border last:border-0"
                >
                  <div className="flex items-start gap-4">
                    <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center mt-1 shrink-0">
                      <Icon className="w-5 h-5 text-primary" />
                    </div>
                    <div>
                      <p className="font-medium text-foreground">
                        {item.title}
                      </p>
                      <p className="text-sm text-muted-foreground">
                        {item.description}
                      </p>
                    </div>
                  </div>
                  <Switch
                    checked={settings[key] as boolean}
                    onCheckedChange={(checked) => updateSetting(key, checked)}
                    disabled={isSaving}
                  />
                </div>
              );
            })}
          </CardContent>
        </Card>
      </motion.div>

      {/* Automation */}
      <motion.div variants={itemVariants}>
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Automation Rules</CardTitle>
            <CardDescription>
              Set up automatic profile activation
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-2">
              <label className="block text-sm font-medium text-foreground">
                Auto-Activate Profile
              </label>
              <p className="text-xs text-muted-foreground mb-3">
                Automatically switch to a profile at specific times
              </p>
              <Select
                value={settings.autoActivateTime}
                onValueChange={(value) =>
                  updateSetting("autoActivateTime", value)
                }
                disabled={isSaving}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Select option" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="never">Never</SelectItem>
                  <SelectItem value="morning">
                    Morning (9:00 AM) → Work
                  </SelectItem>
                  <SelectItem value="afternoon">
                    Afternoon (12:00 PM) → Break
                  </SelectItem>
                  <SelectItem value="evening">
                    Evening (6:00 PM) → Personal
                  </SelectItem>
                  <SelectItem value="custom">Custom Time...</SelectItem>
                </SelectContent>
              </Select>
              {settings.autoActivateTime === "custom" && (
                <div className="mt-3 p-3 bg-muted/50 border border-border rounded-lg">
                  <label className="block text-sm font-medium text-foreground mb-2">
                    Custom Time
                  </label>
                  <input
                    type="time"
                    value={customTime}
                    onChange={(e) => {
                      setCustomTime(e.target.value);
                      updateSetting(
                        "autoActivateTime",
                        `custom:${e.target.value}`,
                      );
                    }}
                    className="w-full px-3 py-2 bg-background border border-border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent"
                    disabled={isSaving}
                  />
                </div>
              )}
            </div>

            <div className="space-y-2 border-t border-border pt-6">
              <label className="block text-sm font-medium text-foreground">
                Keyboard Shortcut
              </label>
              <p className="text-xs text-muted-foreground mb-3">
                Quick access to profile activation
              </p>
              <div className="flex gap-2">
                <div
                  className={`flex-1 font-mono border px-4 py-2 rounded-lg text-sm flex items-center ${
                    isRecordingShortcut
                      ? "bg-red-500/10 border-red-500/50 text-red-400 animate-pulse"
                      : "bg-muted border-border text-foreground"
                  }`}
                >
                  {isRecordingShortcut
                    ? recordedKeys.length > 0
                      ? recordedKeys.join("+")
                      : "Press keys..."
                    : settings.keyboardShortcut}
                </div>
                <Button
                  variant={isRecordingShortcut ? "destructive" : "outline"}
                  onClick={() => setIsRecordingShortcut(!isRecordingShortcut)}
                  disabled={isSaving}
                >
                  {isRecordingShortcut ? "Cancel" : "Record"}
                </Button>
              </div>
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
              <label className="block text-sm font-medium text-foreground">
                Theme
              </label>
              <Select
                value={settings.theme}
                onValueChange={(value) => updateSetting("theme", value)}
                disabled={isSaving}
              >
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

            <div className="space-y-2">
              <label className="block text-sm font-medium text-foreground">
                Default Profile
              </label>
              <p className="text-xs text-muted-foreground mb-3">
                Profile to activate when Smoothie starts
              </p>
              <Select
                value={settings.defaultProfileId || "none"}
                onValueChange={(value) =>
                  updateSetting(
                    "defaultProfileId",
                    value === "none" ? undefined : value,
                  )
                }
                disabled={isSaving}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Select default profile" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">None</SelectItem>
                  {profiles.map((profile) => (
                    <SelectItem key={profile.id} value={profile.id}>
                      {profile.name}
                    </SelectItem>
                  ))}
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
                <span className="text-sm text-muted-foreground">
                  Last Updated
                </span>
                <span className="text-sm font-semibold">Dec 1, 2025</span>
              </div>
              {settings.updatedAt && (
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">
                    Settings Synced
                  </span>
                  <span className="text-sm font-semibold">
                    {new Date(settings.updatedAt).toLocaleString()}
                  </span>
                </div>
              )}
            </div>

            <div className="flex gap-2">
              <Button
                variant="outline"
                className="flex-1 gap-2 bg-transparent"
                onClick={resetToDefaults}
                disabled={isSaving}
              >
                {isSaving ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <RotateCcw className="w-4 h-4" />
                )}
                Reset to Defaults
              </Button>
            </div>
          </CardContent>
        </Card>
      </motion.div>
    </motion.div>
  );

  return (
    <div className="h-full flex flex-col p-8 gap-8 overflow-y-auto">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: -10 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <h3 className="text-lg font-semibold text-foreground mb-1">
            Application Settings
          </h3>
          <p className="text-sm text-muted-foreground">
            Customize Smoothie behavior and preferences
          </p>
        </div>
        {isSaving && (
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <Loader2 className="w-4 h-4 animate-spin" />
            Saving...
          </div>
        )}
      </motion.div>

      {mainContent}
    </div>
  );
}
