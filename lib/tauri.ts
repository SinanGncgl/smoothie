// Tauri API wrapper for frontend-backend communication

import { invoke } from '@tauri-apps/api/core';

// Types matching Rust backend models
export interface Profile {
  id: string;
  user_id: string;
  name: string;
  description?: string;
  profile_type: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
  monitors?: Monitor[];
  apps?: App[];
  browser_tabs?: BrowserTab[];
}

export interface Monitor {
  id: string;
  profile_id: string;
  name: string;
  resolution: string;
  orientation: string;
  is_primary: boolean;
  x: number;
  y: number;
  width: number;
  height: number;
  display_index: number;
}

export interface App {
  id: string;
  profile_id: string;
  name: string;
  path: string;
  arguments?: string;
  auto_launch: boolean;
  monitor_index?: number;
  created_at: string;
}

export interface BrowserTab {
  id: string;
  profile_id: string;
  url: string;
  title?: string;
  browser: string;
  monitor_index?: number;
  tab_order: number;
}

export interface AutomationRule {
  id: string;
  profile_id: string;
  name: string;
  conditions: Record<string, unknown>;
  enabled: boolean;
  created_at: string;
}

export interface CreateProfileRequest {
  name: string;
  description?: string;
  profile_type: string;
}

export interface SuccessResponse<T> {
  success: boolean;
  data: T;
}

// Default user ID for now (will be replaced with auth)
const DEFAULT_USER_ID = "00000000-0000-0000-0000-000000000001";

// Profile API
export const profileApi = {
  async getProfiles(userId: string = DEFAULT_USER_ID): Promise<Profile[]> {
    console.log("[profileApi.getProfiles] Calling invoke with userId:", userId);
    try {
      const response = await invoke<SuccessResponse<Profile[]>>('get_profiles', { userId });
      console.log("[profileApi.getProfiles] Raw response:", response);
      
      if (response && response.data) {
        return response.data;
      }
      
      // If response is directly an array (in case backend returns differently)
      if (Array.isArray(response)) {
        return response as unknown as Profile[];
      }
      
      console.warn("[profileApi.getProfiles] Unexpected response format:", response);
      return [];
    } catch (err) {
      console.error("[profileApi.getProfiles] invoke error:", err);
      console.error("[profileApi.getProfiles] invoke error type:", typeof err);
      // Re-throw with more context
      throw err;
    }
  },

  async getProfile(profileId: string): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('get_profile', { profileId });
    return response.data;
  },

  async createProfile(req: CreateProfileRequest, userId: string = DEFAULT_USER_ID): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('create_profile', { userId, req });
    return response.data;
  },

  async updateProfile(profileId: string, name?: string, description?: string): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('update_profile', { profileId, name, description });
    return response.data;
  },

  async deleteProfile(profileId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('delete_profile', { profileId });
  },

  async activateProfile(profileId: string, userId: string = DEFAULT_USER_ID): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('activate_profile', { profileId, userId });
    return response.data;
  },

  async duplicateProfile(profileId: string, userId: string = DEFAULT_USER_ID): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('duplicate_profile', { profileId, userId });
    return response.data;
  },
};

// Monitor API
export const monitorApi = {
  async getMonitors(profileId: string): Promise<Monitor[]> {
    const response = await invoke<SuccessResponse<Monitor[]>>('get_monitors', { profileId });
    return response.data;
  },

  async createMonitor(monitor: Omit<Monitor, 'id'>): Promise<Monitor> {
    const response = await invoke<SuccessResponse<Monitor>>('create_monitor', { 
      profileId: monitor.profile_id,
      name: monitor.name,
      resolution: monitor.resolution,
      orientation: monitor.orientation,
      isPrimary: monitor.is_primary,
      x: monitor.x,
      y: monitor.y,
      width: monitor.width,
      height: monitor.height,
      displayIndex: monitor.display_index,
    });
    return response.data;
  },

  async updateMonitor(monitorId: string, updates: Partial<Monitor>): Promise<Monitor> {
    const response = await invoke<SuccessResponse<Monitor>>('update_monitor', { 
      monitorId,
      ...updates,
    });
    return response.data;
  },

  async deleteMonitor(monitorId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('delete_monitor', { monitorId });
  },
};

