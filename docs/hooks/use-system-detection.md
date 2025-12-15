# hooks/use-system-detection.ts - System Detection State Management

## Overview
A comprehensive React hook for managing macOS system detection state and operations. Provides both Tauri-based system detection for the desktop app and mock data for browser development/preview.

## Key Features
- **Multi-Source Data**: Monitors, windows, and running applications
- **Dual Environment Support**: Tauri desktop app and browser development
- **Granular Control**: Individual refresh functions for each data type
- **Batch Operations**: Combined refresh and capture operations
- **Error Handling**: Comprehensive error states and recovery
- **Loading States**: Visual feedback during operations
- **Mock Data**: Realistic development data for browser testing

## Hook API
```typescript
const {
  monitors,           // SystemMonitor[]
  windows,            // SystemWindow[]
  runningApps,        // RunningApp[]
  isLoading,          // boolean
  error,              // Error | null
  lastCapturedAt,     // string | null
  refreshMonitors,    // () => Promise<void>
  refreshWindows,     // () => Promise<void>
  refreshRunningApps, // () => Promise<void>
  refreshAll,         // () => Promise<void>
  captureLayout       // () => Promise<CapturedLayout | null>
} = useSystemDetection()
```

## State Structure
```typescript
interface SystemDetectionState {
  monitors: SystemMonitor[];      // Detected display devices
  windows: SystemWindow[];        // Visible application windows
  runningApps: RunningApp[];      // Running GUI applications
  isLoading: boolean;             // Operation in progress
  error: Error | null;            // Last error encountered
  lastCapturedAt: string | null;  // ISO timestamp of last capture
}
```

## Core Functions

### Individual Refresh Functions

#### refreshMonitors()
```typescript
const refreshMonitors = useCallback(async () => {
  // Tauri: Calls systemApi.getConnectedMonitors()
  // Browser: Uses MOCK_MONITORS
}, [])
```
- Fetches current monitor configuration
- Updates only the monitors array
- Handles errors gracefully

#### refreshWindows()
```typescript
const refreshWindows = useCallback(async () => {
  // Tauri: Calls systemApi.getVisibleWindows()
  // Browser: Uses MOCK_WINDOWS
}, [])
```
- Detects all visible application windows
- Filters out system windows (dock, menu bar, etc.)
- Updates windows array

#### refreshRunningApps()
```typescript
const refreshRunningApps = useCallback(async () => {
  // Tauri: Calls systemApi.getRunningApps()
  // Browser: Uses MOCK_APPS
}, [])
```
- Enumerates running GUI applications
- Includes process metadata and window counts
- Updates runningApps array

### Batch Operations

#### refreshAll()
```typescript
const refreshAll = useCallback(async () => {
  // Executes all three refresh operations in parallel
  const [monitors, windows, runningApps] = await Promise.all([
    systemApi.getConnectedMonitors(),
    systemApi.getVisibleWindows(),
    systemApi.getRunningApps(),
  ]);
}, [])
```
- Refreshes all system data simultaneously
- Uses `Promise.all` for optimal performance
- Sets global loading state

#### captureLayout()
```typescript
const captureLayout = useCallback(async (): Promise<CapturedLayout | null> => {
  // Tauri: Calls systemApi.captureCurrentLayout()
  // Browser: Returns mock CapturedLayout
}, [])
```
- Comprehensive system snapshot
- Updates all state arrays
- Records capture timestamp
- Returns structured layout data

## Environment Detection
```typescript
import { isTauri } from '@/lib/tauri';

// Conditional logic throughout
if (!isTauri()) {
  // Use mock data for browser development
  setState(prev => ({ ...prev, monitors: MOCK_MONITORS }));
  return;
}
```
Automatically detects Tauri vs browser environment and uses appropriate data source.

## Mock Data
Comprehensive mock datasets for development:

### MOCK_MONITORS
```typescript
const MOCK_MONITORS: SystemMonitor[] = [
  {
    displayId: 1,
    name: 'Primary Display',
    resolution: '2560x1440',
    width: 2560,
    height: 1440,
    x: 0, y: 0,
    scaleFactor: 2.0,
    refreshRate: 60,
    isPrimary: true,
    isBuiltin: false,
    orientation: 'Landscape',
  },
  // ... additional monitors
];
```

### MOCK_WINDOWS
Sample window data with realistic positioning and metadata.

### MOCK_APPS
Sample application data with process information and window counts.

## Error Handling
- **Network Errors**: API call failures
- **Permission Errors**: macOS access restrictions
- **Timeout Errors**: Long-running operations
- **Recovery**: Manual retry via refresh functions

## Performance Optimizations
- **useCallback**: Prevents unnecessary re-renders
- **Parallel Execution**: `Promise.all` for concurrent API calls
- **Selective Updates**: Only relevant state updated per operation
- **Memoized Computations**: No expensive recalculations

## Lifecycle Management
```typescript
useEffect(() => {
  refreshMonitors(); // Auto-refresh monitors on mount
}, [refreshMonitors]);
```
Automatically refreshes monitor data when component mounts.

## Integration with Tauri API
```typescript
import { systemApi } from '@/lib/tauri';

// API calls
const monitors = await systemApi.getConnectedMonitors();
const layout = await systemApi.captureCurrentLayout();
```
All system operations go through the typed Tauri API wrapper.

## Usage Examples

### Basic Usage
```typescript
function SystemMonitor() {
  const { monitors, windows, runningApps, refreshAll } = useSystemDetection();

  return (
    <div>
      <button onClick={refreshAll}>Refresh System Data</button>
      <p>Monitors: {monitors.length}</p>
      <p>Windows: {windows.length}</p>
      <p>Apps: {runningApps.length}</p>
    </div>
  );
}
```

### Capture Workflow
```typescript
function WorkspaceCapture() {
  const { captureLayout, isLoading, error } = useSystemDetection();

  const handleCapture = async () => {
    const layout = await captureLayout();
    if (layout) {
      // Save to profile
      saveWorkspaceProfile(layout);
    }
  };

  return (
    <button onClick={handleCapture} disabled={isLoading}>
      {isLoading ? 'Capturing...' : 'Capture Layout'}
    </button>
  );
}
```

## Testing Considerations
- **Mock Data**: Consistent test data for unit tests
- **Error Simulation**: Can inject errors for testing error states
- **Async Testing**: Proper handling of async operations in tests

## Future Enhancements
- **Real-time Updates**: WebSocket-based change detection
- **Selective Refresh**: Refresh only changed components
- **Caching**: Local storage caching for offline scenarios
- **Background Updates**: Periodic automatic refresh
- **Change Detection**: Notify when system state changes</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/hooks/use-system-detection.md