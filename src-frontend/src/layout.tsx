import { AppSidebar } from "@/components/app-sidebar";
import { SidebarProvider } from "@/components/ui/sidebar";
import { ThemeProvider } from "@/components/theme-provider";
import { Outlet } from "react-router";

export function Layout() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="ui-theme">
      <SidebarProvider>
        <AppSidebar />
        <main className="w-full">
          <Outlet />
        </main>
      </SidebarProvider>
    </ThemeProvider>
  );
}
