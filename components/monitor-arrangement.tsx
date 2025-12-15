import { motion } from "framer-motion";
import { Maximize2 } from "lucide-react";
import React from "react";

import { MONITOR_PREVIEW_HEIGHT, MONITOR_MIN_WIDTH, MONITOR_MIN_HEIGHT } from "./system-capture-constants";
import { calculateMonitorBounds, calculateMonitorScale, getMonitorPosition, MonitorBounds } from "./system-capture-utils";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { SystemMonitor } from "@/lib/tauri";


interface MonitorArrangementProps {
  monitors: SystemMonitor[];
}

export const MonitorArrangement = React.memo(({ monitors }: MonitorArrangementProps) => {
  const bounds = calculateMonitorBounds(monitors);
  const scale = calculateMonitorScale(bounds);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm flex items-center gap-2">
          <Maximize2 className="w-4 h-4" />
          Monitor Arrangement
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div
          className="relative bg-muted/20 rounded-lg border border-border overflow-hidden"
          style={{
            height: MONITOR_PREVIEW_HEIGHT,
            width: '100%'
          }}
        >
          {monitors.map((monitor) => {
            const position = getMonitorPosition(monitor, bounds, scale);

            return (
              <motion.div
                key={monitor.displayId}
                initial={{ opacity: 0, scale: 0.8 }}
                animate={{ opacity: 1, scale: 1 }}
                className={`absolute rounded border-2 ${
                  monitor.isPrimary
                    ? 'border-primary bg-primary/20'
                    : 'border-border bg-card'
                }`}
                style={{
                  left: position.left,
                  top: position.top,
                  width: Math.max(position.width, MONITOR_MIN_WIDTH),
                  height: Math.max(position.height, MONITOR_MIN_HEIGHT),
                }}
              >
                <div className="p-1 text-xs truncate">
                  <span className="font-medium">{monitor.name}</span>
                  {monitor.isPrimary && (
                    <Badge variant="default" className="ml-1 text-[10px] py-0 px-1">Primary</Badge>
                  )}
                </div>
                <div className="px-1 text-[10px] text-muted-foreground">
                  {monitor.width}×{monitor.height}
                </div>
              </motion.div>
            );
          })}
        </div>
      </CardContent>
    </Card>
  );
});