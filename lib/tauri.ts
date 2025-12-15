// Tauri API wrapper for frontend-backend communication

import { invoke } from '@tauri-apps/api/core';

// Types matching Rust backend models (camelCase to match serde rename_all)
export interface Profile {
  id: string;
  userId: string;
  name: string;
  description?: string;
  profileType: string;
  isActive: boolean;
  tags?: string[];
  monitorCount: number;
  appCount: number;
  browserTabCount: number;
  createdAt: string;
  updatedAt: string;
  lastUsed?: string;
  // New fields from v4
  lastActivatedAt?: string;
  activationCount: number;
  isFavorite: boolean;
  color?: string;
  icon?: string;
  sortOrder: number;
  // Related entities (optional, loaded on demand)
  monitors?: Monitor[];
  apps?: App[];
  browserTabs?: BrowserTab[];
}

export interface Monitor {
  id: string;
  profileId: string;
  name: string;
  resolution: string;
  orientation: string;
  isPrimary: boolean;
  x: number;
  y: number;
  width: number;
  height: number;
  displayIndex: number;
  // New fields from v4
  brand?: string;
  model?: string;
  refreshRate?: number;
  scaleFactor?: number;
  isBuiltin?: boolean;
  colorDepth?: number;
  createdAt?: string;
  updatedAt?: string;
}

export interface App {
  id: string;
  profileId: string;
  name: string;
  bundleId: string;
  exePath?: string;
  launchOnActivate: boolean;
  monitorPreference?: number;
  createdAt: string;
  // New fields from v4
  updatedAt?: string;
  iconPath?: string;
  launchArgs?: string;
  workingDirectory?: string;
  startupDelayMs: number;
  orderIndex: number;
}

export interface BrowserTab {
  id: string;
  profileId: string;
  url: string;
  browser: string;
  monitorId?: string;
  tabOrder: number;
  favicon?: string;
  createdAt: string;
  updatedAt?: string;
}

export interface AutomationRule {
  id: string;
  profileId: string;
  ruleType: string;
  triggerConfig: Record<string, unknown>;
  isEnabled: boolean;
  createdAt: string;
}

export interface CreateProfileRequest {
  name: string;
  description?: string;
  profileType: string;
}

export interface UpdateProfileRequest {
  name?: string;
  description?: string;
  isFavorite?: boolean;
  color?: string;
  icon?: string;
  sortOrder?: number;
}

export interface LaunchResult {
  name: string;
  success: boolean;
  message: string;
}

export interface OpenTabResult {
  url: string;
  browser: string;
  success: boolean;
  message: string;
}

export interface MonitorLayoutResult {
  applied: boolean;
  monitorCount: number;
  message: string;
}

export interface StartProfileResult {
  profileId: string;
  appsLaunched: LaunchResult[];
  tabsOpened: OpenTabResult[];
  monitorLayout: MonitorLayoutResult;
}

export interface SuccessResponse<T> {
  success: boolean;
  data: T;
}

// Activity and Analytics types
export interface ProfileActivation {
  id: string;
  userId: string;
  profileId: string;
  activatedAt: string;
  deactivatedAt?: string;
  activationSource: string;
  monitorsApplied: number;
  appsLaunched: number;
  tabsOpened: number;
  durationSeconds?: number;
  success: boolean;
  errorMessage?: string;
}

export interface ActivityLog {
  id: string;
  userId: string;
  action: string;
  entityType?: string;
  entityId?: string;
  entityName?: string;
  details?: Record<string, unknown>;
  createdAt: string;
}