// App API
export const appApi = {
  async getApps(profileId: string): Promise<App[]> {
    const response = await invoke<SuccessResponse<App[]>>('get_apps', { profileId });
    return response.data;
  },

  async createApp(app: Omit<App, 'id' | 'created_at'>): Promise<App> {
    const response = await invoke<SuccessResponse<App>>('create_app', {
      profileId: app.profile_id,
      name: app.name,
      path: app.path,
      arguments: app.arguments,
      autoLaunch: app.auto_launch,
      monitorIndex: app.monitor_index,
    });
    return response.data;
  },

  async updateApp(appId: string, updates: Partial<App>): Promise<App> {
    const response = await invoke<SuccessResponse<App>>('update_app', { appId, ...updates });
    return response.data;
  },

  async deleteApp(appId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('delete_app', { appId });
  },

  async launchApps(profileId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('launch_apps', { profileId });
  },
};

// Browser Tab API
export const browserTabApi = {
  async getBrowserTabs(profileId: string): Promise<BrowserTab[]> {
    const response = await invoke<SuccessResponse<BrowserTab[]>>('get_browser_tabs', { profileId });
    return response.data;
  },

  async createBrowserTab(tab: Omit<BrowserTab, 'id'>): Promise<BrowserTab> {
    const response = await invoke<SuccessResponse<BrowserTab>>('create_browser_tab', {
      profileId: tab.profile_id,
      url: tab.url,
      title: tab.title,
      browser: tab.browser,
      monitorIndex: tab.monitor_index,
      tabOrder: tab.tab_order,
    });
    return response.data;
  },

  async updateBrowserTab(tabId: string, updates: Partial<BrowserTab>): Promise<BrowserTab> {
    const response = await invoke<SuccessResponse<BrowserTab>>('update_browser_tab', { tabId, ...updates });
    return response.data;
  },

  async deleteBrowserTab(tabId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('delete_browser_tab', { tabId });
  },

  async openTabs(profileId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('open_tabs', { profileId });
  },
};

// Automation API
export const automationApi = {
  async getRules(profileId: string): Promise<AutomationRule[]> {
    const response = await invoke<SuccessResponse<AutomationRule[]>>('get_rules', { profileId });
    return response.data;
  },

  async createRule(rule: Omit<AutomationRule, 'id' | 'created_at'>): Promise<AutomationRule> {
    const response = await invoke<SuccessResponse<AutomationRule>>('create_rule', {
      profileId: rule.profile_id,
      name: rule.name,
      conditions: rule.conditions,
      enabled: rule.enabled,
    });
    return response.data;
  },

  async updateRule(ruleId: string, updates: Partial<AutomationRule>): Promise<AutomationRule> {
    const response = await invoke<SuccessResponse<AutomationRule>>('update_rule', { ruleId, ...updates });
    return response.data;
  },

  async deleteRule(ruleId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('delete_rule', { ruleId });
  },

  async evaluateRules(profileId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('evaluate_rules', { profileId });
  },
};

// Sync API
export const syncApi = {
  async backupProfiles(userId: string = DEFAULT_USER_ID): Promise<unknown> {
    const response = await invoke<SuccessResponse<unknown>>('backup_profiles', { userId });
    return response.data;
  },

  async restoreProfiles(userId: string, backup: unknown, strategy: string = "merge"): Promise<void> {
    await invoke<SuccessResponse<string>>('restore_profiles', { userId, backup, conflictStrategy: strategy });
  },

  async getSyncStatus(userId: string = DEFAULT_USER_ID): Promise<unknown> {
    const response = await invoke<SuccessResponse<unknown>>('get_sync_status', { userId });
    return response.data;
  },
};

// System API
export const systemApi = {
  async getConnectedMonitors(): Promise<unknown[]> {
    const response = await invoke<SuccessResponse<unknown[]>>('get_connected_monitors');
    return response.data;
  },

  async getRunningApps(): Promise<unknown[]> {
    const response = await invoke<SuccessResponse<unknown[]>>('get_running_apps');
    return response.data;
  },
};

// User Preferences API
export const userApi = {
  async getPreferences(userId: string = DEFAULT_USER_ID): Promise<unknown> {
    const response = await invoke<SuccessResponse<unknown>>('get_user_preferences', { userId });
    return response.data;
  },

  async updatePreferences(
    userId: string = DEFAULT_USER_ID,
    theme?: string,
    notificationsEnabled?: boolean,
    autoRestore?: boolean
  ): Promise<unknown> {
    const response = await invoke<SuccessResponse<unknown>>('update_user_preferences', {
      userId,
      theme,
      notificationsEnabled,
      autoRestore,
    });
    return response.data;
  },
};

// Helper to check if running in Tauri environment
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}
