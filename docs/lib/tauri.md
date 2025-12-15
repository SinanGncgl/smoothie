# lib/tauri.ts - Tauri API Wrapper and Type Definitions

## Overview
The central TypeScript API wrapper for all frontend-backend communication in the Smoothie application. Provides typed interfaces for Tauri commands, data structures matching the Rust backend, and environment detection utilities.

## Key Responsibilities
- **Type Safety**: Strongly typed interfaces matching Rust models
- **API Abstraction**: Clean, promise-based API for all backend operations
- **Environment Detection**: Automatic Tauri vs browser environment handling
- **Error Handling**: Consistent error propagation and logging
- **Response Normalization**: Standardized success/error response handling

## Core Architecture

### Type Definitions
All interfaces use camelCase naming to match Tauri/Rust serde serialization:

#### Profile Management Types
```typescript
interface Profile {
  id: string;
  userId: string;
  name: string;
  description?: string;
  profileType: string;
  isActive: boolean;
  tags?: string[];
  createdAt: string;
  updatedAt: string;
  lastUsed?: string;
  monitors?: Monitor[];
  apps?: App[];
  browserTabs?: BrowserTab[];
}
```

#### System Detection Types
```typescript
interface SystemMonitor {
  displayId: number;
  name: string;
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

interface SystemWindow {
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

interface RunningApp {
  pid: number;
  name: string;
  bundleId: string;
  path?: string;
  isActive: boolean;
  isHidden: boolean;
  windowCount: number;
}
```

## API Modules

### Profile API (`profileApi`)
Complete CRUD operations for workspace profiles:

```typescript
const profileApi = {
  getProfiles(userId?: string): Promise<Profile[]>
  getProfile(profileId: string): Promise<Profile>
  createProfile(req: CreateProfileRequest, userId?: string): Promise<Profile>
  updateProfile(profileId: string, name?: string, description?: string): Promise<Profile>
  deleteProfile(profileId: string): Promise<void>
  activateProfile(profileId: string, userId?: string): Promise<Profile>
  duplicateProfile(profileId: string, userId?: string): Promise<Profile>
}
```

### System API (`systemApi`)
macOS system detection operations:

```typescript
const systemApi = {
  getConnectedMonitors(): Promise<SystemMonitor[]>
  getVisibleWindows(): Promise<SystemWindow[]>
  getRunningApps(): Promise<RunningApp[]>
  captureCurrentLayout(): Promise<CapturedLayout>
}
```

### Monitor API (`monitorApi`)
Monitor configuration management:

```typescript
const monitorApi = {
  getMonitors(profileId: string): Promise<Monitor[]>
  createMonitor(monitor: Omit<Monitor, 'id'>): Promise<Monitor>
  updateMonitor(monitorId: string, updates: Partial<Monitor>): Promise<Monitor>
  deleteMonitor(monitorId: string): Promise<void>
}
```

### App API (`appApi`)
Application management within profiles:

```typescript
const appApi = {
  getApps(profileId: string): Promise<App[]>
  createApp(app: Omit<App, 'id' | 'createdAt'>): Promise<App>
  updateApp(appId: string, updates: Partial<App>): Promise<App>
  deleteApp(appId: string): Promise<void>
  launchApps(profileId: string): Promise<void>
}
```

### Browser Tab API (`browserTabApi`)
Browser tab management:

```typescript
const browserTabApi = {
  getBrowserTabs(profileId: string): Promise<BrowserTab[]>
  createBrowserTab(tab: Omit<BrowserTab, 'id' | 'createdAt'>): Promise<BrowserTab>
  updateBrowserTab(tabId: string, updates: Partial<BrowserTab>): Promise<BrowserTab>
  deleteBrowserTab(tabId: string): Promise<void>
  openTabs(profileId: string): Promise<void>
}
```

### Automation API (`automationApi`)
Automation rule management:

```typescript
const automationApi = {
  getRules(profileId: string): Promise<AutomationRule[]>
  createRule(rule: Omit<AutomationRule, 'id' | 'createdAt'>): Promise<AutomationRule>
  updateRule(ruleId: string, updates: Partial<AutomationRule>): Promise<AutomationRule>
  deleteRule(ruleId: string): Promise<void>
  evaluateRules(profileId: string): Promise<void>
}
```

