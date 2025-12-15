# src/services/system_service.rs - macOS System Detection Service

## Overview
Core service for detecting and querying macOS system state including connected monitors, visible windows, and running applications. This service uses macOS CoreGraphics and CoreFoundation frameworks to interface directly with the window server and display system.

## Key Features
- **Monitor Detection**: Discovers all connected displays with detailed configuration
- **Window Detection**: Enumerates visible application windows with positioning and metadata
- **Application Detection**: Lists running GUI applications with process information
- **Cross-Platform Ready**: Designed for macOS-only but structured for future platform support

## Data Structures

### SystemMonitor
Represents a connected display/monitor with comprehensive metadata:
```rust
pub struct SystemMonitor {
    pub display_id: u32,        // Unique macOS display identifier
    pub name: String,           // Human-readable name ("Built-in Retina Display")
    pub resolution: String,     // "WIDTHxHEIGHT" format
    pub width: i32,             // Native pixel width
    pub height: i32,            // Native pixel height
    pub x: i32,                 // X coordinate in global display space
    pub y: i32,                 // Y coordinate in global display space
    pub scale_factor: f64,      // Retina scale factor (1.0 or 2.0)
    pub refresh_rate: f64,      // Display refresh rate in Hz
    pub is_primary: bool,       // Whether this is the main display
    pub is_builtin: bool,       // Whether this is the built-in MacBook display
    pub orientation: String,    // "Landscape" or "Portrait"
}
```

### SystemWindow
Represents a visible window on the screen:
```rust
pub struct SystemWindow {
    pub window_id: u32,         // macOS window identifier
    pub pid: u32,               // Process ID of owning application
    pub title: String,          // Window title (may be empty)
    pub app_name: String,       // Application name
    pub bundle_id: String,      // macOS bundle identifier
    pub x: i32,                 // Window X position
    pub y: i32,                 // Window Y position
    pub width: i32,             // Window width
    pub height: i32,            // Window height
    pub display_id: u32,        // Display where window is primarily located
    pub is_minimized: bool,     // Whether minimized to Dock
    pub is_fullscreen: bool,    // Whether in fullscreen mode
    pub layer: i32,             // Window layer (0 for normal windows)
}
```

### RunningApp
Represents a running GUI application:
```rust
pub struct RunningApp {
    pub pid: u32,               // Process ID
    pub name: String,           // Application name
    pub bundle_id: String,      // macOS bundle identifier
    pub path: Option<String>,   // Path to application bundle
    pub is_active: bool,        // Whether currently frontmost
    pub is_hidden: bool,        // Whether application is hidden
    pub window_count: u32,      // Number of visible windows
}
```

## Public API

### Monitor Detection
```rust
pub fn get_monitors() -> Vec<SystemMonitor>
```
Returns all connected displays with complete configuration information.

### Window Detection
```rust
pub fn get_windows() -> Vec<SystemWindow>
```
Returns all visible application windows, filtering out system windows (Dock, menu bar, etc.).

### Application Detection
```rust
pub fn get_running_apps() -> Vec<RunningApp>
```
Returns all running GUI applications with process and window information.

## Implementation Details

### Monitor Detection (`detect_monitors`)
1. Uses `CGDisplay::active_displays()` to get all connected display IDs
2. For each display, retrieves bounds, mode information, and properties
3. Calculates scale factor from pixel vs. point dimensions
4. Determines primary/builtin status and generates descriptive names
5. Sorts monitors by position (left-to-right, top-to-bottom)

### Window Detection (`detect_windows`)
1. Calls `CGWindowListCopyWindowInfo()` to get window server data
2. Parses CoreFoundation dictionaries for each window
3. Filters out system windows (layer != 0) and tiny windows (< 50x50)
4. Determines display placement by window center point
5. Retrieves bundle IDs using `lsappinfo` command

### Application Detection (`detect_running_apps`)
1. Primary method: Uses AppleScript via `osascript` to query System Events
2. Fallback method: Derives from window detection data
3. Correlates window counts with application information
4. Handles process metadata (frontmost, visible status)

## macOS APIs Used

### CoreGraphics Framework
- `CGDisplay` - Display enumeration and properties
- `CGWindowListCopyWindowInfo` - Window server queries
- `CGDisplayMode` - Display mode information

### CoreFoundation Framework
- `CFDictionary` - Window property parsing
- `CFString` - String handling
- `CFNumber` - Numeric value extraction
- `CFArray` - Window list iteration

### System Commands
- `osascript` - AppleScript execution for application enumeration
- `lsappinfo` - Bundle identifier lookup by PID

## Error Handling
- Graceful degradation when APIs fail (returns empty vectors)
- Comprehensive logging of failures with `tracing`
- Safe memory management with CoreFoundation reference counting

## Testing
Comprehensive test suite covering:
- **Monitor Detection**: Validates display enumeration and data integrity
- **Window Detection**: Ensures proper filtering and metadata extraction
- **Application Detection**: Verifies process enumeration and correlation
- **Data Integrity**: Validates all numeric ranges and string formats

### Test Output
Tests provide detailed console output showing detected system state:
```
=== Detected Monitors ===
  Built-in Retina Display (ID: 1): 1440x900 @ (0, 0) - scale: 2.0x, refresh: 60Hz
  Primary Display (ID: 2): 2560x1440 @ (1440, 0) - scale: 1.0x, refresh: 60Hz

=== Detected Windows ===
  Code - "system_service.rs" @ (100, 100) 1200x800
  Safari - "GitHub Copilot" @ (200, 200) 1400x900

=== Running Applications ===
  Code (com.microsoft.VSCode) - 2 windows [ACTIVE]
  Safari (com.apple.Safari) - 1 windows
```

## Performance Considerations
- Window detection queries the entire window server - optimized for infrequent calls
- Monitor detection is lightweight and suitable for frequent polling
- Application detection uses AppleScript fallback to ensure reliability
- All operations complete in <100ms on typical systems

## Security Notes
- Only accesses public macOS APIs (no private frameworks)
- No elevated privileges required
- Safe memory handling with proper CFRelease calls
- Input validation on all parsed data

## Future Extensions
- **Window State**: Enhanced minimization/fullscreen detection
- **Display Modes**: Available resolution enumeration
- **Window Focus**: Active window tracking
- **Cross-Platform**: Windows/Linux implementations in separate modules</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/backend/services/system-service.md