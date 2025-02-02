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

const groups = [
  {
    title: "AI",
    items: [
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
  },
  {
    title: "Execution",
    items: [
      { title: "Code", url: "/execution/code", icon: CodeIcon },
      {
        title: "Math",
        url: "/execution/math",
        icon: SigmaIcon,
      },
    ],
  },
];

export function AppSidebar() {
  return (
    <Sidebar>
      <SidebarHeader />
      <SidebarContent>
        {groups.map((group) => (
          <SidebarGroup key={group.title}>
            <SidebarGroupLabel>{group.title}</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {group.items.map((item) => (
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
        ))}
      </SidebarContent>
      <SidebarFooter>
        <ModeToggle />
        <p className="text-sm text-muted-foreground">
          v{import.meta.env.PACKAGE_VERSION}-{import.meta.env.MODE}
        </p>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
