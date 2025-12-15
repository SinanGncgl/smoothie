"use client";

import { createContext, useContext } from "react";

// Subscription types (simplified for free-only version)
export type SubscriptionTier = "free";

export interface SubscriptionInfo {
  tier: SubscriptionTier;
  status: "active" | null;
}

// OSS version: unlimited free tier
export const TIER_LIMITS = {
  free: {
    maxProfiles: Infinity,
    maxMonitorsPerProfile: Infinity,
    features: [
      "Unlimited profiles",
      "Unlimited monitors per profile",
      "All features included",
    ],
  },
};

// Default subscription info for free users
export const DEFAULT_SUBSCRIPTION: SubscriptionInfo = {
  tier: "free",
  status: "active",
};

// OSS version: always allow creating profiles
export function canCreateProfile(
  tier: SubscriptionTier,
  currentProfileCount: number,
): boolean {
  return true; // Unlimited profiles
}

// Get remaining profile slots (always infinite)
export function getRemainingProfileSlots(
  tier: SubscriptionTier,
  currentProfileCount: number,
): number {
  return Infinity;
}

interface SubscriptionContextType {
  subscription: SubscriptionInfo;
  isLoading: boolean;
  isPro: boolean;
  tier: SubscriptionTier;
  canCreateProfile: (currentCount: number) => boolean;
  getRemainingProfileSlots: (currentCount: number) => number;
}

const SubscriptionContext = createContext<SubscriptionContextType | undefined>(
  undefined,
);

export function SubscriptionProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  // Always return free tier for open source version
  const subscription = DEFAULT_SUBSCRIPTION;
  const isLoading = false;

  const value: SubscriptionContextType = {
    subscription,
    isLoading,
    isPro: false, // Always free tier
    tier: "free",
    canCreateProfile: (currentCount: number) =>
      canCreateProfile("free", currentCount),
    getRemainingProfileSlots: (currentCount: number) =>
      getRemainingProfileSlots("free", currentCount),
  };

  return (
    <SubscriptionContext.Provider value={value}>
      {children}
    </SubscriptionContext.Provider>
  );
}

export function useSubscription() {
  const context = useContext(SubscriptionContext);
  if (context === undefined) {
    throw new Error(
      "useSubscription must be used within a SubscriptionProvider",
    );
  }
  return context;
}