### Sync API (`syncApi`)
Data synchronization operations:

```typescript
const syncApi = {
  backupProfiles(userId?: string): Promise<unknown>
  restoreProfiles(userId: string, backup: unknown, strategy?: string): Promise<void>
  getSyncStatus(userId?: string): Promise<unknown>
}
```

### User API (`userApi`)
User preferences management:

```typescript
const userApi = {
  getPreferences(userId?: string): Promise<unknown>
  updatePreferences(userId?: string, theme?: string, notificationsEnabled?: boolean, autoRestore?: boolean): Promise<unknown>
}
```

## Response Handling

### Success Response Wrapper
All API calls return normalized responses:
```typescript
interface SuccessResponse<T> {
  success: boolean;  // Always true for successful responses
  data: T;          // The actual response data
}
```

### Error Handling
- **Tauri Errors**: Automatically converted to JavaScript Error objects
- **Network Errors**: Connection failures to backend
- **Validation Errors**: Input validation failures
- **Logging**: Comprehensive error logging with context

## Environment Detection

### isTauri() Function
```typescript
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}
```
Detects whether code is running in Tauri desktop environment vs browser.

## Implementation Details

### Invoke Pattern
All API calls use Tauri's `invoke` function:
```typescript
import { invoke } from '@tauri-apps/api/core';

const response = await invoke<SuccessResponse<Profile[]>>('get_profiles', { userId });
return response.data;
```

### Parameter Mapping
- **Rust snake_case** → **TypeScript camelCase** conversion handled by serde
- **Optional Parameters**: Sensible defaults (e.g., `DEFAULT_USER_ID`)
- **Complex Objects**: Proper destructuring and parameter passing

### Logging and Debugging
Comprehensive logging for development:
```typescript
console.log("[profileApi.getProfiles] Calling invoke with userId:", userId);
console.log("[profileApi.getProfiles] Raw response:", response);
```

## Usage Examples

### Basic Profile Operations
```typescript
import { profileApi } from '@/lib/tauri';

// Get all profiles
const profiles = await profileApi.getProfiles();

// Create new profile
const newProfile = await profileApi.createProfile({
  name: "Development Workspace",
  description: "My coding setup",
  profileType: "development"
});

// Activate profile
await profileApi.activateProfile(newProfile.id);
```

### System Detection
```typescript
import { systemApi } from '@/lib/tauri';

// Capture current layout
const layout = await systemApi.captureCurrentLayout();
console.log(`Detected ${layout.monitors.length} monitors`);
console.log(`Found ${layout.windows.length} windows`);
console.log(`Running ${layout.runningApps.length} apps`);
```

### Environment-Aware Code
```typescript
import { isTauri, systemApi } from '@/lib/tauri';

if (isTauri()) {
  // Running in desktop app
  const monitors = await systemApi.getConnectedMonitors();
} else {
  // Running in browser - use mock data or show message
  console.log("System detection not available in browser");
}
```

## Type Safety Guarantees
- **Compile-time Checks**: TypeScript catches API misuse at build time
- **Runtime Validation**: Backend validates all inputs
- **Interface Consistency**: Types match Rust models exactly
- **CamelCase Conversion**: Automatic field name conversion

## Error Scenarios
- **Connection Failed**: Tauri backend not running
- **Command Not Found**: Handler not registered in Rust
- **Invalid Parameters**: Type mismatches or missing required fields
- **Permission Denied**: macOS access restrictions

## Performance Considerations
- **Lazy Loading**: APIs only called when needed
- **Promise-based**: Non-blocking async operations
- **Connection Reuse**: Tauri maintains persistent IPC connection
- **Minimal Serialization**: Efficient data transfer

## Testing Support
- **Mock Data**: Environment detection enables test mocking
- **Type Checking**: Compile-time API contract validation
- **Error Simulation**: Can test error handling paths
- **Browser Testing**: Full functionality testing in development

## Future Extensions
- **WebSocket Support**: Real-time updates from backend
- **Batch Operations**: Multiple API calls in single request
- **Caching Layer**: Response caching for frequently accessed data
- **Retry Logic**: Automatic retry for transient failures
- **Progress Callbacks**: Upload/download progress for large operations</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/lib/tauri.md