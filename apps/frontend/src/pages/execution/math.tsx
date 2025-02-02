import { useAuth } from "@/hooks/use-auth";
import { api } from "@/lib/utils";
import { FormEventHandler, useRef, useState } from "react";

import "@/styles/numbat-syntax.css";

type BufferEntry = {
  type: "input" | "output";
  data: string;
};

function useBuffer() {
  const [bufferState, setBufferState] = useState({
    entries: [] as BufferEntry[],
  });

  return {
    entries: bufferState.entries,
    push(entry: BufferEntry) {
      setBufferState((prev) => ({
        entries: [...prev.entries, entry],
      }));
    },
    clear() {
      setBufferState({ entries: [] });
    },
  };
}

type MathResponse = {
  success: boolean;
  output: string;
};

export function ExecutionMath() {
  const { auth } = useAuth();

  const inputRef = useRef<HTMLInputElement>(null);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const { entries, push, clear } = useBuffer();

  const onSubmit: FormEventHandler<HTMLFormElement> = async (event) => {
    event.preventDefault();

    if (!input) return;

    setLoading(true);

    push({ type: "input", data: input });

    try {
      switch (input) {
        case "clear": {
          clear();
          break;
        }
        case "help": {
          push({
            type: "output",
            data: 'You can view <a class="text-blue-500 hover:text-blue-400 transition" href="https://numbat.dev/doc/" target="_blank">the documentation</a> for help.',
          });

          break;
        }
        default: {
          const response = await fetch(api("/math"), {
            method: "POST",
            headers: {
              Authorization: `Bearer ${auth?.token}`,
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              input,
            }),
          });

          if (!response.ok)
            throw new Error(
              `Server responded with status code ${response.status} `
            );

          const mathResponse: MathResponse = await response.json();

          push({ type: "output", data: mathResponse.output });
          break;
        }
      }
    } catch (error) {
      push({ type: "output", data: String(error) });
    } finally {
      setLoading(false);
      setInput("");
    }
  };

  return (
    <div className="p-3 h-full">
      <div
        className="p-3 bg-sidebar rounded border font-mono h-full"
        onClick={() => inputRef.current?.focus()}
      >
        <pre className="data">
          {entries.map((entry) => (
            <>
              {entry.type === "input" && <>&gt;&gt;&gt; {entry.data}</>}
              {entry.type === "output" && (
                <>
                  {"    "}
                  <span dangerouslySetInnerHTML={{ __html: entry.data }}></span>
                </>
              )}
              {"\n"}
            </>
          ))}
        </pre>
        {loading || (
          <div className="flex">
            <pre>&gt;&gt;&gt; </pre>
            <form onSubmit={onSubmit} className="w-full">
              <input
                ref={inputRef}
                className="bg-transparent outline-none w-full"
                type="text"
                value={input}
                autoFocus={true}
                onChange={(e) => setInput(e.target.value)}
              />
            </form>
          </div>
        )}
      </div>
    </div>
  );
}
