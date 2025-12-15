# app/page.tsx - Main Application Page

## Overview
The main page component that serves as the root of the Smoothie application. This client-side component manages the overall application layout and view routing between different sections of the workspace manager.

## Component Structure
```tsx
export default function Home() {
  const [currentView, setCurrentView] = useState<ViewType>("dashboard")
  const [currentProfile, setCurrentProfile] = useState<string | null>(null)

  return (
    <div className="flex h-screen bg-background text-foreground overflow-hidden">
      <Sidebar currentView={currentView} setCurrentView={setCurrentView} />
      <div className="flex-1 flex flex-col">
        <TopBar currentView={currentView} />
        <main className="flex-1 overflow-auto">
          {/* View routing logic */}
        </main>
      </div>
    </div>
  )
}
```

## State Management

### View State
```typescript
const [currentView, setCurrentView] = useState<ViewType>("dashboard")
```
Manages which main section is currently displayed. Available views:
- `"dashboard"` - Main dashboard with overview
- `"profiles"` - Profile management interface
- `"monitor"` - Monitor configuration editor
- `"capture"` - System capture interface
- `"settings"` - Application settings

### Profile Selection State
```typescript
const [currentProfile, setCurrentProfile] = useState<string | null>(null)
```
Tracks the currently selected profile ID, used for passing context between profile selection and monitor editing.

## Layout Structure

### Root Container
- **Full Height**: `h-screen` ensures the app fills the entire viewport
- **Background**: Uses CSS custom properties for theme support
- **Overflow Hidden**: Prevents unwanted scrolling on the root container

### Sidebar
```tsx
<Sidebar currentView={currentView} setCurrentView={setCurrentView} />
```
- Left navigation panel
- Receives current view state and view change handler
- Fixed width, contains navigation items

### Main Content Area
```tsx
<div className="flex-1 flex flex-col">
  <TopBar currentView={currentView} />
  <main className="flex-1 overflow-auto">
    {/* Dynamic content */}
  </main>
</div>
```

#### Top Bar
- Displays current view title and actions
- Receives current view for context-aware rendering

#### Main Content
- Flexible height (`flex-1`) to fill remaining space
- Scrollable (`overflow-auto`) for content that exceeds viewport
- Conditionally renders different components based on `currentView`

## View Routing Logic

### Dashboard View
```tsx
{currentView === "dashboard" && <Dashboard setCurrentView={setCurrentView} />}
```
- Default landing page
- Can navigate to other views via `setCurrentView`

### Profile Manager
```tsx
{currentView === "profiles" && <ProfileManager onSelectProfile={setCurrentProfile} />}
```
- CRUD interface for workspace profiles
- Updates `currentProfile` when a profile is selected

### Monitor Editor
```tsx
{currentView === "monitor" && <MonitorEditor selectedProfile={currentProfile} />}
```
- Monitor configuration interface
- Receives selected profile ID for context

### System Capture
```tsx
{currentView === "capture" && <div className="p-8"><SystemCapture /></div>}
```
- System detection and capture interface
- Wrapped in padding container for consistent spacing

### Settings
```tsx
{currentView === "settings" && <Settings />}
```
- Application configuration interface

## Component Dependencies
- `Sidebar` - Navigation component
- `TopBar` - Header component
- `Dashboard` - Main dashboard view
- `ProfileManager` - Profile CRUD interface
- `MonitorEditor` - Monitor configuration
- `SystemCapture` - System detection UI
- `Settings` - Settings panel

## Styling Approach
- **Tailwind CSS**: Utility-first styling
- **CSS Custom Properties**: Theme variables (`bg-background`, `text-foreground`)
- **Responsive Design**: Flexbox layout adapts to different screen sizes
- **Accessibility**: Semantic HTML structure

## Client-Side Rendering
Marked with `"use client"` directive because:
- Uses React state (`useState`)
- Handles user interactions
- Manages view routing client-side

## Performance Considerations
- **Lazy Loading**: Components are conditionally rendered
- **Minimal Re-renders**: State updates only affect relevant components
- **Efficient Layout**: CSS Grid/Flexbox for optimal rendering

## Navigation Flow
1. **Initial Load**: Shows dashboard
2. **User Navigation**: Sidebar clicks update `currentView`
3. **Profile Selection**: Profile manager updates `currentProfile`
4. **Context Passing**: Selected profile flows to monitor editor

## Future Enhancements
- **URL Routing**: Could integrate with Next.js router for deep linking
- **Breadcrumb Navigation**: For complex view hierarchies
- **View Transitions**: Smooth animations between views
- **Keyboard Shortcuts**: Quick view switching</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/frontend/page.md