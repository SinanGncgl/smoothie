# components/monitor-editor.tsx - Monitor Layout Editor

## Overview
An interactive React component for designing, arranging, and applying monitor layouts. Users can visually drag monitors to create custom workspace arrangements, save layouts to profiles, and apply configurations directly to the system display settings.

## Component Props
```typescript
interface MonitorEditorProps {
  selectedProfile: string | null  // Currently active profile ID
}
```

## Key Features
- **Visual Drag & Drop**: Interactive canvas for arranging monitor positions
- **Real-time Preview**: Scaled representation of monitor layout
- **Profile Integration**: Save/load layouts from workspace profiles
- **System Application**: Apply layouts directly to macOS display settings
- **Preset Layouts**: Quick setup for common configurations (dual, triple)
- **Monitor Management**: Add/remove monitors from layout
- **Detailed Monitor Info**: Display specifications and properties

## State Management
```typescript
const [monitors, setMonitors] = useState<Monitor[]>([])
const [profileMonitors, setProfileMonitors] = useState<Monitor[]>([])
const [isLoading, setIsLoading] = useState(false)
const [isSaving, setIsSaving] = useState(false)
const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)
const [draggingMonitor, setDraggingMonitor] = useState<string | null>(null)
const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 })
```

## Core Functionality

### Monitor Loading
```typescript
useEffect(() => {
  const loadProfileMonitors = async () => {
    if (!selectedProfile || !isTauri()) {
      // Use detected monitors as fallback
      const mappedMonitors = detectedMonitors.map(m => ({
        id: m.displayId.toString(),
        x: 50 + index * 350,
        y: 50,
        width: Math.min(m.width / 8, 320),
        height: Math.min(m.height / 8, 180),
        // ... other properties
      }))
      setMonitors(mappedMonitors)
      return
    }

    // Load from profile database
    const profileMonitorData = await monitorApi.getMonitors(selectedProfile)
    // Map and set profile monitors
  }
}, [selectedProfile, detectedMonitors])
```

### Drag & Drop System
```typescript
const handleMonitorMouseDown = (e: React.MouseEvent, monitorId: string) => {
  const monitor = monitors.find(m => m.id === monitorId)
  const rect = canvasRef.current?.getBoundingClientRect()
  setDraggingMonitor(monitorId)
  setDragOffset({
    x: e.clientX - rect.left - monitor.x,
    y: e.clientY - rect.top - monitor.y,
  })
}

const handleMouseMove = (e: React.MouseEvent) => {
  if (!draggingMonitor) return
  const rect = canvasRef.current.getBoundingClientRect()
  const newX = Math.max(0, e.clientX - rect.left - dragOffset.x)
  const newY = Math.max(0, e.clientY - rect.top - dragOffset.y)
  setMonitors(prev => prev.map(m =>
    m.id === draggingMonitor ? { ...m, x: newX, y: newY } : m
  ))
  setHasUnsavedChanges(true)
}
```

### Layout Saving
```typescript
const handleSaveLayout = async () => {
  for (const monitor of monitors) {
    if (existingMonitor) {
      await monitorApi.updateMonitor(monitor.id, {
        x: Math.round(monitor.x),
        y: Math.round(monitor.y),
        width: Math.round(monitor.width),
        height: Math.round(monitor.height),
      })
    } else {
      await monitorApi.createMonitor({
        profileId: selectedProfile,
        // ... monitor data
      })
    }
  }
  setHasUnsavedChanges(false)
}
```

### System Layout Application
```typescript
const handleApplyLayout = async () => {
  const systemMonitors = monitors.map(monitor => ({
    displayId: parseInt(monitor.id),
    name: monitor.name,
    resolution: monitor.resolution,
    width: monitor.width,
    height: monitor.height,
    x: monitor.x,
    y: monitor.y,
    // ... other system properties
  }))

  await systemApi.applyMonitorLayout(systemMonitors)
}
```

## UI Components

