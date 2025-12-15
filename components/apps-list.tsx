import React from "react";

import { SCROLL_AREA_HEIGHT } from "./system-capture-constants";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { RunningApp } from "@/lib/tauri";


interface AppsListProps {
  runningApps: RunningApp[];
}

export const AppsList = React.memo(({ runningApps }: AppsListProps) => {
  return (
    <ScrollArea className={`h-[${SCROLL_AREA_HEIGHT}px]`}>
      <div className="grid gap-2">
        {runningApps.map((app) => (
          <Card key={app.pid} className={`bg-card/50 ${app.isActive ? 'border-primary/50' : ''}`}>
            <CardContent className="py-3">
              <div className="flex items-center justify-between">
                <div className="space-y-1 flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="font-medium">{app.name}</span>
                    {app.isActive && <Badge>Active</Badge>}
                    {app.isHidden && <Badge variant="outline">Hidden</Badge>}
                  </div>
                  <p className="text-xs text-muted-foreground font-mono truncate">
                    {app.bundleId}
                  </p>
                </div>
                <div className="text-right text-sm text-muted-foreground shrink-0 ml-4">
                  <p className="font-medium">{app.windowCount} window{app.windowCount !== 1 ? 's' : ''}</p>
                  <p className="text-xs">PID: {app.pid}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
        {runningApps.length === 0 && (
          <div className="text-center py-8 text-muted-foreground">
            No running apps detected. Click Refresh to scan.
          </div>
        )}
      </div>
    </ScrollArea>
  );
});