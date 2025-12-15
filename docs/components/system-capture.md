# components/system-capture.tsx - System Detection UI Component

## Overview
A comprehensive React component for detecting, visualizing, and capturing the current macOS system state including monitors, windows, and running applications. This component provides both real-time system monitoring and workspace capture functionality.

## Component Props
```typescript
interface SystemCaptureProps {
  onCaptureComplete?: (layout: unknown) => void  // Callback when capture completes
}
```

## Key Features
- **Real-time System Detection**: Monitors, windows, and applications
- **Visual Monitor Layout**: Graphical representation of display arrangement
- **Tabbed Interface**: Organized views for different system components
- **Capture Functionality**: Save current layout for workspace profiles
- **Error Handling**: User-friendly error display and recovery
- **Loading States**: Visual feedback during detection operations

## State Management
Uses the `useSystemDetection` hook for all system data:
```typescript
const {
  monitors,        // Array of detected monitors
  windows,         // Array of visible windows
  runningApps,     // Array of running applications
  isLoading,       // Loading state for operations
  error,          // Error state with message
  lastCapturedAt, // Timestamp of last successful capture
  refreshAll,     // Function to refresh all data
  captureLayout   // Function to capture complete layout
} = useSystemDetection()
```

## Component Structure

### Header Section
```tsx
<div className="flex items-center justify-between">
  <h3>System Capture</h3>
  <div className="flex gap-2">
    <Button onClick={refreshAll}>Refresh</Button>
    <Button onClick={handleCapture}>Capture Layout</Button>
  </div>
</div>
```

#### Refresh Button
- Triggers `refreshAll()` to update all system data
- Shows loading spinner during operation
- Disabled during loading

#### Capture Button
- Calls `captureLayout()` to save current state
- Triggers `onCaptureComplete` callback with captured data
- Shows capture confirmation animation

### Status Indicators

#### Error Display
```tsx
{error && (
  <motion.div className="text-red-500 bg-red-500/10 px-4 py-2 rounded-lg">
    <XCircle className="w-4 h-4" />
    {error.message}
  </motion.div>
)}
```
- Animated error messages with red styling
- Uses Framer Motion for smooth transitions

#### Capture Timestamp
```tsx
{lastCapturedAt && (
  <div className="text-xs text-muted-foreground">
    <CheckCircle2 className="w-3 h-3 text-green-500" />
    Last captured: {new Date(lastCapturedAt).toLocaleString()}
  </div>
)}
```
- Shows when last capture occurred
- Green checkmark for successful captures

### Summary Cards
Three metric cards showing counts:
- **Monitors**: Number of detected displays
- **Windows**: Number of visible application windows
- **Running Apps**: Number of running GUI applications

Each card includes:
- Count display with large typography
- Icon with themed background
- Consistent card styling

### Tabbed Interface
Uses Radix UI Tabs for organized content:

#### Monitors Tab
Contains two sections:

**Visual Layout Preview**
- Canvas showing relative monitor positions
- Scaled representation of display arrangement
- Primary monitor highlighted with different styling
- Monitor names and resolutions displayed

**Monitor Details List**
- Individual cards for each monitor
- Detailed specifications (resolution, refresh rate, scale factor)
- Position coordinates and orientation
- Primary/built-in badges

#### Windows Tab
Scrollable list of all visible windows:
- Window title and application name
- Bundle identifier
- Dimensions and position coordinates
- Status badges (minimized, fullscreen)
- Empty state when no windows detected

#### Apps Tab
Scrollable list of running applications:
- Application name with active/hidden badges
- Bundle identifier
- Window count
- Process ID
- Active application highlighting

## Visual Design

### Layout Calculations
```typescript
const monitorBounds = monitors.reduce((acc, m) => ({
  minX: Math.min(acc.minX, m.x),
  minY: Math.min(acc.minY, m.y),
  maxX: Math.max(acc.maxX, m.x + m.width),
  maxY: Math.max(acc.maxY, m.y + m.height),
}), { minX: 0, minY: 0, maxX: 800, maxY: 600 })
```
Calculates bounding box for proper scaling in preview.

### Scaling Logic
```typescript
const scale = Math.min(
  (400 - 40) / totalWidth,
  (180) / totalHeight
) * 0.8
```
Ensures monitors fit within preview area while maintaining proportions.

### Animations
- **Framer Motion**: Smooth transitions for errors and monitor appearances
- **Loading States**: Spinning refresh icon during operations
- **Hover Effects**: Interactive feedback on cards

## Error Handling
- Network/API errors displayed prominently
- Recovery through refresh functionality
- Graceful degradation when data unavailable
- User-friendly error messages

## Performance Optimizations
- **Conditional Rendering**: Tabs only render when active
- **Scroll Areas**: Virtualized scrolling for large lists
- **Memoized Calculations**: Layout bounds calculated efficiently
- **Lazy Loading**: Data fetched only when needed

## Accessibility
- **Semantic HTML**: Proper heading hierarchy
- **Keyboard Navigation**: Tab-based interface navigation
- **Screen Reader Support**: Descriptive labels and ARIA attributes
- **Color Contrast**: High contrast text and icons

## Integration Points
- **useSystemDetection Hook**: All data and actions come from this hook
- **Parent Callbacks**: `onCaptureComplete` for layout saving
- **Theme System**: Uses CSS custom properties for theming
- **UI Components**: Built on shadcn/ui component library

## Usage Example
```tsx
function WorkspaceManager() {
  const handleCaptureComplete = (layout) => {
    // Save layout to profile
    saveWorkspaceProfile(layout)
  }

  return (
    <SystemCapture onCaptureComplete={handleCaptureComplete} />
  )
}
```

## Future Enhancements
- **Real-time Updates**: Auto-refresh on system changes
- **Window Previews**: Thumbnails of window contents
- **Drag & Drop**: Interactive layout editing
- **Export Options**: Save layouts as images or JSON
- **Comparison Mode**: Diff between captures</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/components/system-capture.md