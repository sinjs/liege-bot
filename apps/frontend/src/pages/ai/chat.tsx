import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { api, cn } from "@/lib/utils";
import { BotIcon, SendIcon, UserIcon } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { createEventSource } from "eventsource-client";
import { useAuth } from "@/hooks/use-auth";

type ChatMessage = {
  sender: "user" | "bot";
  content: string;
  state: "final" | "loading" | "error";
};

function useMessages() {
  const [messagesState, setMessagesState] = useState({
    map: new Map<string, ChatMessage>(),
  });

  const messages = useMemo(
    () =>
      Array.from(messagesState.map, ([id, message]) => ({ id, ...message })),
    [messagesState]
  );

  return {
    messages,
    getMessage(id: string) {
      return messagesState.map.get(id);
    },
    setMessage(id: string, message: ChatMessage) {
      setMessagesState({ map: messagesState.map.set(id, message) });
    },
    addMessage(message: ChatMessage) {
      const id = crypto.randomUUID();
      setMessagesState({
        map: messagesState.map.set(id, message),
      });
      return id;
    },
  };
}

export function AIChat() {
  const { messages, getMessage, setMessage, addMessage } = useMessages();

  const [sending, setSending] = useState(false);
  const [prompt, setPrompt] = useState("");
  const { auth } = useAuth();

  const messagesEndRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  async function send() {
    setSending(true);
    addMessage({ sender: "user", content: prompt, state: "final" });
    setPrompt("");

    const searchParams = new URLSearchParams();
    searchParams.set("modelType", "text");
    searchParams.set("prompt", prompt);

    const messageId = addMessage({
      sender: "bot",
      content: "",
      state: "loading",
    });

    try {
      const eventSource = createEventSource({
        url: api(`ai?${searchParams.toString()}`),
        headers: { Authorization: `Bearer ${auth?.token}` },
      });

      for await (const { data } of eventSource) {
        const parsedData:
          | { type: "Done" }
          | { type: "Response"; data: string } = JSON.parse(data);

        const message = getMessage(messageId);
        if (!message) throw new TypeError(`message ${messageId} is undefined`);

        if (parsedData.type === "Done") {
          eventSource.close();
          setSending(false);
          setMessage(messageId, { ...message, state: "final" });
          return;
        }

        setMessage(messageId, {
          content: message.content + parsedData.data,
          sender: "bot",
          state: "loading",
        });
      }
    } catch (error) {
      console.error("Failed to send", error);
      const message = getMessage(messageId);
      if (message) {
        setMessage(messageId, {
          ...message,
          state: "error",
          content: `Failed to receive data from the server: ${error}`,
        });
      }
    }
  }

  return (
    <div className="flex w-full flex-col justify-between h-full">
      <div className="px-3 py-6 space-y-3 grow">
        {messages.map((message) => (
          <div className="flex space-x-3">
            <div className="bg-sidebar border rounded p-1 h-min">
              {message.sender === "user" ? <UserIcon /> : <BotIcon />}
            </div>
            <div className="flex flex-col space-y-1">
              <div className="font-bold">
                {message.sender === "user" ? "User" : "Bot"}
              </div>
              <div
                className={cn(
                  { "text-destructive-foreground": message.state === "error" },
                  "text-sm"
                )}
              >
                <Markdown
                  className="prose dark:prose-invert prose-neutral prose-sm"
                  remarkPlugins={[remarkGfm]}
                >
                  {message.content}
                </Markdown>
                {message.state === "loading" && (
                  <div className="inline-block rounded-full bg-muted-foreground h-3 w-3 animate-pulse pl-2"></div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
      <div ref={messagesEndRef} />
      <div className="p-3 flex space-x-3 border-t sticky bottom-0 bg-background">
        <Textarea
          placeholder="Hello, dear LiegeAI!"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
        />
        <div className="h-full flex items-center">
          <Button
            variant="outline"
            size="icon"
            onClick={send}
            disabled={sending}
          >
            <SendIcon />
          </Button>
        </div>
      </div>
    </div>
  );
}
