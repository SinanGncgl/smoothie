"use client";

import { useState, useEffect, useCallback } from "react";
import { profileApi, isTauri, type Profile, type CreateProfileRequest } from "@/lib/tauri";

export function useProfiles() {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isBackendConnected, setIsBackendConnected] = useState(false);

  const fetchProfiles = useCallback(async () => {
    setLoading(true);
    setError(null);
    
    console.log("[useProfiles] Checking if running in Tauri...");
    console.log("[useProfiles] isTauri():", isTauri());
    
    if (isTauri()) {
      try {
        console.log("[useProfiles] Calling profileApi.getProfiles()...");
        const data = await profileApi.getProfiles();
        console.log("[useProfiles] Received profiles:", data);
        setProfiles(data);
        setIsBackendConnected(true);
      } catch (err: unknown) {
        console.error("[useProfiles] Failed to fetch profiles from backend:");
        console.error("[useProfiles] Error:", err);
        console.error("[useProfiles] Error type:", typeof err);
        
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
        
        console.error("[useProfiles] Error message:", errorMessage);
        setError(errorMessage);
        setProfiles([]);
        setIsBackendConnected(false);
      }
    } else {
      // Browser mode - no backend available
      console.log("[useProfiles] Running in browser mode - backend not available");
      setError("Running in browser mode. Please use the Tauri app for full functionality.");
      setProfiles([]);
      setIsBackendConnected(false);
    }
    
    setLoading(false);
  }, []);

  useEffect(() => {
    fetchProfiles();
  }, [fetchProfiles]);

  const createProfile = useCallback(async (req: CreateProfileRequest) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot create profile.");
    }
    
    try {
      const newProfile = await profileApi.createProfile(req);
      setProfiles((prev) => [...prev, newProfile]);
      return newProfile;
    } catch (err) {
      console.error("Failed to create profile:", err);
      throw err;
    }
  }, [isBackendConnected]);

  const updateProfile = useCallback(async (profileId: string, name?: string, description?: string) => {
    if (!isTauri() || !isBackendConnected) {
      throw new Error("Backend not connected. Cannot update profile.");
    }
    
    try {
      const updated = await profileApi.updateProfile(profileId, name, description);
      setProfiles((prev) => prev.map((p) => (p.id === profileId ? updated : p)));
      return updated;
    } catch (err) {
      console.error("Failed to update profile:", err);
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
      console.error("Failed to delete profile:", err);
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
          is_active: p.id === profileId,
        }))
      );
      return activated;
    } catch (err) {
      console.error("Failed to activate profile:", err);
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
      console.error("Failed to duplicate profile:", err);
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
  };
}