export interface ScheduledTask {
  id: string;
  userId: string;
  profileId: string;
  name: string;
  scheduleType: string;
  cronExpression?: string;
  triggerConditions?: Record<string, unknown>;
  isEnabled: boolean;
  lastTriggeredAt?: string;
  nextTriggerAt?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface SyncHistory {
  id: string;
  userId: string;
  profileId?: string;
  syncType: string;
  status: string;
  startedAt: string;
  completedAt?: string;
  itemsSynced: number;
  errorMessage?: string;
  metadata?: Record<string, unknown>;
  createdAt?: string;
}

// Default user ID for local-only mode (when Supabase is not configured)
const DEFAULT_USER_ID = "00000000-0000-0000-0000-000000000001";

// User ID storage - can be set dynamically from auth context
let currentUserId: string = DEFAULT_USER_ID;

// Set the current user ID (called from auth context when user logs in)
export function setCurrentUserId(userId: string) {
  currentUserId = userId || DEFAULT_USER_ID;
}

// Get the current user ID
export function getCurrentUserId(): string {
  return currentUserId;
}

// Profile API
export const profileApi = {
  async getProfiles(userId: string = currentUserId): Promise<Profile[]> {
    try {
      const response = await invoke<SuccessResponse<Profile[]>>('get_profiles', { userId });
      
      if (response && response.data) {
        return response.data;
      }
      
      // If response is directly an array (in case backend returns differently)
      if (Array.isArray(response)) {
        return response as unknown as Profile[];
      }
      
      return [];
    } catch (err) {
      throw err;
    }
  },

  async getProfile(profileId: string): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('get_profile', { profileId });
    return response.data;
  },

  async getFavorites(userId: string = currentUserId): Promise<Profile[]> {
    const response = await invoke<SuccessResponse<Profile[]>>('get_favorite_profiles', { userId });
    return response.data;
  },

  async getMostUsed(userId: string = currentUserId, limit: number = 5): Promise<Profile[]> {
    const response = await invoke<SuccessResponse<Profile[]>>('get_most_used_profiles', { userId, limit });
    return response.data;
  },

  async createProfile(req: CreateProfileRequest, userId: string = currentUserId): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('create_profile', { userId, req });
    return response.data;
  },

  async updateProfile(profileId: string, updates: UpdateProfileRequest): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('update_profile', { profileId, ...updates });
    return response.data;
  },

  async setFavorite(profileId: string, isFavorite: boolean): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('set_profile_favorite', { profileId, isFavorite });
    return response.data;
  },

  async deleteProfile(profileId: string): Promise<void> {
    await invoke<SuccessResponse<string>>('delete_profile', { profileId });
  },

  async activateProfile(profileId: string, userId: string = currentUserId): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('activate_profile', { profileId, userId });
    return response.data;
  },

  async duplicateProfile(profileId: string, userId: string = currentUserId): Promise<Profile> {
    const response = await invoke<SuccessResponse<Profile>>('duplicate_profile', { profileId, userId });
    return response.data;
  },

  async startProfile(profileId: string, userId: string = currentUserId): Promise<StartProfileResult> {
    const response = await invoke<SuccessResponse<StartProfileResult>>('start_profile', { profileId, userId });
    return response.data;
  },
};

