// Hook for system detection - monitors, windows, and running apps

import { useState, useCallback, useEffect } from 'react';
import { 
  systemApi, 
  SystemMonitor, 
  SystemWindow, 
  RunningApp,
  InstalledApp, 
  CapturedLayout,
  isTauri 
} from '@/lib/tauri';

interface SystemDetectionState {
  monitors: SystemMonitor[];
  windows: SystemWindow[];
  runningApps: RunningApp[];
  installedApps: InstalledApp[];
  isLoading: boolean;
  error: Error | null;
  lastCapturedAt: string | null;
}

export function useSystemDetection() {
  const [state, setState] = useState<SystemDetectionState>({
    monitors: [],
    windows: [],
    runningApps: [],
    installedApps: [],
    isLoading: false,
    error: null,
    lastCapturedAt: null,
  });

  // Refresh monitors only
  const refreshMonitors = useCallback(async () => {
    if (!isTauri()) {
      setState(prev => ({
        ...prev,
        monitors: MOCK_MONITORS,
      }));
      return;
    }

    try {
      const monitors = await systemApi.getConnectedMonitors();
      setState(prev => ({ ...prev, monitors, error: null }));
    } catch (err) {
      setState(prev => ({ ...prev, error: err as Error }));
    }
  }, []);

  // Refresh windows only
  const refreshWindows = useCallback(async () => {
    if (!isTauri()) {
      setState(prev => ({
        ...prev,
        windows: MOCK_WINDOWS,
      }));
      return;
    }

    try {
      const windows = await systemApi.getVisibleWindows();
      setState(prev => ({ ...prev, windows, error: null }));
    } catch (err) {
      setState(prev => ({ ...prev, error: err as Error }));
    }
  }, []);

  // Refresh running apps only
  const refreshRunningApps = useCallback(async () => {
    if (!isTauri()) {
      setState(prev => ({
        ...prev,
        runningApps: MOCK_APPS,
      }));
      return;
    }

    try {
      const runningApps = await systemApi.getRunningApps();
      setState(prev => ({ ...prev, runningApps, error: null }));
    } catch (err) {
      setState(prev => ({ ...prev, error: err as Error }));
    }
  }, []);

  // Refresh installed apps only
  const refreshInstalledApps = useCallback(async () => {
    if (!isTauri()) {
      setState(prev => ({
        ...prev,
        installedApps: MOCK_INSTALLED_APPS,
      }));
      return;
    }

    try {
      const installedApps = await systemApi.getInstalledApps();
      setState(prev => ({ ...prev, installedApps, error: null }));
    } catch (err) {
      setState(prev => ({ ...prev, error: err as Error }));
    }
  }, []);

  // Capture full layout (monitors + windows + apps)
  const captureLayout = useCallback(async (): Promise<CapturedLayout | null> => {
    if (!isTauri()) {
      const mockLayout: CapturedLayout = {
        capturedAt: new Date().toISOString(),
        monitors: MOCK_MONITORS,
        windows: MOCK_WINDOWS,
        runningApps: MOCK_APPS,
      };
      setState(prev => ({
        ...prev,
        monitors: mockLayout.monitors,
        windows: mockLayout.windows,
        runningApps: mockLayout.runningApps,
        lastCapturedAt: mockLayout.capturedAt,
      }));
      return mockLayout;
    }

    setState(prev => ({ ...prev, isLoading: true, error: null }));

    try {
      const layout = await systemApi.captureCurrentLayout();
      setState(prev => ({
        ...prev,
        monitors: layout.monitors,
        windows: layout.windows,
        runningApps: layout.runningApps,
        isLoading: false,
        lastCapturedAt: layout.capturedAt,
      }));
      return layout;
    } catch (err) {
      setState(prev => ({ 
        ...prev, 
        isLoading: false, 
        error: err as Error 
      }));
      return null;
    }
  }, []);

  // Refresh everything - optimized to load fast data first
  const refreshAll = useCallback(async () => {
    setState(prev => ({ ...prev, isLoading: true, error: null }));
    
    if (!isTauri()) {
      setState({
        monitors: MOCK_MONITORS,
        windows: MOCK_WINDOWS,
        runningApps: MOCK_APPS,
        installedApps: MOCK_INSTALLED_APPS,
        isLoading: false,
        error: null,
        lastCapturedAt: new Date().toISOString(),
      });
      return;
    }

    try {
      // First load monitors and windows quickly (these are fast native calls)
      const [monitors, windows] = await Promise.all([
        systemApi.getConnectedMonitors(),
        systemApi.getVisibleWindows(),
      ]);
      
      // Update state immediately so UI shows data fast
      setState(prev => ({
        ...prev,
        monitors,
        windows,
        isLoading: false,
        error: null,
        lastCapturedAt: new Date().toISOString(),
      }));
      
      // Then load running apps in background (AppleScript is slower)
      systemApi.getRunningApps().then(runningApps => {
        setState(prev => ({
          ...prev,
          runningApps,
        }));
      }).catch(() => {
        // Silently fail - running apps are optional
      });
    } catch (err) {
      setState(prev => ({ 
        ...prev, 
        isLoading: false, 
        error: err as Error 
      }));
    }
  }, []);

  // Auto-refresh all data on mount
  useEffect(() => {
    refreshAll();
  }, [refreshAll]);

  return {
    ...state,
    refreshMonitors,
    refreshWindows,
    refreshRunningApps,
    refreshInstalledApps,
    refreshAll,
    captureLayout,
  };
}

