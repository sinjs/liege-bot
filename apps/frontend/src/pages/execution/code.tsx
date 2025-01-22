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
import { useAuth } from "@/hooks/use-auth";
import { api } from "@/lib/utils";
import Editor from "@monaco-editor/react";
import { PlayIcon, UtensilsIcon } from "lucide-react";
import { useEffect, useState } from "react";

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

const defaultCode: Record<string, string> = {
  javascript: `function greet(name) {
    console.log("Hello, " + name + "!");
}
greet("Marcus");`,

  cpp: `#include <iostream>
#include <string>

void greet(const std::string& name) {
    std::cout << "Hello, " << name << "!" << std::endl;
}

int main() {
    greet("Marcus");
}`,

  shell: `greet() {
  echo "Hello, $1!"
}

greet Marcus`,

  rust: `fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    greet("Marcia");
}`,

  python: "# I hate python so no default code",
};

export function ExecutionCode() {
  const { auth } = useAuth();

  const [language, setLanguage] = useState("javascript");
  const [code, setCode] = useState(defaultCode[language]);
  const [isDirty, setDirty] = useState(false);

  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<ExecuteResponse | null>(null);

  useEffect(() => {
    if (!isDirty) setCode(defaultCode[language]);
  }, [language, isDirty]);

  useEffect(() => {
    setDirty(code !== defaultCode[language]);
  }, [code, language]);

  async function run() {
    setRunning(true);

    try {
      const response = await fetch(api("/code"), {
        method: "POST",
        headers: {
          Authorization: `Bearer ${auth?.token}`,
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

  function reset() {
    setCode(defaultCode[language]);
  }

  return (
    <div className="h-screen flex flex-col">
      <div className="p-3 flex space-x-3 border-b">
        <Button onClick={run} disabled={running}>
          <PlayIcon /> Run
        </Button>
        <Button onClick={reset} variant="outline">
          <UtensilsIcon /> Reset
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