// Monitor API
export const monitorApi = {
  async getMonitors(profileId: string): Promise<Monitor[]> {
    const response = await invoke<SuccessResponse<Monitor[]>>('get_monitors', { profileId });
    return response.data;
  },

  async createMonitor(monitor: Omit<Monitor, 'id' | 'createdAt' | 'updatedAt'>): Promise<Monitor> {
    const response = await invoke<SuccessResponse<Monitor>>('create_monitor', { 
      profileId: monitor.profileId,
      name: monitor.name,
      resolution: monitor.resolution,
      orientation: monitor.orientation,
      isPrimary: monitor.isPrimary,
      x: monitor.x,
      y: monitor.y,
      width: monitor.width,
      height: monitor.height,
      displayIndex: monitor.displayIndex,
      brand: monitor.brand,
      model: monitor.model,
      refreshRate: monitor.refreshRate,
      scaleFactor: monitor.scaleFactor,
      isBuiltin: monitor.isBuiltin,
      colorDepth: monitor.colorDepth,
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

  async createApp(app: Omit<App, 'id' | 'createdAt' | 'updatedAt'>, userId: string = currentUserId): Promise<App> {
    const response = await invoke<SuccessResponse<App>>('create_app', {
      profileId: app.profileId,
      userId,
      name: app.name,
      bundleId: app.bundleId,
      exePath: app.exePath,
      launchOnActivate: app.launchOnActivate,
      monitorPreference: app.monitorPreference,
      iconPath: app.iconPath,
      launchArgs: app.launchArgs,
      workingDirectory: app.workingDirectory,
      startupDelayMs: app.startupDelayMs,
      orderIndex: app.orderIndex,
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

  async launchApps(profileId: string): Promise<LaunchResult[]> {
    const response = await invoke<SuccessResponse<LaunchResult[]>>('launch_apps', { profileId });
    return response.data;
  },
};

// Browser Tab API
export const browserTabApi = {
  async getBrowserTabs(profileId: string): Promise<BrowserTab[]> {
    const response = await invoke<SuccessResponse<BrowserTab[]>>('get_browser_tabs', { profileId });
    return response.data;
  },

  async createBrowserTab(tab: Omit<BrowserTab, 'id' | 'createdAt' | 'updatedAt'>): Promise<BrowserTab> {
    const response = await invoke<SuccessResponse<BrowserTab>>('create_browser_tab', {
      profileId: tab.profileId,
      url: tab.url,
      browser: tab.browser,
      monitorId: tab.monitorId || null,
      tabOrder: tab.tabOrder,
      favicon: tab.favicon,
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

  async openTabs(profileId: string): Promise<OpenTabResult[]> {
    const response = await invoke<SuccessResponse<OpenTabResult[]>>('open_tabs', { profileId });
    return response.data;
  },
};

// Automation API
export const automationApi = {
  async getRules(profileId: string): Promise<AutomationRule[]> {
    const response = await invoke<SuccessResponse<AutomationRule[]>>('get_rules', { profileId });
    return response.data;
  },

  async createRule(rule: Omit<AutomationRule, 'id' | 'createdAt'>): Promise<AutomationRule> {
    const response = await invoke<SuccessResponse<AutomationRule>>('create_rule', {
      profileId: rule.profileId,
      ruleType: rule.ruleType,
      triggerConfig: rule.triggerConfig,
      isEnabled: rule.isEnabled,
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

// System Types - detected from OS
export interface SystemMonitor {
  displayId: number;
  name: string;
  brand?: string;
  model?: string;
  resolution: string;
  width: number;
  height: number;
  x: number;
  y: number;
  scaleFactor: number;
  refreshRate: number;
  isPrimary: boolean;
  isBuiltin: boolean;
  orientation: string;
}

export interface SystemWindow {
  windowId: number;
  pid: number;
  title: string;
  appName: string;
  bundleId: string;
  x: number;
  y: number;
  width: number;
  height: number;
  displayId: number;
  isMinimized: boolean;
  isFullscreen: boolean;
  layer: number;
}

export interface RunningApp {
  pid: number;
  name: string;
  bundleId: string;
  path?: string;
  isActive: boolean;
  isHidden: boolean;
  windowCount: number;
}

export interface InstalledApp {
  name: string;
  bundleId: string;
  path: string;
  version?: string;
  category?: string;
}

export interface CapturedLayout {
  capturedAt: string;
  monitors: SystemMonitor[];
  windows: SystemWindow[];
  runningApps: RunningApp[];
}

// System API
export const systemApi = {
  async getConnectedMonitors(): Promise<SystemMonitor[]> {
    const response = await invoke<SuccessResponse<SystemMonitor[]>>('get_connected_monitors');
    return response.data;
  },

  async getVisibleWindows(): Promise<SystemWindow[]> {
    const response = await invoke<SuccessResponse<SystemWindow[]>>('get_visible_windows');
    return response.data;
  },

  async getRunningApps(): Promise<RunningApp[]> {
    const response = await invoke<SuccessResponse<RunningApp[]>>('get_running_apps');
    return response.data;
  },

  async getInstalledApps(): Promise<InstalledApp[]> {
    const response = await invoke<SuccessResponse<InstalledApp[]>>('get_installed_apps');
    return response.data;
  },

  async captureCurrentLayout(): Promise<CapturedLayout> {
    const response = await invoke<SuccessResponse<CapturedLayout>>('capture_current_layout');
    return response.data;
  },

  async applyMonitorLayout(monitors: SystemMonitor[]): Promise<string> {
    const response = await invoke<SuccessResponse<string>>('apply_monitor_layout', { monitors });
    return response.data;
  },

  async checkDisplayPermission(): Promise<boolean> {
    const response = await invoke<SuccessResponse<boolean>>('check_display_permission');
    return response.data;
  },

  async requestDisplayPermission(): Promise<boolean> {
    const response = await invoke<SuccessResponse<boolean>>('request_display_permission');
    return response.data;
  },
};

// User Preferences API
export interface UserSettings {
  id: string;
  userId: string;
  theme: string;
  autoRestore: boolean;
  monitorDetection: boolean;
  animationsEnabled: boolean;
  autoActivateTime: string;
  keyboardShortcut: string;
  notificationsEnabled: boolean;
  createdAt: string;
  updatedAt: string;
  // New fields from v4
  defaultProfileId?: string;
  lastActiveProfileId?: string;
  onboardingCompleted: boolean;
  onboardingStep: number;
  featureFlags?: Record<string, unknown>;
  keyboardShortcuts?: Record<string, string>;
  uiPreferences?: Record<string, unknown>;
}

export const userApi = {
  async getSettings(userId: string = currentUserId): Promise<UserSettings> {
    const response = await invoke<SuccessResponse<UserSettings>>('get_user_settings', { userId });
    return response.data;
  },

  async updateSettings(
    userId: string = currentUserId,
    settings: Partial<Omit<UserSettings, 'id' | 'userId' | 'createdAt' | 'updatedAt'>>
  ): Promise<UserSettings> {
    const response = await invoke<SuccessResponse<UserSettings>>('update_user_settings', {
      userId,
      ...settings,
    });
    return response.data;
  },

  // Keep old methods for backward compatibility
  async getPreferences(userId: string = currentUserId): Promise<UserSettings> {
    return this.getSettings(userId);
  },

  async updatePreferences(
    userId: string = currentUserId,
    theme?: string,
    notificationsEnabled?: boolean,
    autoRestore?: boolean
  ): Promise<UserSettings> {
    return this.updateSettings(userId, { theme, notificationsEnabled, autoRestore });
  },
};

// System Event / Notification types
export interface SystemEvent {
  id: string;
  eventType: string;
  severity: string;
  source: string;
  message: string;
  details?: Record<string, unknown>;
  stackTrace?: string;
  osInfo?: Record<string, unknown>;
  appVersion?: string;
  createdAt: string;
}

export interface GetSystemEventsParams {
  eventType?: string;
  severity?: string;
  source?: string;
  limit?: number;
  offset?: number;
}

// Notification / Audit API
export const notificationApi = {
  async getSystemEvents(params: GetSystemEventsParams = {}): Promise<SystemEvent[]> {
    // Note: get_system_events returns Vec<SystemEventDto> directly, not wrapped in SuccessResponse
    const events = await invoke<SystemEvent[]>('get_system_events', {
      limit: params.limit,
      offset: params.offset,
      severity: params.severity,
      eventType: params.eventType,
    });
    return events;
  },

  async getRecentNotifications(limit: number = 10): Promise<SystemEvent[]> {
    return this.getSystemEvents({ limit });
  },
};

// Feedback types
export interface Feedback {
  id: string;
  userId: string;
  feedbackType: string;
  title: string;
  description: string;
  priority: string;
  status: string;
  category?: string;
  contactEmail?: string;
  appVersion?: string;
  osInfo?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

export interface CreateFeedbackRequest {
  feedbackType: string;
  title: string;
  description: string;
  priority?: string;
  category?: string;
  contactEmail?: string;
}

// Feedback API
export const feedbackApi = {
  async submitFeedback(req: CreateFeedbackRequest): Promise<Feedback> {
    const response = await invoke<SuccessResponse<Feedback>>('submit_feedback', { req });
    return response.data;
  },

  async getFeedback(status?: string, feedbackType?: string, limit?: number): Promise<Feedback[]> {
    const response = await invoke<SuccessResponse<Feedback[]>>('get_feedback', {
      status,
      feedbackType,
      limit,
    });
    return response.data;
  },

  async updateFeedbackStatus(feedbackId: string, status: string): Promise<Feedback> {
    const response = await invoke<SuccessResponse<Feedback>>('update_feedback_status', {
      feedbackId,
      status,
    });
    return response.data;
  },
};

// Helper to check if running in Tauri environment
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}
