import { AlertTriangle } from "lucide-react";
import React from "react";

import { Button } from "@/components/ui/button";

interface ErrorFallbackProps {
  error: Error;
  resetError: () => void;
}

export function SystemCaptureErrorBoundary({ error, resetError }: ErrorFallbackProps) {
  return (
    <div className="flex flex-col items-center justify-center p-8 space-y-4 text-center">
      <AlertTriangle className="w-12 h-12 text-red-500" />
      <div className="space-y-2">
        <h3 className="text-lg font-semibold text-foreground">Something went wrong</h3>
        <p className="text-sm text-muted-foreground max-w-md">
          We encountered an error while capturing your system information. This might be due to
          permission issues or system compatibility.
        </p>
        <details className="text-xs text-muted-foreground mt-4">
          <summary className="cursor-pointer hover:text-foreground">Error details</summary>
          <pre className="mt-2 p-2 bg-muted rounded text-left overflow-auto max-w-md">
            {error.message}
          </pre>
        </details>
      </div>
      <Button onClick={resetError} variant="outline">
        Try Again
      </Button>
    </div>
  );
}