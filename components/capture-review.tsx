import React from "react";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Eye, Save, X, Monitor } from "lucide-react";
import { CapturedLayout, SystemMonitor, RunningApp } from "@/lib/tauri";
import { APPS_PREVIEW_LIMIT } from "./system-capture-constants";

interface CaptureReviewProps {
  capturedLayout: CapturedLayout;
  onSave: () => void;
  onDiscard: () => void;
}

export const CaptureReview = React.memo(function CaptureReview({ 
  capturedLayout, 
  onSave, 
  onDiscard 
}: CaptureReviewProps) {
  return (
    <motion.div
      initial={{ opacity: 0, height: 0 }}
      animate={{ opacity: 1, height: 'auto' }}
      exit={{ opacity: 0, height: 0 }}
      className="border border-primary/50 rounded-lg bg-primary/5 p-4 space-y-4"
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Eye className="w-4 h-4 text-primary" />
          <span className="font-medium text-primary">Captured Layout Review</span>
          <Badge variant="outline" className="text-xs">
            {new Date(capturedLayout.capturedAt).toLocaleString()}
          </Badge>
        </div>
        <div className="flex gap-2">
          <Button size="sm" onClick={onSave} className="gap-1">
            <Save className="w-3 h-3" />
            Save to Profile
          </Button>
          <Button size="sm" variant="outline" onClick={onDiscard} className="gap-1">
            <X className="w-3 h-3" />
            Discard
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4">
        <div className="text-center">
          <p className="text-2xl font-bold text-primary">{capturedLayout.monitors.length}</p>
          <p className="text-xs text-muted-foreground">Monitors Captured</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-primary">{capturedLayout.windows.length}</p>
          <p className="text-xs text-muted-foreground">Windows Captured</p>
        </div>
        <div className="text-center">
          <p className="text-2xl font-bold text-primary">{capturedLayout.runningApps.length}</p>
          <p className="text-xs text-muted-foreground">Apps Captured</p>
        </div>
      </div>

      <div className="space-y-3">
        <div>
          <h4 className="text-sm font-medium mb-2">Monitors:</h4>
          <div className="grid gap-2">
            {capturedLayout.monitors.map((monitor: SystemMonitor) => (
              <div key={monitor.displayId} className="flex items-center justify-between bg-card/50 p-2 rounded text-sm">
                <div className="flex items-center gap-2">
                  <Monitor className="w-3 h-3" />
                  <span className="font-medium">{monitor.name}</span>
                  {monitor.isPrimary && <Badge variant="default" className="text-[10px] py-0 px-1">Primary</Badge>}
                </div>
                <span className="text-xs text-muted-foreground">{monitor.resolution}</span>
              </div>
            ))}
          </div>
        </div>

        <div>
          <h4 className="text-sm font-medium mb-2">Key Applications:</h4>
          <div className="flex flex-wrap gap-2">
            {capturedLayout.runningApps.slice(0, APPS_PREVIEW_LIMIT).map((app: RunningApp) => (
              <Badge key={app.pid} variant="secondary" className="text-xs">
                {app.name}
              </Badge>
            ))}
            {capturedLayout.runningApps.length > APPS_PREVIEW_LIMIT && (
              <Badge variant="outline" className="text-xs">
                +{capturedLayout.runningApps.length - APPS_PREVIEW_LIMIT} more
              </Badge>
            )}
          </div>
        </div>

        <div className="text-xs text-muted-foreground">
          <p>This layout will be saved to your profile and can be restored later.</p>
          <p>Review the details above before saving.</p>
        </div>
      </div>
    </motion.div>
  );
});