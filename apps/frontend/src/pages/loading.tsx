import { LoaderCircle } from "lucide-react";

export function Loading({ message }: { message?: string }) {
  return (
    <div className="h-screen w-screen bg-background text-foreground flex flex-col space-y-2 justify-center items-center">
      <LoaderCircle className="animate-spin " />
      <p className="text-sm [&:not(:first-child)]:mt-6">
        {message ?? "Loading"}
      </p>
    </div>
  );
}