### Header Controls
- **Refresh Button**: Updates detected monitor information
- **Preset Buttons**: "Dual" and "Triple" layout shortcuts
- **Add Monitor**: Creates new monitor in layout
- **Save Layout**: Persists current arrangement to profile
- **Apply to System**: Changes actual display configuration

### Canvas Area
Interactive drag canvas with:
- **Monitor Representations**: Scaled visual monitors
- **Primary Monitor Highlighting**: Different styling for main display
- **Monitor Details**: Names, resolutions, specs displayed on monitors
- **Delete Buttons**: Remove monitors from layout

### Monitor Details Grid
Detailed cards showing:
- **Resolution & Position**: Technical specifications
- **Brand/Model**: Hardware identification
- **Refresh Rate & Scale**: Display properties
- **Orientation**: Landscape/portrait mode
- **Window Count**: Number of windows on each monitor

## System Integration

### displayplacer Utility
The "Apply to System" functionality uses the `displayplacer` command-line tool:

```bash
displayplacer "id:1 res:1920x1080 origin:(0,0) degree:0" "id:2 res:1920x1080 origin:(1920,0) degree:0"
```

**Requirements:**
- `displayplacer` must be installed (`brew install displayplacer`)
- May require administrator privileges
- Only works on macOS

### Monitor Data Mapping
Converts between internal `Monitor` format and system `SystemMonitor` format:
```typescript
const systemMonitor = {
  displayId: parseInt(monitor.id),
  name: monitor.name,
  resolution: monitor.resolution,
  width: monitor.width,
  height: monitor.height,
  x: monitor.x,
  y: monitor.y,
  scaleFactor: monitor.scaleFactor || 1.0,
  refreshRate: monitor.refreshRate || 60.0,
  isPrimary: monitor.isPrimary,
  isBuiltin: monitor.isBuiltin || false,
  orientation: monitor.orientation || "Landscape",
}
```

## Visual Design

### Canvas Scaling
```typescript
const scale = Math.min(
  canvasWidth / totalLayoutWidth,
  canvasHeight / totalLayoutHeight
) * 0.8
```
Maintains proportions while fitting monitors in preview area.

### Monitor Styling
- **Primary Monitors**: Blue border and background tint
- **Regular Monitors**: Gray borders with subtle backgrounds
- **Dragging State**: Elevated shadows and z-index changes
- **Hover Effects**: Shadow and cursor changes

### Animations
- **Framer Motion**: Smooth monitor appearance animations
- **Loading States**: Spinner animations during operations
- **Drag Feedback**: Visual feedback during interactions

## Error Handling
- **API Errors**: Toast notifications for failed operations
- **Validation**: Checks for profile selection and Tauri environment
- **Fallbacks**: Graceful degradation when features unavailable
- **User Feedback**: Clear success/error messaging

## Performance Considerations
- **Efficient Updates**: Only re-renders changed monitors
- **Debounced Operations**: Prevents excessive API calls
- **Memory Management**: Proper cleanup of event listeners
- **Canvas Optimization**: Hardware-accelerated rendering

## Accessibility
- **Keyboard Navigation**: Focus management for interactive elements
- **Screen Reader Support**: Descriptive labels and ARIA attributes
- **Color Contrast**: High contrast for monitor boundaries
- **Motion Preferences**: Respects user animation preferences

## Integration Points
- **useSystemDetection Hook**: Provides detected monitor data
- **monitorApi**: Database operations for profile persistence
- **systemApi**: System-level monitor configuration
- **useToast**: User notifications and feedback
- **Profile System**: Integration with workspace profiles

## Usage Example
```tsx
function Dashboard() {
  const [selectedProfile, setSelectedProfile] = useState<string | null>(null)

  return (
    <MonitorEditor selectedProfile={selectedProfile} />
  )
}
```

## Future Enhancements
- **Live Preview**: Real-time system layout updates
- **Rotation Support**: Monitor orientation changes
- **Resolution Switching**: Dynamic resolution adjustment
- **Multi-Display Detection**: Better handling of complex setups
- **Layout Templates**: Saved layout presets
- **Undo/Redo**: Layout editing history
- **Import/Export**: Layout file operations</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/components/monitor-editor.md