import React from "react";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { AppWindow, Minimize2, Maximize2 } from "lucide-react";
import { SystemWindow } from "@/lib/tauri";
import { SCROLL_AREA_HEIGHT } from "./system-capture-constants";

interface WindowsListProps {
  windows: SystemWindow[];
}

export const WindowsList = React.memo(function WindowsList({ windows }: WindowsListProps) {
  return (
    <ScrollArea className={`h-[${SCROLL_AREA_HEIGHT}px]`}>
      <div className="grid gap-2">
        {windows.map((window) => (
          <Card key={window.windowId} className="bg-card/50">
            <CardContent className="py-3">
              <div className="flex items-start justify-between">
                <div className="space-y-1 flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <AppWindow className="w-4 h-4 text-muted-foreground shrink-0" />
                    <span className="font-medium truncate">
                      {window.title || "(Untitled)"}
                    </span>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    {window.appName}
                  </p>
                  <p className="text-xs text-muted-foreground font-mono">
                    {window.bundleId}
                  </p>
                </div>
                <div className="text-right text-xs text-muted-foreground shrink-0 ml-4">
                  <p>{window.width}×{window.height}</p>
                  <p>({window.x}, {window.y})</p>
                  <div className="flex gap-1 justify-end mt-1">
                    {window.isMinimized && (
                      <Badge variant="outline" className="text-[10px]">
                        <Minimize2 className="w-3 h-3 mr-1" />Min
                      </Badge>
                    )}
                    {window.isFullscreen && (
                      <Badge variant="outline" className="text-[10px]">
                        <Maximize2 className="w-3 h-3 mr-1" />Full
                      </Badge>
                    )}
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
        {windows.length === 0 && (
          <div className="text-center py-8 text-muted-foreground">
            No windows detected. Click Refresh to scan.
          </div>
        )}
      </div>
    </ScrollArea>
  );
});