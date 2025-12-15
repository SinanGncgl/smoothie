"use client"

import { AnimatePresence, motion } from "framer-motion"
import {
  Camera,
  RefreshCw,
  Loader2,
  CheckCircle2,
  XCircle,
  Eye,
  EyeOff
} from "lucide-react"
import { useState } from "react"

import { AppsList } from "./apps-list"
import { CaptureReview } from "./capture-review"
import { MonitorArrangement } from "./monitor-arrangement"
import { MonitorList } from "./monitor-list"
import { SummaryCards } from "./summary-cards"
import { WindowsList } from "./windows-list"

import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useSystemDetection } from "@/hooks/use-system-detection"
import { CapturedLayout } from "@/lib/tauri"


interface SystemCaptureProps {
  onCaptureComplete?: (layout: unknown) => void
}

export function SystemCapture({ onCaptureComplete }: SystemCaptureProps) {
  const { 
    monitors, 
    windows, 
    runningApps, 
    isLoading, 
    error,
    lastCapturedAt,
    refreshAll,
    captureLayout 
  } = useSystemDetection()

  const [showCapture, setShowCapture] = useState(false)
  const [capturedLayout, setCapturedLayout] = useState<CapturedLayout | null>(null)
  const [showCapturedDetails, setShowCapturedDetails] = useState(false)

  const handleCapture = async () => {
    const layout = await captureLayout()
    if (layout) {
      setCapturedLayout(layout)
      setShowCapturedDetails(true)
      setShowCapture(true)
    }
  }

  const handleSaveCapturedLayout = () => {
    if (capturedLayout && onCaptureComplete) {
      onCaptureComplete(capturedLayout)
    }
    // Reset capture state to allow new captures
    setCapturedLayout(null)
    setShowCapturedDetails(false)
    setShowCapture(false)
  }

  const handleDiscardCapturedLayout = () => {
    // Reset capture state to allow new captures
    setCapturedLayout(null)
    setShowCapturedDetails(false)
    setShowCapture(false)
  }

  return (
    <div className="space-y-6" role="region" aria-labelledby="system-capture-heading">
      {/* Header with Actions */}
      <div className="flex items-center justify-between">
        <div>
          <h3 id="system-capture-heading" className="text-lg font-semibold text-foreground flex items-center gap-2">
            <Camera className="w-5 h-5" aria-hidden="true" />
            System Capture
          </h3>
          <p className="text-sm text-muted-foreground">
            Detect and capture your current monitor and window layout
          </p>
        </div>
        <div className="flex gap-2">
          <Button 
            variant="outline" 
            size="sm" 
            onClick={refreshAll}
            disabled={isLoading}
            aria-label="Refresh system information"
          >
            {isLoading ? (
              <Loader2 className="w-4 h-4 animate-spin" aria-hidden="true" />
            ) : (
              <RefreshCw className="w-4 h-4" aria-hidden="true" />
            )}
            <span className="ml-2">Refresh</span>
          </Button>
          <Button 
            onClick={handleCapture} 
            disabled={isLoading}
            variant={showCapturedDetails ? "secondary" : "default"}
            className="gap-2"
            aria-label={showCapturedDetails ? "Review captured layout" : "Capture current layout"}
          >
            {showCapturedDetails ? (
              <>
                <Eye className="w-4 h-4" aria-hidden="true" />
                Review Capture
              </>
            ) : (
              <>
                <Camera className="w-4 h-4" aria-hidden="true" />
                Capture Layout
              </>
            )}
          </Button>
        </div>
      </div>

      {/* Error display */}
      <AnimatePresence>
        {error && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="flex items-center gap-2 text-sm text-red-500 bg-red-500/10 px-4 py-2 rounded-lg"
          >
            <XCircle className="w-4 h-4" />
            {error.message}
          </motion.div>
        )}
      </AnimatePresence>

      {/* Capture timestamp */}
      {lastCapturedAt && !showCapturedDetails && (
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <CheckCircle2 className="w-3 h-3 text-green-500" />
          Last captured: {new Date(lastCapturedAt).toLocaleString()}
        </div>
      )}

      {showCapturedDetails && capturedLayout && (
        <div className="flex items-center gap-2 text-xs text-primary">
          <Eye className="w-3 h-3" />
          Layout captured and ready for review
        </div>
      )}

      {/* Summary Cards */}
      <SummaryCards
        monitorsCount={monitors.length}
        windowsCount={windows.length}
        appsCount={runningApps.length}
        isLoading={isLoading}
      />

      {/* Captured Layout Review */}
      <AnimatePresence>
        {showCapturedDetails && capturedLayout && (
          <CaptureReview
            capturedLayout={capturedLayout}
            onSave={handleSaveCapturedLayout}
            onDiscard={handleDiscardCapturedLayout}
          />
        )}
      </AnimatePresence>

      {/* Tabs for details */}
      <Tabs defaultValue="monitors" className="w-full">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="monitors">Monitors</TabsTrigger>
          <TabsTrigger value="windows">Windows</TabsTrigger>
          <TabsTrigger value="apps">Apps</TabsTrigger>
        </TabsList>

        {/* Monitors Tab */}
        <TabsContent value="monitors" className="space-y-4">
          <MonitorArrangement monitors={monitors} />
          <MonitorList monitors={monitors} windows={windows} />
        </TabsContent>

        {/* Windows Tab */}
        <TabsContent value="windows">
          <WindowsList windows={windows} />
        </TabsContent>

        {/* Apps Tab */}
        <TabsContent value="apps">
          <AppsList runningApps={runningApps} />
        </TabsContent>
      </Tabs>
    </div>
  )
}
