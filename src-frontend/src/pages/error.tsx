import { Button } from "@/components/ui/button";
import { CircleXIcon } from "lucide-react";
import { useMemo } from "react";

import { FallbackProps } from "react-error-boundary";

export function Error({ error, resetErrorBoundary }: FallbackProps) {
  const message = useMemo(() => String(error), [error]);

  return (
    <div className="h-screen w-screen bg-background text-foreground flex flex-col space-y-3 justify-center items-center">
      <CircleXIcon />
      <div className="text-center space-y-3">
        <h1 className="scroll-m-20 text-2xl font-semibold tracking-tight">
          Error
        </h1>
        <p className="text-sm [&:not(:first-child)]:mt-6">
          We have encountered an error while loading the page. Please check your
          browser console for more details.
        </p>
        <code className="relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold">
          {message}
        </code>
      </div>
      <Button onClick={resetErrorBoundary}>Retry</Button>
    </div>
  );
}
