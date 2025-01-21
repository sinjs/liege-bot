import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "@/components/ui/sidebar";
import {
  CodeIcon,
  ImageIcon,
  MessageCircleIcon,
  SigmaIcon,
} from "lucide-react";
import { ModeToggle } from "./mode-toggle";
import { Link } from "react-router";

const items = {
  ai: [
    {
      title: "Chat",
      url: "/ai/chat",
      icon: MessageCircleIcon,
    },
    {
      title: "Image",
      url: "/ai/image",
      icon: ImageIcon,
    },
  ],
  execution: [
    { title: "Code", url: "/execution/code", icon: CodeIcon },
    { title: "Math", url: "/execution/math", icon: SigmaIcon },
  ],
};

export function AppSidebar() {
  return (
    <Sidebar>
      <SidebarHeader />
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>AI</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.ai.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <Link to={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
        <SidebarGroup>
          <SidebarGroupLabel>Execution</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.execution.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <Link to={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <ModeToggle />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
