"use client"

import type React from "react"
import { useState, useRef } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Plus, Trash2, Maximize2, Grid3X3 } from "lucide-react"
import { motion } from "framer-motion"

interface Monitor {
  id: string
  x: number
  y: number
  width: number
  height: number
  name: string
  resolution: string
  isPrimary: boolean
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
}

export function MonitorEditor({ selectedProfile }: MonitorEditorProps) {
  const canvasRef = useRef<HTMLDivElement>(null)
  const [monitors, setMonitors] = useState<Monitor[]>([
    {
      id: "1",
      x: 50,
      y: 50,
      width: 320,
      height: 180,
      name: "Monitor 1",
      resolution: "3840×2160",
      isPrimary: true,
      windows: [
        { id: "w1", x: 10, y: 10, width: 300, height: 80, app: "VS Code" },
        { id: "w2", x: 10, y: 100, width: 300, height: 60, app: "Browser" },
      ],
    },
    {
      id: "2",
      x: 400,
      y: 50,
      width: 280,
      height: 180,
      name: "Monitor 2",
      resolution: "2560×1440",
      isPrimary: false,
      windows: [{ id: "w3", x: 10, y: 10, width: 260, height: 160, app: "Figma" }],
    },
  ])

  const [draggingMonitor, setDraggingMonitor] = useState<string | null>(null)
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 })

  const applyPresetLayout = (preset: string) => {
    if (preset === "dual-side") {
      setMonitors([
        { ...monitors[0], x: 20, y: 50, width: 320, height: 180 },
        { ...monitors[1], x: 370, y: 50, width: 280, height: 180 },
      ])
    } else if (preset === "triple") {
      setMonitors([
        { ...monitors[0], x: 20, y: 50, width: 280, height: 160 },
        { ...monitors[1], x: 320, y: 50, width: 280, height: 160 },
        {
          id: "3",
          x: 620,
          y: 50,
          width: 240,
          height: 160,
          name: "Monitor 3",
          resolution: "1920×1080",
          isPrimary: false,
          windows: [],
        },
      ])
    }
  }

  const handleMonitorMouseDown = (e: React.MouseEvent, monitorId: string) => {
    const monitor = monitors.find((m) => m.id === monitorId)
    if (!monitor) return

    const rect = canvasRef.current?.getBoundingClientRect()
    if (!rect) return

    setDraggingMonitor(monitorId)
    setDragOffset({
      x: e.clientX - rect.left - monitor.x,
      y: e.clientY - rect.top - monitor.y,
    })
  }

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!draggingMonitor || !canvasRef.current) return

    const rect = canvasRef.current.getBoundingClientRect()
    const newX = Math.max(0, e.clientX - rect.left - dragOffset.x)
    const newY = Math.max(0, e.clientY - rect.top - dragOffset.y)

    setMonitors(monitors.map((m) => (m.id === draggingMonitor ? { ...m, x: newX, y: newY } : m)))
  }

  const handleMouseUp = () => {
    setDraggingMonitor(null)
  }

  const handleAddMonitor = () => {
    const newMonitor: Monitor = {
      id: Date.now().toString(),
      x: 200 + monitors.length * 50,
      y: 50,
      width: 280,
      height: 180,
      name: `Monitor ${monitors.length + 1}`,
      resolution: "1920×1080",
      isPrimary: false,
      windows: [],
    }
    setMonitors([...monitors, newMonitor])
  }

  const handleDeleteMonitor = (id: string) => {
    setMonitors(monitors.filter((m) => m.id !== id))
  }

  return (
    <div className="h-full flex flex-col p-8 gap-8">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold text-foreground">Monitor Layout</h3>
          <p className="text-sm text-muted-foreground">
            {selectedProfile ? "Drag to arrange monitors and design your workspace" : "Select a profile first"}
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={() => applyPresetLayout("dual-side")} className="gap-1">
            <Grid3X3 className="w-4 h-4" />
            Dual
          </Button>
          <Button variant="outline" size="sm" onClick={() => applyPresetLayout("triple")} className="gap-1">
            <Grid3X3 className="w-4 h-4" />
            Triple
          </Button>
          <Button onClick={handleAddMonitor} className="gap-2">
            <Plus className="w-4 h-4" />
            Add Monitor
          </Button>
        </div>
      </div>

      {/* Canvas */}
      <Card className="flex-1 flex flex-col">
        <CardHeader>
          <CardTitle className="text-base flex items-center gap-2">
            <Maximize2 className="w-4 h-4" />
            Monitor Arrangement Canvas
          </CardTitle>
          <CardDescription>Drag monitors to position them. Darker area shows primary monitor.</CardDescription>
        </CardHeader>

        <CardContent className="flex-1 p-0">
          <div
            ref={canvasRef}
            onMouseMove={handleMouseMove}
            onMouseUp={handleMouseUp}
            onMouseLeave={handleMouseUp}
            className="relative w-full h-full bg-gradient-to-br from-background to-muted/20 border-t border-border overflow-hidden cursor-grab active:cursor-grabbing"
          >
            {monitors.map((monitor) => (
              <motion.div
                key={monitor.id}
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.3 }}
                style={{
                  position: "absolute",
                  left: `${monitor.x}px`,
                  top: `${monitor.y}px`,
                }}
                onMouseDown={(e) => handleMonitorMouseDown(e, monitor.id)}
                className={`group select-none transition-shadow duration-200 ${
                  draggingMonitor === monitor.id ? "shadow-2xl z-50" : "hover:shadow-lg"
                }`}
              >
                <div
                  style={{
                    width: `${monitor.width}px`,
                    height: `${monitor.height}px`,
                  }}
                  className={`rounded-lg overflow-hidden shadow-lg cursor-move border-2 ${
                    monitor.isPrimary ? "border-primary bg-card" : "border-primary/30 bg-card/80"
                  }`}
                >
                  {/* Monitor Content */}
                  <div className="h-full w-full flex flex-col bg-gradient-to-b from-card to-card/70 p-2">
                    {/* Header */}
                    <div className="text-xs font-bold text-foreground px-2 py-1 mb-1 truncate flex items-center gap-1">
                      {monitor.name}
                      {monitor.isPrimary && <span className="text-primary text-xs font-bold">●</span>}
                    </div>

                    {/* Resolution */}
                    <div className="text-xs text-muted-foreground px-2 mb-1">{monitor.resolution}</div>

                    {/* Windows */}
                    <div className="flex-1 bg-background/50 rounded-sm relative overflow-hidden border border-border/30">
                      {monitor.windows.map((window) => (
                        <div
                          key={window.id}
                          style={{
                            position: "absolute",
                            left: `${window.x}px`,
                            top: `${window.y}px`,
                            width: `${window.width}px`,
                            height: `${window.height}px`,
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

      {/* Monitor Details Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {monitors.map((monitor) => (
          <motion.div key={monitor.id} initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }}>
            <Card className={monitor.isPrimary ? "border-primary/50 bg-primary/5" : ""}>
              <CardHeader>
                <div className="flex items-start justify-between">
                  <CardTitle className="text-sm flex items-center gap-2">
                    {monitor.name}
                    {monitor.isPrimary && (
                      <span className="text-xs bg-primary text-primary-foreground px-2 py-0.5 rounded">Primary</span>
                    )}
                  </CardTitle>
                </div>
              </CardHeader>
              <CardContent className="space-y-3 text-sm">
                <div className="grid grid-cols-2 gap-2">
                  <div>
                    <p className="text-xs text-muted-foreground">Resolution</p>
                    <p className="font-mono font-semibold">{monitor.resolution}</p>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">Position</p>
                    <p className="font-mono font-semibold">
                      ({Math.round(monitor.x)}, {Math.round(monitor.y)})
                    </p>
                  </div>
                </div>
                <div>
                  <p className="text-xs text-muted-foreground mb-2">Windows: {monitor.windows.length}</p>
                  <div className="flex flex-wrap gap-1">
                    {monitor.windows.map((w) => (
                      <span key={w.id} className="text-xs bg-primary/10 text-primary px-2 py-1 rounded">
                        {w.app}
                      </span>
                    ))}
                  </div>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        ))}
      </div>
    </div>
  )
}
