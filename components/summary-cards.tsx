import { Monitor, Layout, AppWindow , Loader2 } from "lucide-react";
import React from "react";

import { Card, CardContent } from "@/components/ui/card";

interface SummaryCardsProps {
  monitorsCount: number;
  windowsCount: number;
  appsCount: number;
  isLoading: boolean;
}

export const SummaryCards = React.memo(({ 
  monitorsCount, 
  windowsCount, 
  appsCount, 
  isLoading 
}: SummaryCardsProps) => {
  return (
    <div className="grid grid-cols-3 gap-4">
      <Card className="bg-card/50">
        <CardContent className="pt-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-muted-foreground">Monitors</p>
              <p className="text-2xl font-bold">
                {isLoading ? <Loader2 className="w-6 h-6 animate-spin" /> : monitorsCount}
              </p>
            </div>
            <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
              <Monitor className="w-5 h-5 text-primary" />
            </div>
          </div>
        </CardContent>
      </Card>
      <Card className="bg-card/50">
        <CardContent className="pt-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-muted-foreground">Windows</p>
              <p className="text-2xl font-bold">
                {isLoading ? <Loader2 className="w-6 h-6 animate-spin" /> : windowsCount}
              </p>
            </div>
            <div className="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center">
              <Layout className="w-5 h-5 text-accent" />
            </div>
          </div>
        </CardContent>
      </Card>
      <Card className="bg-card/50">
        <CardContent className="pt-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-muted-foreground">Running Apps</p>
              <p className="text-2xl font-bold">
                {isLoading ? <Loader2 className="w-6 h-6 animate-spin" /> : appsCount}
              </p>
            </div>
            <div className="w-10 h-10 rounded-lg bg-chart-1/10 flex items-center justify-center">
              <AppWindow className="w-5 h-5 text-chart-1" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
});