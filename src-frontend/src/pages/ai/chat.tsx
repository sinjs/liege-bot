import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { BotIcon, SendIcon, UserIcon } from "lucide-react";
import { useState } from "react";

type ChatMessage = {
  sender: "user" | "bot";
  content: string;
};

export function AIChat() {
  const [messages, setMessages] = useState<ChatMessage[]>([
    { sender: "user", content: "What's your name?" },
    { sender: "bot", content: "My name is LIEGE!" },
  ]);

  return (
    <div className="flex w-full flex-col justify-between h-full">
      <div className="p-3 pt-6 space-y-3">
        {messages.map((message) => (
          <div className="flex space-x-3">
            <div className="bg-sidebar border rounded p-1 h-min">
              {message.sender === "user" ? <UserIcon /> : <BotIcon />}
            </div>
            <div className="flex flex-col space-y-1">
              <div className="font-bold">
                {message.sender === "user" ? "User" : "Bot"}
              </div>
              <div className="text-sm">{message.content}</div>
            </div>
          </div>
        ))}
      </div>
      <div className="p-3 flex space-x-3 border-t">
        <Textarea placeholder="Hello, dear LiegeAI!" />
        <div className="h-full flex items-center">
          <Button variant="outline" size="icon">
            <SendIcon />
          </Button>
        </div>
      </div>
    </div>
  );
}
