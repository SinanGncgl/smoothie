"use client";

import { createContext, useContext, useEffect, useState } from "react";

import { setCurrentUserId } from "@/lib/tauri";

// Simplified user type for local-only mode
interface User {
  id: string;
  email: string;
  user_metadata?: {
    avatar_url?: string;
    full_name?: string;
  };
}

interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  isConfigured: boolean;
  signInWithGoogle: () => Promise<void>;
  signInWithGitHub: () => Promise<void>;
  signInWithApple: () => Promise<void>;
  signInWithEmail: (email: string, password: string) => Promise<void>;
  signUpWithEmail: (email: string, password: string) => Promise<void>;
  signInWithMagicLink: (email: string) => Promise<void>;
  signOut: () => Promise<void>;
  refreshSession: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

// Default user for local-only mode
const DEFAULT_USER: User = {
  id: "00000000-0000-0000-0000-000000000001",
  email: "local@example.com",
};

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Initialize with default user for local-only mode
  useEffect(() => {
    setUser(DEFAULT_USER);
    setCurrentUserId(DEFAULT_USER.id);
    setIsLoading(false);
  }, []);

  // No-op auth methods for open source version
  const signInWithGoogle = async () => {};
  const signInWithGitHub = async () => {};
  const signInWithApple = async () => {};
  const signInWithEmail = async (_email: string, _password: string) => {};
  const signUpWithEmail = async (_email: string, _password: string) => {};
  const signInWithMagicLink = async (_email: string) => {};
  const signOut = async () => {};
  const refreshSession = async () => {};

  const value: AuthContextType = {
    user,
    isLoading,
    isAuthenticated: !!user,
    isConfigured: true,
    signInWithGoogle,
    signInWithGitHub,
    signInWithApple,
    signInWithEmail,
    signUpWithEmail,
    signInWithMagicLink,
    signOut,
    refreshSession,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}

// Hook for getting user ID (always returns default user ID for local-only mode)
export function useUserId(): string {
  return DEFAULT_USER.id;
}
