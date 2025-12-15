"use client";

import { useState, useEffect, useCallback } from "react";
import { profileApi, monitorApi, systemApi, isTauri, type Profile, type CreateProfileRequest, type SystemMonitor } from "@/lib/tauri";

export function useProfiles() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isBackendConnected, setIsBackendConnected] = useState(false);

  const fetchProfiles = useCallback(async () => {
    setLoading(true);
    setError(null);
    
    if (isTauri()) {
      try {
        const data = await profileApi.getProfiles();
        setProfiles(data);
        setIsBackendConnected(true);
      } catch (err: unknown) {
        // Extract error message
        let errorMessage = "Failed to load profiles from database.";
        if (typeof err === "string") {
          errorMessage = err;
        } else if (err && typeof err === "object") {
          if ("message" in err) {
            errorMessage = String((err as { message: unknown }).message);
          } else {
            try {
              const errStr = JSON.stringify(err);
              if (errStr !== "{}") {
                errorMessage = errStr;
              }
            } catch {
              // ignore
            }
          }
        }
        
        setError(errorMessage);
        setProfiles([]);
        setIsBackendConnected(false);
      }
    } else {
      // Browser mode - no backend available
      setError("Running in browser mode. Please use the Tauri app for full functionality.");
      setProfiles([]);
      setIsBackendConnected(false);
    }
    
    setLoading(false);
  }, []);

  useEffect(() => {
    fetchProfiles();
  }, [fetchProfiles]);

  const createProfile = useCallback(async (req: CreateProfileRequest, captureCurrentMonitors: boolean = false) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot create profile.");
    }
    
    try {
      const newProfile = await profileApi.createProfile(req);
      
      // If capture is enabled, detect and save current monitors
      if (captureCurrentMonitors) {
        try {
          const connectedMonitors = await systemApi.getConnectedMonitors();
          
          // Save each monitor to the profile
          for (let i = 0; i < connectedMonitors.length; i++) {
            const monitor = connectedMonitors[i];
            await monitorApi.createMonitor({
              profileId: newProfile.id,
              name: monitor.name,
              resolution: monitor.resolution,
              orientation: monitor.orientation,
              isPrimary: monitor.isPrimary,
              x: monitor.x,
              y: monitor.y,
              width: monitor.width,
              height: monitor.height,
              displayIndex: monitor.displayId,
              brand: monitor.brand,
              model: monitor.model,
              refreshRate: monitor.refreshRate,
              scaleFactor: monitor.scaleFactor,
              isBuiltin: monitor.isBuiltin,
              colorDepth: undefined,
            });
          }
          
          // Refresh the profile to get updated monitor count
          const updatedProfile = await profileApi.getProfile(newProfile.id);
          setProfiles((prev) => [...prev, updatedProfile]);
          return updatedProfile;
        } catch {
          // Still return the profile even if monitor capture fails
          setProfiles((prev) => [...prev, newProfile]);
          return newProfile;
        }
      }
      
      setProfiles((prev) => [...prev, newProfile]);
      return newProfile;
    } catch (err) {
      throw err;
    }
  }, [isBackendConnected]);

  const updateProfile = useCallback(async (profileId: string, updates: { name?: string; description?: string; isFavorite?: boolean; color?: string; icon?: string; sortOrder?: number }) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot update profile.");
    }
    
    try {
      const updated = await profileApi.updateProfile(profileId, updates);
      setProfiles((prev) => prev.map((p) => (p.id === profileId ? updated : p)));
      return updated;
    } catch (err) {
      throw err;
    }
  }, [isBackendConnected]);

  const deleteProfile = useCallback(async (profileId: string) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot delete profile.");
    }
    
    try {
      await profileApi.deleteProfile(profileId);
      setProfiles((prev) => prev.filter((p) => p.id !== profileId));
    } catch (err) {
      throw err;
    }
  }, [isBackendConnected]);

  const activateProfile = useCallback(async (profileId: string) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot activate profile.");
    }
    
    try {
      const activated = await profileApi.activateProfile(profileId);
      setProfiles((prev) =>
        prev.map((p) => ({
          ...p,
          isActive: p.id === profileId,
        }))
      );
      return activated;
    } catch (err) {
      throw err;
    }
  }, [isBackendConnected]);

  const duplicateProfile = useCallback(async (profileId: string) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot duplicate profile.");
    }
    
    try {
      const duplicated = await profileApi.duplicateProfile(profileId);
      setProfiles((prev) => [...prev, duplicated]);
      return duplicated;
    } catch (err) {
      throw err;
    }
  }, [isBackendConnected]);

  // Save current monitors to an existing profile
  const captureMonitorsToProfile = useCallback(async (profileId: string) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot capture monitors.");
    }
    
    try {
      // Get current monitors from the profile first (to avoid duplicates)
      const existingMonitors = await monitorApi.getMonitors(profileId);
      
      // Delete existing monitors
      for (const monitor of existingMonitors) {
        await monitorApi.deleteMonitor(monitor.id);
      }
      
      // Detect and save current monitors
      const connectedMonitors = await systemApi.getConnectedMonitors();
      
      for (let i = 0; i < connectedMonitors.length; i++) {
        const monitor = connectedMonitors[i];
        await monitorApi.createMonitor({
          profileId: profileId,
          name: monitor.name,
          resolution: monitor.resolution,
          orientation: monitor.orientation,
          isPrimary: monitor.isPrimary,
          x: monitor.x,
          y: monitor.y,
          width: monitor.width,
          height: monitor.height,
          displayIndex: monitor.displayId,
          brand: monitor.brand,
          model: monitor.model,
          refreshRate: monitor.refreshRate,
          scaleFactor: monitor.scaleFactor,
          isBuiltin: monitor.isBuiltin,
          colorDepth: undefined,
        });
      }
      
      // Refresh the profile to get updated monitor count
      const updatedProfile = await profileApi.getProfile(profileId);
      setProfiles((prev) => prev.map((p) => (p.id === profileId ? updatedProfile : p)));
      return updatedProfile;
    } catch (err) {
      throw err;
    }
  }, [isBackendConnected]);

  return {
    profiles,
    loading,
    error,
    isBackendConnected,
    fetchProfiles,
    createProfile,
    updateProfile,
    deleteProfile,
    activateProfile,
    duplicateProfile,
    captureMonitorsToProfile,
  };
}
