import React from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Monitor } from "lucide-react";
import { SystemMonitor, SystemWindow } from "@/lib/tauri";

interface MonitorListProps {
  monitors: SystemMonitor[];
  windows: SystemWindow[];
}

export const MonitorList = React.memo(function MonitorList({ monitors, windows }: MonitorListProps) {
  return (
    <div className="grid gap-3">
      {monitors.map((monitor) => {
        // Count windows on this monitor
        const monitorWindows = windows.filter(window => window.displayId === monitor.displayId);

        return (
          <Card key={monitor.displayId} className={monitor.isPrimary ? "border-primary/50" : ""}>
            <CardHeader className="pb-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Monitor className="w-4 h-4 text-primary" />
                  <CardTitle className="text-sm font-medium">{monitor.name}</CardTitle>
                  {monitor.isPrimary && <Badge>Primary</Badge>}
                  {monitor.isBuiltin && <Badge variant="outline">Built-in</Badge>}
                </div>
                <div className="text-right text-xs text-muted-foreground">
                  ID: {monitor.displayId}
                </div>
              </div>
            </CardHeader>
            <CardContent className="pt-0 space-y-2">
              {/* Brand / Model */}
              {(monitor.brand || monitor.model) && (
                <div className="text-xs text-muted-foreground">
                  {monitor.brand && monitor.model ? `${monitor.brand} ${monitor.model}` : (monitor.brand || monitor.model)}
                </div>
              )}

              {/* Resolution and specs */}
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3 text-sm">
                  <span className="font-mono font-semibold">{monitor.resolution}</span>
                  {monitor.refreshRate && <span className="text-sm text-muted-foreground">{monitor.refreshRate}Hz</span>}
                  {monitor.scaleFactor && monitor.scaleFactor !== 1 && <span className="text-sm text-muted-foreground">{monitor.scaleFactor}x</span>}
                </div>
                {monitor.isBuiltin !== undefined && (
                  <div className="text-xs px-2 py-0.5 bg-muted/10 rounded text-muted-foreground whitespace-nowrap">
                    {monitor.isBuiltin ? 'Built-in' : 'External'}
                  </div>
                )}
              </div>

              {/* Position and Orientation */}
              <div className="flex items-center justify-between text-xs text-muted-foreground">
                <span>Position: ({monitor.x}, {monitor.y})</span>
                <span>{monitor.orientation}</span>
              </div>

              {/* Windows count */}
              <div className="text-xs text-muted-foreground">
                Windows: <span className="font-medium text-foreground">{monitorWindows.length}</span>
              </div>
            </CardContent>
          </Card>
        );
      })}
    </div>
  );
});