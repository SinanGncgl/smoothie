// Utility functions for system capture component

import { SystemMonitor } from "@/lib/tauri";
import { MONITOR_PREVIEW_HEIGHT, MONITOR_PREVIEW_PADDING, MONITOR_PREVIEW_SCALE_FACTOR } from "./system-capture-constants";

export interface MonitorBounds {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export function calculateMonitorBounds(monitors: SystemMonitor[]): MonitorBounds {
  return monitors.reduce(
    (acc, m) => ({
      minX: Math.min(acc.minX, m.x),
      minY: Math.min(acc.minY, m.y),
      maxX: Math.max(acc.maxX, m.x + m.width),
      maxY: Math.max(acc.maxY, m.y + m.height),
    }),
    { minX: 0, minY: 0, maxX: 800, maxY: 600 }
  );
}

export function calculateMonitorScale(bounds: MonitorBounds): number {
  const totalWidth = bounds.maxX - bounds.minX;
  const totalHeight = bounds.maxY - bounds.minY;
  const availableWidth = 400 - 2 * MONITOR_PREVIEW_PADDING;
  const availableHeight = MONITOR_PREVIEW_HEIGHT - 2 * MONITOR_PREVIEW_PADDING;

  return Math.min(
    availableWidth / totalWidth,
    availableHeight / totalHeight
  ) * MONITOR_PREVIEW_SCALE_FACTOR;
}

export function getMonitorPosition(
  monitor: SystemMonitor,
  bounds: MonitorBounds,
  scale: number
): { left: number; top: number; width: number; height: number } {
  const offsetX = (monitor.x - bounds.minX) * scale + MONITOR_PREVIEW_PADDING;
  const offsetY = (monitor.y - bounds.minY) * scale + MONITOR_PREVIEW_PADDING;
  const width = monitor.width * scale;
  const height = monitor.height * scale;

  return { left: offsetX, top: offsetY, width, height };
}