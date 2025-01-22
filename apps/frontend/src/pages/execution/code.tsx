import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { api } from "@/lib/utils";
import Editor from "@monaco-editor/react";
import { PlayIcon } from "lucide-react";
import { useState } from "react";

interface ExecuteResponse {
  language: string;
  version: string;
  run: ExecuteStage;
  compile: ExecuteStage | null;
}

interface ExecuteStage {
  stdout: string;
  stderr: string;
  output: string;
  code: number | null;
  signal: string | null;
}

export function ExecutionCode() {
  const [language, setLanguage] = useState("javascript");
  const [code, setCode] = useState("console.log('Hello, world!')");

  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<ExecuteResponse | null>({
    language: "javascript",
    version: "20.11.1",
    run: {
      stdout: "Hello, world!\n",
      stderr: "",
      output: "Hello, world!\n",
      code: 0,
      signal: null,
    },
    compile: null,
  });

  async function run() {
    setRunning(true);

    try {
      const response = await fetch(api("/code"), {
        method: "POST",
        headers: {
          Authorization: "wip",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          language,
          code,
        }),
      });

      const executeResponse: ExecuteResponse = await response.json();

      setResult(executeResponse);
    } catch (error) {
      console.error(error);
    } finally {
      setRunning(false);
    }
  }

  console.log(result);

  return (
    <div className="h-screen flex flex-col">
      <div className="p-3 flex space-x-3 border-b">
        <Button onClick={run} disabled={running}>
          <PlayIcon /> Run
        </Button>
        <Select value={language} onValueChange={(value) => setLanguage(value)}>
          <SelectTrigger className="w-[180px]">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>Languages</SelectLabel>
              <SelectItem value="javascript">JavaScript</SelectItem>
              <SelectItem value="cpp">C++</SelectItem>
              <SelectItem value="shell">Shell (bash)</SelectItem>
              <SelectItem value="rust">Rust</SelectItem>
              <SelectItem value="python">Python</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
      <div className="grid grid-cols-2 grow">
        <Editor
          theme="vs-dark"
          language={language}
          value={code}
          onChange={(value) => value && setCode(value)}
        />
        <div className="p-3">
          {result && (
            <div className="flex flex-col">
              <div className="space-y-2">
                <h3 className="scroll-m-20 text-2xl font-semibold tracking-tight">
                  {result.compile && result.compile.code !== 0
                    ? "Compile Error"
                    : "Output"}
                </h3>
                <pre className="rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold overflow-auto">
                  {result.run.output}
                </pre>
                <p className="text-xs text-muted-foreground">
                  Exited with code {result.run.code}
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
