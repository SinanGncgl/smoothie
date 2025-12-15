"use client";

import { motion } from "framer-motion";
import {
  Plus,
  Play,
  Trash2,
  Edit2,
  Clock,
  Monitor,
  Package,
  AlertCircle,
  Loader2,
  Camera,
} from "lucide-react";
import { useState } from "react";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useProfiles } from "@/hooks/use-profiles";

interface DashboardProps {
  setCurrentView: (
    view: "dashboard" | "profiles" | "monitor" | "settings",
  ) => void;
}

// Profile colors mapping
const PROFILE_COLORS: Record<string, string> = {
  Work: "from-blue-400 to-blue-600",
  Gaming: "from-purple-400 to-pink-600",
  Research: "from-green-400 to-emerald-600",
  Custom: "from-orange-400 to-red-500",
};

export function Dashboard({ setCurrentView }: DashboardProps) {
  const {
    profiles,
    loading,
    error,
    isBackendConnected,
    deleteProfile,
    activateProfile,
    createProfile,
    fetchProfiles,
  } = useProfiles();

  const [hoveredId, setHoveredId] = useState<string | null>(null)
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false)
  const [newProfileName, setNewProfileName] = useState("")
  const [newProfileDescription, setNewProfileDescription] = useState("")
  const [newProfileType, setNewProfileType] = useState("Custom")
  const [captureCurrentMonitors, setCaptureCurrentMonitors] = useState(true)
  const [isCreating, setIsCreating] = useState(false)

  const handleDelete = async (id: string) => {
    try {
      await deleteProfile(id);
    } catch (err) {
      // Error already handled by hook
    }
  };

  const handleActivate = async (id: string) => {
    try {
      await activateProfile(id);
    } catch (err) {
      // Error already handled by hook
    }
  };

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
        captureCurrentMonitors,
      );
      setIsCreateDialogOpen(false);
      setNewProfileName("");
      setNewProfileDescription("");
      setNewProfileType("Custom");
      setCaptureCurrentMonitors(true);
      // Refresh profiles to ensure UI is updated
      await fetchProfiles();
    } catch (err) {
      // Error already handled by hook
    } finally {
      setIsCreating(false);
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
    hidden: { opacity: 0, y: 16 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.3, ease: "easeOut" as const },
    },
  };

  const activeProfile = profiles.find((p) => p.isActive);
  const totalMonitors = profiles.reduce(
    (sum, p) => sum + (p.monitorCount || 0),
    0,
  );
  const totalApps = profiles.reduce((sum, p) => sum + (p.appCount || 0), 0);

  if (loading) {
    return (
      <div className="h-full flex items-center justify-center">
        <div className="flex flex-col items-center gap-4">
          <Loader2 className="w-8 h-8 animate-spin text-primary" />
          <p className="text-muted-foreground">Loading profiles...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col p-8 gap-8">
      {/* Backend Connection Status */}
      {error && (
        <motion.div
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center gap-2 text-sm text-yellow-500 bg-yellow-500/10 px-4 py-2 rounded-lg"
        >
          <AlertCircle className="w-4 h-4" />
          {error}
          {!isBackendConnected && " (Running in demo mode)"}
        </motion.div>
      )}

      {/* Quick Stats Row */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.05 }}
        >
          <Card className="bg-card/50 border-border/50 backdrop-blur">
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground mb-1">
                    Total Profiles
                  </p>
                  <p className="text-3xl font-bold">{profiles.length}</p>
                </div>
                <div className="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center">
                  <Package className="w-6 h-6 text-primary" />
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
        >
          <Card className="bg-card/50 border-border/50 backdrop-blur">
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground mb-1">
                    Connected Monitors
                  </p>
                  <p className="text-3xl font-bold">{totalMonitors}</p>
                </div>
                <div className="w-12 h-12 rounded-lg bg-accent/10 flex items-center justify-center">
                  <Monitor className="w-6 h-6 text-accent" />
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.15 }}
        >
          <Card className="bg-card/50 border-border/50 backdrop-blur">
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground mb-1">
                    Apps Managed
                  </p>
                  <p className="text-3xl font-bold">{totalApps}</p>
                </div>
                <div className="w-12 h-12 rounded-lg bg-chart-1/10 flex items-center justify-center">
                  <Package className="w-6 h-6 text-chart-1" />
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
        >
          <Card className="bg-card/50 border-border/50 backdrop-blur">
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground mb-1">
                    Active Profile
                  </p>
                  <p className="text-2xl font-bold text-primary">
                    {activeProfile?.name || "None"}
                  </p>
                </div>
                <div className="w-12 h-12 rounded-lg bg-primary/20 flex items-center justify-center">
                  <Clock className="w-6 h-6 text-primary animate-pulse" />
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </div>

      {/* Profiles Grid */}
      <div>
        <div className="flex items-center justify-between mb-6">
          <div>
            <h3 className="text-lg font-semibold text-foreground">
              Your Profiles
            </h3>
            <p className="text-sm text-muted-foreground">
              Click to activate or manage your workspace layouts
            </p>
          </div>
          <Dialog
            open={isCreateDialogOpen}
            onOpenChange={setIsCreateDialogOpen}
          >
            <DialogTrigger asChild>
              <Button className="gap-2">
                <Plus className="w-4 h-4" />
                New Profile
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Create New Profile</DialogTitle>
              </DialogHeader>
              <div className="space-y-4 py-4">
                <div className="space-y-2">
                  <Label htmlFor="name">Name</Label>
                  <Input
                    id="name"
                    placeholder="e.g., Work, Gaming, Research"
                    value={newProfileName}
                    onChange={(e) => setNewProfileName(e.target.value)}
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="description">Description</Label>
                  <Input
                    id="description"
                    placeholder="Describe this profile..."
                    value={newProfileDescription}
                    onChange={(e) => setNewProfileDescription(e.target.value)}
                  />
                </div>
                <div className="space-y-2">
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
                    checked={captureCurrentMonitors}
                    onCheckedChange={(checked) =>
                      setCaptureCurrentMonitors(checked === true)
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
                <Button
                  variant="outline"
                  onClick={() => setIsCreateDialogOpen(false)}
                >
                  Cancel
                </Button>
                <Button
                  onClick={handleCreateProfile}
                  disabled={isCreating || !newProfileName.trim()}
                >
                  {isCreating ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      Creating...
                    </>
                  ) : (
                    "Create Profile"
                  )}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>

        <motion.div
          className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
          variants={containerVariants}
          initial="hidden"
          animate="visible"
        >
          {profiles.map((profile) => (
            <motion.div
              key={profile.id}
              variants={itemVariants}
              onMouseEnter={() => setHoveredId(profile.id)}
              onMouseLeave={() => setHoveredId(null)}
            >
              <Card
                className={cn(
                  "h-full hover:border-primary/50 transition-all duration-300 cursor-pointer group relative overflow-hidden",
                  profile.isActive &&
                    "border-primary/50 ring-1 ring-primary/30",
                )}
              >
                {/* Active Indicator */}
                {profile.isActive && (
                  <div className="absolute top-0 right-0 w-full h-1 bg-linear-to-r from-primary to-accent" />
                )}

                <CardHeader>
                  <div
                    className={`w-full h-28 bg-linear-to-br ${PROFILE_COLORS[profile.profileType] || PROFILE_COLORS.Custom} rounded-lg mb-4 opacity-75 group-hover:opacity-100 transition-opacity duration-300 flex items-center justify-center`}
                  >
                    {profile.isActive && (
                      <div className="text-white font-bold text-sm bg-black/30 px-3 py-1 rounded">
                        Active
                      </div>
                    )}
                  </div>
                  <div className="flex items-start justify-between">
                    <div>
                      <CardTitle>{profile.name}</CardTitle>
                      <CardDescription>
                        {profile.description || "No description"}
                      </CardDescription>
                    </div>
                  </div>
                </CardHeader>

                <CardContent>
                  <div className="grid grid-cols-2 gap-3 mb-6">
                    <div className="flex items-center gap-2 text-sm">
                      <Monitor className="w-4 h-4 text-primary" />
                      <div>
                        <p className="text-xs text-muted-foreground">
                          Monitors
                        </p>
                        <p className="font-semibold">
                          {profile.monitorCount || 0}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-2 text-sm">
                      <Package className="w-4 h-4 text-accent" />
                      <div>
                        <p className="text-xs text-muted-foreground">Apps</p>
                        <p className="font-semibold">{profile.appCount || 0}</p>
                      </div>
                    </div>
                  </div>

                  <p className="text-xs text-muted-foreground mb-4 flex items-center gap-1">
                    <Clock className="w-3 h-3" />
                    Updated {new Date(profile.updatedAt).toLocaleDateString()}
                  </p>

                  <div className="flex gap-2">
                    <Button
                      size="sm"
                      className="flex-1 gap-2"
                      variant={profile.isActive ? "secondary" : "default"}
                      onClick={() => handleActivate(profile.id)}
                      disabled={profile.isActive}
                    >
                      <Play className="w-4 h-4" />
                      {profile.isActive ? "Active" : "Activate"}
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      className="gap-1 bg-transparent"
                      onClick={() => setCurrentView("profiles")}
                    >
                      <Edit2 className="w-4 h-4" />
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      className="text-destructive hover:text-destructive bg-transparent"
                      onClick={() => handleDelete(profile.id)}
                    >
                      <Trash2 className="w-4 h-4" />
                    </Button>
                  </div>
                </CardContent>
              </Card>
            </motion.div>
          ))}

          {/* Create New Profile Card */}
          <motion.div variants={itemVariants}>
            <Card
              className="h-full border-dashed hover:border-primary/50 transition-all duration-300 cursor-pointer flex items-center justify-center group"
              onClick={() => setIsCreateDialogOpen(true)}
            >
              <CardContent className="flex flex-col items-center gap-4 py-12">
                <div className="w-14 h-14 rounded-lg bg-primary/10 flex items-center justify-center group-hover:bg-primary/20 transition-colors">
                  <Plus className="w-7 h-7 text-primary" />
                </div>
                <div className="text-center">
                  <p className="font-semibold text-foreground">
                    Create Profile
                  </p>
                  <p className="text-sm text-muted-foreground">
                    New workspace layout
                  </p>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        </motion.div>
      </div>
    </div>
  );
}

function cn(...classes: (string | undefined | null | false)[]) {
  return classes.filter(Boolean).join(" ");
}
