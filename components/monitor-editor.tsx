"use client"

import type React from "react"
import { useState, useRef, useEffect, useCallback } from "react"
import { useMemo } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { AlertDialog, AlertDialogAction, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/components/ui/alert-dialog"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Plus, Trash2, Maximize2, Grid3X3, RefreshCw, Save, Loader2, Copy, Terminal, ZoomIn, ZoomOut } from "lucide-react"
import { motion } from "framer-motion"
import { useSystemDetection } from "@/hooks/use-system-detection"
import { monitorApi, isTauri, systemApi, profileApi, type Profile } from "@/lib/tauri"
import { useToast } from "@/hooks/use-toast"

// Internal monitor representation uses ACTUAL pixel coordinates
interface Monitor {
  id: string
  x: number  // Actual pixel x
  y: number  // Actual pixel y
  width: number  // Actual pixel width
  height: number  // Actual pixel height
  name: string
  resolution: string
  isPrimary: boolean
  refreshRate?: number
  scaleFactor?: number
  isBuiltin?: boolean
  orientation?: string
  brand?: string
  model?: string
  windows: Window[]
}

interface Window {
  id: string
  x: number
  y: number
  width: number
  height: number
  app: string
}

interface MonitorEditorProps {
  selectedProfile: string | null
  onSelectProfile?: (profileId: string | null) => void
}

export function MonitorEditor({ selectedProfile, onSelectProfile }: MonitorEditorProps) {
  const canvasRef = useRef<HTMLDivElement>(null)
  const { monitors: detectedMonitors, refreshMonitors, isLoading: systemLoading } = useSystemDetection()
  const [monitors, setMonitors] = useState<Monitor[]>([])
  const [profileMonitors, setProfileMonitors] = useState<Monitor[]>([])
  const [profiles, setProfiles] = useState<Profile[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [isSaving, setIsSaving] = useState(false)
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)
  const [manualCommand, setManualCommand] = useState<string | null>(null)
  const [permissionNeeded, setPermissionNeeded] = useState<boolean>(false)
  const { toast } = useToast()

  const [draggingMonitor, setDraggingMonitor] = useState<string | null>(null)
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 })
  
  // Canvas dimensions for responsive scaling
  const [canvasSize, setCanvasSize] = useState({ width: 800, height: 400 })
  // Slightly smaller default scale so more monitors fit when canvas is smaller
  const [displayScale, setDisplayScale] = useState(0.14) // Scale factor for display (smaller default so more fits)
  const [lockedScale, setLockedScale] = useState<number | null>(null) // Lock scale during drag
  const [lockedOffsets, setLockedOffsets] = useState<{ offsetX: number; offsetY: number } | null>(null) // Lock offsets during drag
  
  // Use locked scale during drag to prevent monitors from resizing
  const activeScale = lockedScale ?? displayScale

  // Whether the canvas is expanded (bigger view)
  const [canvasExpanded, setCanvasExpanded] = useState(false)
  // Flag to prevent auto-recalculation when manually zoomed
  const [manualZoom, setManualZoom] = useState(false)

  // Load profiles for the selector
  useEffect(() => {
    const loadProfiles = async () => {
      if (!isTauri()) return
      try {
        const profileList = await profileApi.getProfiles()
        setProfiles(profileList)
      } catch {
        // Silent failure for profile loading
      }
    }
    loadProfiles()
  }, [])

  // Calculate the display scale based on monitors and canvas size
  const calculateDisplayScale = useCallback(() => {
    if (monitors.length === 0 || !canvasRef.current) return 0.15
    
    const canvas = canvasRef.current
    const canvasWidth = canvas.clientWidth - 32 // smaller padding so monitors fit tighter
    const canvasHeight = canvas.clientHeight - 32
    
    // Find the bounding box of all monitors
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
    monitors.forEach(m => {
      minX = Math.min(minX, m.x)
      minY = Math.min(minY, m.y)
      maxX = Math.max(maxX, m.x + m.width)
      maxY = Math.max(maxY, m.y + m.height)
    })
    
    const totalWidth = maxX - minX
    const totalHeight = maxY - minY
    
    // Calculate scale to fit both dimensions with some margin
    const scaleX = canvasWidth / totalWidth
    const scaleY = canvasHeight / totalHeight
    // Allow a larger cap so monitors render bigger in smaller windows
    const scale = Math.min(scaleX, scaleY, 0.28) // Cap at 0.28 for better visibility on a smaller canvas

    return Math.max(scale, 0.05) // Minimum scale
  }, [monitors])

  // compute offsets so monitors are centered and don't leave large empty margins
  const canvasOffsets = useMemo(() => {
    if (monitors.length === 0) return { offsetX: 0, offsetY: 0 }

    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
    monitors.forEach(m => {
      minX = Math.min(minX, m.x)
      minY = Math.min(minY, m.y)
      maxX = Math.max(maxX, m.x + m.width)
      maxY = Math.max(maxY, m.y + m.height)
    })

    const totalWidth = maxX - minX
    const totalHeight = maxY - minY

    const scaledWidth = totalWidth * activeScale
    const scaledHeight = totalHeight * activeScale

    const canvasW = canvasSize.width || 0
    const canvasH = canvasSize.height || 0

    const padding = 12
    const offsetX = Math.max(padding, (canvasW - scaledWidth) / 2) - (minX * activeScale)
    const offsetY = Math.max(padding, (canvasH - scaledHeight) / 2) - (minY * activeScale)

    return { offsetX, offsetY }
  }, [monitors, canvasSize, activeScale])

  // Update canvas size on resize
  useEffect(() => {
    const updateCanvasSize = () => {
      if (canvasRef.current) {
        setCanvasSize({
          width: canvasRef.current.clientWidth,
          height: canvasRef.current.clientHeight
        })
      }
    }
    
    updateCanvasSize()
    window.addEventListener('resize', updateCanvasSize)
    
    // Use ResizeObserver for more accurate tracking
    const observer = new ResizeObserver(updateCanvasSize)
    if (canvasRef.current) {
      observer.observe(canvasRef.current)
    }
    
    return () => {
      window.removeEventListener('resize', updateCanvasSize)
      observer.disconnect()
    }
  }, [])

  // Recalculate display scale when monitors or canvas size changes (only when not dragging and not manually zoomed)
  useEffect(() => {
    if (lockedScale === null && !manualZoom) {
      const newScale = calculateDisplayScale()
      setDisplayScale(newScale)
    }
  }, [monitors, canvasSize, calculateDisplayScale, lockedScale, manualZoom])

  // compute bounding box for monitors so we can pick a tighter canvas height
  const monitorsBounds = useMemo(() => {
    if (monitors.length === 0) return { width: 0, height: 0, minY: 0, maxY: 0 }

    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
    monitors.forEach(m => {
      minX = Math.min(minX, m.x)
      minY = Math.min(minY, m.y)
      maxX = Math.max(maxX, m.x + m.width)
      maxY = Math.max(maxY, m.y + m.height)
    })

    return { width: maxX - minX, height: maxY - minY, minY, maxY }
  }, [monitors])

  // pick a canvas min-height that fits the scaled monitors tightly but still allows padding
  const desiredCanvasMinHeight = useMemo(() => {
    const padding = 48 // top/bottom padding in px
    const scaledHeight = Math.max(0, monitorsBounds.height * activeScale)
    // prefer a small min height when there are no monitors
    const base = scaledHeight > 0 ? Math.ceil(scaledHeight + padding) : 180
    // cap to a reasonable value so it never grows too tall
    const cap = Math.max(180, Math.min(base, window?.innerHeight ? Math.round(window.innerHeight * 0.75) : 900))
    return cap
  }, [monitorsBounds, activeScale])

  // Load monitors from profile when selectedProfile changes
  useEffect(() => {
    const loadProfileMonitors = async () => {
      if (!selectedProfile || !isTauri()) {
        // If no profile selected or not in Tauri, use detected monitors (actual pixel coords)
        if (detectedMonitors.length > 0) {
          const mappedMonitors: Monitor[] = detectedMonitors.map((m) => ({
            id: m.displayId.toString(),
            x: m.x,
            y: m.y,
            width: m.width,
            height: m.height,
            name: m.name,
            resolution: m.resolution,
            isPrimary: m.isPrimary,
            refreshRate: m.refreshRate,
            scaleFactor: m.scaleFactor,
            isBuiltin: m.isBuiltin,
            orientation: m.orientation,
            brand: m.brand,
            model: m.model,
            windows: [],
          }))
          setMonitors(mappedMonitors)
          setProfileMonitors([])
        }
        return
      }

      setIsLoading(true)
      try {
        const profileMonitorData = await monitorApi.getMonitors(selectedProfile)
        if (profileMonitorData && profileMonitorData.length > 0) {
          // Load existing profile monitors (already in actual pixel coords)
          const mappedProfileMonitors: Monitor[] = profileMonitorData.map((m: any) => ({
            id: m.id,
            x: m.x,
            y: m.y,
            width: m.width,
            height: m.height,
            name: m.name,
            resolution: m.resolution,
            isPrimary: m.isPrimary,
            refreshRate: undefined,
            scaleFactor: undefined,
            isBuiltin: undefined,
            orientation: m.orientation,
            brand: undefined,
            model: undefined,
            windows: [],
          }))
          setMonitors(mappedProfileMonitors)
          setProfileMonitors(mappedProfileMonitors)
          setHasUnsavedChanges(false)
        } else {
          // No profile monitors, use detected monitors as base (actual pixel coordinates)
          if (detectedMonitors.length > 0) {
            const mappedMonitors: Monitor[] = detectedMonitors.map((m) => ({
              id: m.displayId.toString(),
              x: m.x,
              y: m.y,
              width: m.width,
              height: m.height,
              name: m.name,
              resolution: m.resolution,
              isPrimary: m.isPrimary,
              refreshRate: m.refreshRate,
              scaleFactor: m.scaleFactor,
              isBuiltin: m.isBuiltin,
              orientation: m.orientation,
              brand: m.brand,
              model: m.model,
              windows: [],
            }))
            setMonitors(mappedMonitors)
            setProfileMonitors([])
            // Mark as unsaved since profile has no monitors but we have detected ones
            setHasUnsavedChanges(true)
          } else {
            setHasUnsavedChanges(false)
          }
        }
      } catch {
        toast({
          title: "Error",
          description: "Failed to load monitor layout from profile",
          variant: "destructive",
        })
        // Fallback to detected monitors (actual pixel coordinates)
        if (detectedMonitors.length > 0) {
          const mappedMonitors: Monitor[] = detectedMonitors.map((m) => ({
            id: m.displayId.toString(),
            x: m.x,
            y: m.y,
            width: m.width,
            height: m.height,
            name: m.name,
            resolution: m.resolution,
            isPrimary: m.isPrimary,
            refreshRate: m.refreshRate,
            scaleFactor: m.scaleFactor,
            isBuiltin: m.isBuiltin,
            orientation: m.orientation,
            brand: m.brand,
            model: m.model,
            windows: [],
          }))
          setMonitors(mappedMonitors)
          setProfileMonitors([])
        }
      } finally {
        setIsLoading(false)
      }
    }

    loadProfileMonitors()
  }, [selectedProfile, detectedMonitors, toast])

  // Refresh detected monitors
  const handleRefreshMonitors = () => {
    refreshMonitors()
  }

  // Apply monitor layout to system
  const handleApplyLayout = async () => {
    if (!isTauri()) {
      toast({
        title: "Error",
        description: "This feature is only available in the Tauri app",
        variant: "destructive",
      })
      return
    }

    // Validate monitor data
    if (monitors.length === 0) {
      toast({
        title: "Error",
        description: "No monitors to apply layout for",
        variant: "destructive",
      })
      return
    }

    setIsSaving(true)
    try {
      // Monitors are already stored in actual pixel coordinates
      const systemMonitors = monitors.map(monitor => ({
        displayId: parseInt(monitor.id),
        name: monitor.name,
        brand: monitor.brand,
        model: monitor.model,
        resolution: monitor.resolution,
        width: Math.round(monitor.width),
        height: Math.round(monitor.height),
        x: Math.round(monitor.x),
        y: Math.round(monitor.y),
        scaleFactor: monitor.scaleFactor || 1.0,
        refreshRate: monitor.refreshRate || 60.0,
        isPrimary: monitor.isPrimary,
        isBuiltin: monitor.isBuiltin || false,
        orientation: monitor.orientation || "Landscape",
      }))

      const result = await systemApi.applyMonitorLayout(systemMonitors)
      
      // Check if permission is required
      if (result && typeof result === 'string' && result.startsWith('PERMISSION_REQUIRED:')) {
        setPermissionNeeded(true)
        return
      }
      
      // Check if the response indicates manual command is needed
      if (result && typeof result === 'string' && result.startsWith('MANUAL_COMMAND:')) {
        const manualCommandText = result.substring('MANUAL_COMMAND:'.length)
        setManualCommand(manualCommandText)
      } else {
        toast({
          title: "Success",
          description: "Monitor layout applied to system successfully!",
        })
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error)
      
      // Check if this is a manual command error
      if (errorMessage.includes("Please run this command manually")) {
        setManualCommand(errorMessage)
      } else {
        toast({
          title: "Failed to Apply Layout",
          description: `Error: ${errorMessage}. Make sure displayplacer is installed and you have sufficient permissions.`,
          variant: "destructive",
        })
      }
    } finally {
      setIsSaving(false)
    }
  }

  // Save monitor layout to profile
  const handleSaveLayout = async () => {
    if (!selectedProfile || !isTauri()) {
      toast({
        title: "Error",
        description: "No profile selected or not running in Tauri app",
        variant: "destructive",
      })
      return
    }

    setIsSaving(true)
    try {
      // Monitors are already stored in actual pixel coordinates
      for (const monitor of monitors) {
        const existingMonitor = profileMonitors.find(pm => pm.id === monitor.id)
        
        if (existingMonitor) {
          // Update existing monitor
          await monitorApi.updateMonitor(monitor.id, {
            x: Math.round(monitor.x),
            y: Math.round(monitor.y),
            width: Math.round(monitor.width),
            height: Math.round(monitor.height),
          })
        } else {
          // Create new monitor
          await monitorApi.createMonitor({
            profileId: selectedProfile,
            name: monitor.name,
            resolution: monitor.resolution,
            orientation: monitor.orientation || "Landscape",
            isPrimary: monitor.isPrimary,
            x: Math.round(monitor.x),
            y: Math.round(monitor.y),
            width: Math.round(monitor.width),
            height: Math.round(monitor.height),
            displayIndex: monitors.indexOf(monitor),
          })
        }
      }

      // Remove monitors that were deleted
      for (const profileMonitor of profileMonitors) {
        if (!monitors.find(m => m.id === profileMonitor.id)) {
          await monitorApi.deleteMonitor(profileMonitor.id)
        }
      }

      setProfileMonitors([...monitors])
      setHasUnsavedChanges(false)
      toast({
        title: "Success",
        description: "Monitor layout saved to profile",
      })
    } catch {
      toast({
        title: "Error",
        description: "Failed to save monitor layout",
        variant: "destructive",
      })
    } finally {
      setIsSaving(false)
    }
  }

  const applyPresetLayout = (preset: string) => {
    let newMonitors: Monitor[] = []
    const spacing = 50 // Spacing in actual pixels between monitors

    if (preset === "dual-side") {
      let currentX = 0
      newMonitors = monitors.map((monitor, index) => {
        const result = {
          ...monitor,
          x: currentX,
          y: 0,
        }
        currentX += monitor.width + spacing
        return result
      })
    } else if (preset === "dual-vertical") {
      let currentY = 0
      newMonitors = monitors.slice(0, 2).map((monitor, index) => {
        const result = {
          ...monitor,
          x: 0,
          y: currentY,
        }
        currentY += monitor.height + spacing
        return result
      })
    } else if (preset === "triple") {
      let currentX = 0
      newMonitors = monitors.slice(0, 3).map((monitor, index) => {
        const result = {
          ...monitor,
          x: currentX,
          y: 0,
        }
        currentX += monitor.width + spacing
        return result
      })
      // Add a third monitor if we don't have enough
      if (newMonitors.length < 3) {
        newMonitors.push({
          id: "3",
          x: currentX,
          y: 0,
          width: 1920, // Actual pixel dimensions
          height: 1080,
          name: "Monitor 3",
          resolution: "1920×1080",
          isPrimary: false,
          windows: [],
        })
      }
    }

    setMonitors(newMonitors)
    setHasUnsavedChanges(true)
  }

  const handleMonitorMouseDown = (e: React.MouseEvent, monitorId: string) => {
    const monitor = monitors.find((m) => m.id === monitorId)
    if (!monitor) return

    const rect = canvasRef.current?.getBoundingClientRect()
    if (!rect) return

    // Lock the scale and offsets when starting to drag
    const scaleToLock = displayScale
    setLockedScale(scaleToLock)
    setLockedOffsets(canvasOffsets)
    setDraggingMonitor(monitorId)

    // Calculate anchor inside the monitor in monitor-space so movement is consistent
    const offsetsAtDown = canvasOffsets
    const anchorX = (e.clientX - rect.left - offsetsAtDown.offsetX) / scaleToLock - monitor.x
    const anchorY = (e.clientY - rect.top - offsetsAtDown.offsetY) / scaleToLock - monitor.y
    setDragOffset({ x: anchorX, y: anchorY })
  }

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!draggingMonitor || !canvasRef.current || lockedScale === null) return

    const rect = canvasRef.current.getBoundingClientRect()
    // use locked offsets while dragging (so center doesn't jump)
    const offsetsAtDrag = lockedOffsets ?? canvasOffsets

    // pointer position in monitor (actual pixel) coordinates
    const pointerXMon = (e.clientX - rect.left - offsetsAtDrag.offsetX) / lockedScale
    const pointerYMon = (e.clientY - rect.top - offsetsAtDrag.offsetY) / lockedScale

    const newX = pointerXMon - dragOffset.x
    const newY = pointerYMon - dragOffset.y

    setMonitors(monitors.map((m) => (m.id === draggingMonitor ? { ...m, x: newX, y: newY } : m)))
    setHasUnsavedChanges(true)
  }

  const handleMouseUp = () => {
    setDraggingMonitor(null)
    // Unlock scale after drag ends - scale will recalculate on next effect run
    setLockedScale(null)
    setLockedOffsets(null)
  }

  const handleAddMonitor = () => {
    // Calculate position based on existing monitors (actual pixel coordinates)
    let maxX = 0
    monitors.forEach(m => {
      const rightEdge = m.x + m.width + 50 // 50px spacing
      if (rightEdge > maxX) maxX = rightEdge
    })
    
    const newMonitor: Monitor = {
      id: Date.now().toString(),
      x: maxX,
      y: 0,
      width: 1920, // Actual pixel dimensions
      height: 1080,
      name: `Monitor ${monitors.length + 1}`,
      resolution: "1920×1080",
      isPrimary: false,
      windows: [],
    }
    setMonitors([...monitors, newMonitor])
    setHasUnsavedChanges(true)
  }

  const handleDeleteMonitor = (id: string) => {
    setMonitors(monitors.filter((m) => m.id !== id))
    setHasUnsavedChanges(true)
  }

  return (
    <div className="h-full flex flex-col p-8 gap-8">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div>
            <h3 className="text-lg font-semibold text-foreground">Monitor Layout</h3>
            <p className="text-sm text-muted-foreground">
              {selectedProfile ? "Drag to arrange monitors and design your workspace" : "Select a profile to save layout"}
            </p>
          </div>
          {/* Profile Selector */}
          <Select
            value={selectedProfile || ""}
            onValueChange={(value) => onSelectProfile?.(value || null)}
          >
            <SelectTrigger className="w-[200px]">
              <SelectValue placeholder="Select a profile" />
            </SelectTrigger>
            <SelectContent>
              {profiles.map((profile) => (
                <SelectItem key={profile.id} value={profile.id}>
                  {profile.name}
                </SelectItem>
              ))}
              {profiles.length === 0 && (
                <SelectItem value="__no_profiles__" disabled>
                  No profiles available
                </SelectItem>
              )}
            </SelectContent>
          </Select>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={handleRefreshMonitors} disabled={systemLoading || isLoading} className="gap-1">
            <RefreshCw className={`w-4 h-4 ${systemLoading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          <Button variant="outline" size="sm" onClick={() => applyPresetLayout("dual-side")} className="gap-1">
            <Grid3X3 className="w-4 h-4" />
            Side-by-Side
          </Button>
          <Button variant="outline" size="sm" onClick={() => applyPresetLayout("dual-vertical")} className="gap-1">
            <Grid3X3 className="w-4 h-4" />
            Vertical
          </Button>
          <Button variant="outline" size="sm" onClick={() => applyPresetLayout("triple")} className="gap-1">
            <Grid3X3 className="w-4 h-4" />
            Triple
          </Button>
          <Button onClick={handleAddMonitor} className="gap-2">
            <Plus className="w-4 h-4" />
            Add Monitor
          </Button>
          {selectedProfile && (
            <Button 
              onClick={handleSaveLayout} 
              disabled={isSaving || !hasUnsavedChanges} 
              className="gap-2"
              variant={hasUnsavedChanges ? "default" : "outline"}
            >
              {isSaving ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                <Save className="w-4 h-4" />
              )}
              {isSaving ? "Saving..." : hasUnsavedChanges ? "Save Layout" : "Saved"}
            </Button>
          )}
          {isTauri() && (
            <Button 
              onClick={handleApplyLayout} 
              disabled={isSaving} 
              className="gap-2"
              variant="secondary"
            >
              {isSaving ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                <Maximize2 className="w-4 h-4" />
              )}
              {isSaving ? "Applying..." : "Apply to System"}
            </Button>
          )}
        </div>
      </div>

      {/* Loading State */}
      {isLoading && (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center space-y-4">
            <Loader2 className="w-8 h-8 animate-spin mx-auto text-primary" />
            <p className="text-sm text-muted-foreground">Loading monitor layout...</p>
          </div>
        </div>
      )}

        {/* Canvas and Details Container */}
      <div className="flex-1 flex flex-col gap-4 min-h-0">
        {/* Canvas */}
        {!isLoading && (
        <Card className="flex-4 flex flex-col overflow-hidden">
          <CardHeader className="shrink-0 flex items-start justify-between gap-4">
            <CardTitle className="text-base flex items-center gap-2">
              <Maximize2 className="w-4 h-4" />
              Monitor Arrangement Canvas
            </CardTitle>
            <CardDescription>
              {selectedProfile && profileMonitors.length > 0 
                ? `Editing layout for profile. ${hasUnsavedChanges ? 'Unsaved changes.' : 'All changes saved.'}`
                : `Drag monitors to position them. Darker area shows primary monitor. Detected ${detectedMonitors.length} monitor${detectedMonitors.length !== 1 ? 's' : ''}.`
              }
            </CardDescription>

            {/* Zoom controls + Fit + Expand */}
            <div className="flex items-center gap-2 self-center">
              <Button size="sm" variant="ghost" onClick={() => { setDisplayScale((s) => Math.min(s + 0.05, 0.6)); setManualZoom(true); }} className="p-1">
                <ZoomIn className="w-4 h-4" />
              </Button>
              <Button size="sm" variant="ghost" onClick={() => { setDisplayScale((s) => Math.max(s - 0.05, 0.04)); setManualZoom(true); }} className="p-1">
                <ZoomOut className="w-4 h-4" />
              </Button>
              <Button
                size="sm"
                variant="ghost"
                onClick={() => {
                  // Unset locked scale and re-calculate to fit monitors inside the canvas.
                  // Use requestAnimationFrame to ensure layout settled.
                  setLockedScale(null)
                  setManualZoom(false)
                  requestAnimationFrame(() => setDisplayScale(calculateDisplayScale()))
                }}
                className="p-1"
                title="Fit to canvas"
              >
                <Grid3X3 className="w-4 h-4" />
              </Button>

              <Button
                size="sm"
                variant={canvasExpanded ? "default" : "ghost"}
                onClick={() => {
                  setCanvasExpanded((s) => {
                    const newVal = !s
                    // If expanding, ensure we fit into the new larger size; otherwise fit back to smaller canvas
                    requestAnimationFrame(() => setDisplayScale(calculateDisplayScale()))
                    return newVal
                  })
                }}
                className="p-1"
                title={canvasExpanded ? "Collapse canvas" : "Expand canvas"}
              >
                <Maximize2 className="w-4 h-4" />
              </Button>
            </div>
          </CardHeader>

          <CardContent className={`flex-1 p-0 min-h-0 ${canvasExpanded ? 'max-h-[75vh]' : 'max-h-[30vh]'}`}>
            <div
              ref={canvasRef}
              onMouseMove={handleMouseMove}
              onMouseUp={handleMouseUp}
              onMouseLeave={handleMouseUp}
              className="relative w-full h-full bg-linear-to-br from-background via-muted/20 to-muted/30 border-t border-border overflow-auto cursor-grab active:cursor-grabbing"
                style={{ 
                  backgroundImage: 'radial-gradient(circle, hsl(var(--muted-foreground) / 0.1) 1px, transparent 1px)',
                  backgroundSize: '12px 12px',
                  minHeight: `${desiredCanvasMinHeight}px`
                }}
            >
              {monitors.map((monitor) => (
                <motion.div
                  key={monitor.id}
                  initial={{ opacity: 0, scale: 0.8 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ duration: 0.3 }}
                  style={{
                    position: "absolute",
                    left: `${monitor.x * activeScale + (lockedOffsets ?? canvasOffsets).offsetX}px`,
                    top: `${monitor.y * activeScale + (lockedOffsets ?? canvasOffsets).offsetY}px`,
                  }}
                  onMouseDown={(e) => handleMonitorMouseDown(e, monitor.id)}
                  className={`group select-none transition-shadow duration-200 ${
                    draggingMonitor === monitor.id ? "shadow-2xl z-50" : "hover:shadow-lg"
                  }`}
                >
                  <div
                    style={{
                      width: `${monitor.width * activeScale}px`,
                      height: `${monitor.height * activeScale}px`,
                    }}
                    className={`rounded-lg overflow-hidden shadow-lg cursor-move border-2 ${
                      monitor.isPrimary ? "border-primary bg-card" : "border-primary/30 bg-card/80"
                    }`}
                  >
                    {/* Monitor Content */}
                    <div className="h-full w-full flex flex-col bg-linear-to-b from-card to-card/50 p-1">
                      {/* Header */}
                      <div className="text-xs font-bold text-foreground px-2 py-1 mb-1 truncate flex items-center gap-1">
                        {monitor.name}
                        {monitor.isPrimary && <span className="text-primary text-xs font-bold">●</span>}
                        {monitor.isBuiltin && <span className="text-muted-foreground text-xs">Built-in</span>}
                      </div>

                      {/* Brand/Model */}
                      {(monitor.brand || monitor.model) && (
                        <div className="text-xs text-muted-foreground px-2 mb-1">
                          {monitor.brand && monitor.model ? `${monitor.brand} ${monitor.model}` : (monitor.brand || monitor.model)}
                        </div>
                      )}

                      {/* Resolution and specs */}
                      <div className="text-xs text-muted-foreground px-2 mb-1 space-y-0.5">
                        <div>{monitor.resolution}</div>
                        {monitor.refreshRate && <div>{monitor.refreshRate}Hz</div>}
                        {monitor.scaleFactor && monitor.scaleFactor !== 1 && <div>{monitor.scaleFactor}x scale</div>}
                        {monitor.orientation && monitor.orientation !== 'Landscape' && <div>{monitor.orientation}</div>}
                      </div>

                      {/* Windows */}
                      <div className="flex-1 bg-background/30 rounded-sm relative overflow-hidden border border-border/30">
                        {monitor.windows.map((window) => (
                          <div
                            key={window.id}
                              style={{
                              position: "absolute",
                              left: `${window.x * activeScale}px`,
                              top: `${window.y * activeScale}px`,
                              width: `${Math.max(2, window.width * activeScale)}px`,
                              height: `${Math.max(2, window.height * activeScale)}px`,
                            }}
                            className="bg-primary/15 border border-primary/30 rounded-sm flex items-center justify-center p-1"
                          >
                            <span className="text-xs text-muted-foreground text-center font-medium">{window.app}</span>
                          </div>
                        ))}
                      </div>
                    </div>

                    {/* Delete Button */}
                    <Button
                      size="sm"
                      variant="ghost"
                      className="absolute -top-2 -right-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-destructive hover:text-destructive bg-card border border-border"
                      onClick={() => handleDeleteMonitor(monitor.id)}
                    >
                      <Trash2 className="w-3 h-3" />
                    </Button>
                  </div>
                </motion.div>
              ))}
            </div>
          </CardContent>
        </Card>
        )}
      </div>      {/* Manual Command Dialog */}
      <AlertDialog open={!!manualCommand} onOpenChange={() => setManualCommand(null)}>
        <AlertDialogContent className="max-w-2xl">
          <AlertDialogHeader>
            <AlertDialogTitle className="flex items-center gap-2">
              <Terminal className="w-5 h-5" />
              Manual Monitor Layout Application
            </AlertDialogTitle>
            <AlertDialogDescription>
              The automatic layout application failed. Please run the following command manually in Terminal to apply your monitor layout.
            </AlertDialogDescription>
          </AlertDialogHeader>
          
          {manualCommand && (
            <div className="space-y-4">
              <div className="bg-muted p-4 rounded-lg font-mono text-sm overflow-x-auto">
                <pre className="whitespace-pre-wrap break-all">{manualCommand}</pre>
              </div>
              
              <div className="text-sm text-muted-foreground space-y-2">
                <p><strong>Instructions:</strong></p>
                <ol className="list-decimal list-inside space-y-1 ml-4">
                  <li>Copy the command above</li>
                  <li>Open Terminal.app</li>
                  <li>Paste and run the command</li>
                  <li>You may need to enter your password when prompted by sudo</li>
                </ol>
                
                <p className="mt-3">
                  <strong>Note:</strong> Make sure displayplacer is installed: 
                  <code className="bg-background px-1 py-0.5 rounded text-xs ml-1">
                    brew install jakehilborn/jakehilborn/displayplacer
                  </code>
                </p>
              </div>
            </div>
          )}
          
          <AlertDialogFooter>
            <AlertDialogAction 
              onClick={() => {
                if (manualCommand) {
                  navigator.clipboard.writeText(manualCommand)
                  toast({
                    title: "Copied!",
                    description: "Command copied to clipboard",
                  })
                }
                setManualCommand(null)
              }}
              className="gap-2"
            >
              <Copy className="w-4 h-4" />
              Copy Command
            </AlertDialogAction>
            <AlertDialogAction onClick={() => setManualCommand(null)}>
              Close
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Permission Required Dialog */}
      <AlertDialog open={permissionNeeded} onOpenChange={() => setPermissionNeeded(false)}>
        <AlertDialogContent className="max-w-lg">
          <AlertDialogHeader>
            <AlertDialogTitle className="flex items-center gap-2">
              🔐 Permission Required
            </AlertDialogTitle>
            <AlertDialogDescription>
              Smoothie needs <strong>Screen Recording</strong> permission to change monitor layouts.
              This permission is required by macOS to allow apps to configure display arrangements.
            </AlertDialogDescription>
          </AlertDialogHeader>
          
          <div className="space-y-4">
            <div className="bg-muted p-4 rounded-lg text-sm space-y-2">
              <p><strong>To grant permission:</strong></p>
              <ol className="list-decimal list-inside space-y-1 ml-2">
                <li>Open <strong>System Settings</strong></li>
                <li>Go to <strong>Privacy & Security</strong></li>
                <li>Click <strong>Screen Recording</strong></li>
                <li>Enable <strong>Smoothie</strong> in the list</li>
                <li>Restart Smoothie for the change to take effect</li>
              </ol>
            </div>
          </div>
          
          <AlertDialogFooter>
            <AlertDialogAction 
              onClick={async () => {
                // Try to request permission (will open System Settings)
                if (isTauri()) {
                  await systemApi.requestDisplayPermission()
                }
                setPermissionNeeded(false)
              }}
              className="gap-2"
            >
              Open System Settings
            </AlertDialogAction>
            <AlertDialogAction onClick={() => setPermissionNeeded(false)}>
              Close
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}