// Mock data for development/browser preview
const MOCK_MONITORS: SystemMonitor[] = [
  {
    displayId: 1,
    name: 'Primary Display',
    brand: 'Apple',
    model: 'Studio Display',
    resolution: '2560x1440',
    width: 2560,
    height: 1440,
    x: 0,
    y: 0,
    scaleFactor: 2.0,
    refreshRate: 60,
    isPrimary: true,
    isBuiltin: false,
    orientation: 'Landscape',
  },
  {
    displayId: 2,
    name: 'Built-in Display',
    brand: 'Apple',
    model: 'MacBook Pro 16"',
    resolution: '1440x900',
    width: 1440,
    height: 900,
    x: 2560,
    y: 540,
    scaleFactor: 2.0,
    refreshRate: 60,
    isPrimary: false,
    isBuiltin: true,
    orientation: 'Landscape',
  },
];

const MOCK_WINDOWS: SystemWindow[] = [
  {
    windowId: 1001,
    pid: 1234,
    title: 'VS Code - smoothie',
    appName: 'Code',
    bundleId: 'com.microsoft.VSCode',
    x: 0,
    y: 25,
    width: 2560,
    height: 1415,
    displayId: 1,
    isMinimized: false,
    isFullscreen: false,
    layer: 0,
  },
  {
    windowId: 1002,
    pid: 2345,
    title: 'Safari - GitHub',
    appName: 'Safari',
    bundleId: 'com.apple.Safari',
    x: 2560,
    y: 540,
    width: 1440,
    height: 900,
    displayId: 2,
    isMinimized: false,
    isFullscreen: false,
    layer: 0,
  },
];

const MOCK_APPS: RunningApp[] = [
  {
    pid: 1234,
    name: 'Code',
    bundleId: 'com.microsoft.VSCode',
    path: '/Applications/Visual Studio Code.app',
    isActive: true,
    isHidden: false,
    windowCount: 1,
  },
  {
    pid: 2345,
    name: 'Safari',
    bundleId: 'com.apple.Safari',
    path: '/Applications/Safari.app',
    isActive: false,
    isHidden: false,
    windowCount: 1,
  },
  {
    pid: 3456,
    name: 'Finder',
    bundleId: 'com.apple.finder',
    path: '/System/Library/CoreServices/Finder.app',
    isActive: false,
    isHidden: false,
    windowCount: 2,
  },
];

const MOCK_INSTALLED_APPS: InstalledApp[] = [
  {
    name: 'Visual Studio Code',
    bundleId: 'com.microsoft.VSCode',
    path: '/Applications/Visual Studio Code.app',
    version: '1.85.0',
    category: 'public.app-category.developer-tools',
  },
  {
    name: 'Safari',
    bundleId: 'com.apple.Safari',
    path: '/Applications/Safari.app',
    version: '17.2',
    category: 'public.app-category.productivity',
  },
  {
    name: 'Google Chrome',
    bundleId: 'com.google.Chrome',
    path: '/Applications/Google Chrome.app',
    version: '120.0.6099.109',
    category: 'public.app-category.productivity',
  },
  {
    name: 'Slack',
    bundleId: 'com.tinyspeck.slackmacgap',
    path: '/Applications/Slack.app',
    version: '4.35.126',
    category: 'public.app-category.business',
  },
  {
    name: 'Spotify',
    bundleId: 'com.spotify.client',
    path: '/Applications/Spotify.app',
    version: '1.2.26',
    category: 'public.app-category.music',
  },
  {
    name: 'Figma',
    bundleId: 'com.figma.Desktop',
    path: '/Applications/Figma.app',
    version: '116.14.5',
    category: 'public.app-category.graphics-design',
  },
];

export default useSystemDetection;
