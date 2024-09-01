import ky from "ky";

export interface Language {
  language: string;
  version: string;
  aliases: string[];
  runtime?: string;
}

export interface ExecuteOptions {
  language: string;
  version: string;
  files: {
    name?: string;
    content: string;
    encoding?: "base64" | "hex" | "utf8";
  }[];
  stdin?: string;
  args?: string[];
  run_timeout?: number;
  compile_timeout?: number;
  compile_memory_limit?: number;
  run_memory_limit?: number;
}

export interface Execution {
  language: string;
  version: string;
  run: {
    stdout: string;
    stderr: string;
    output: string;
    code: number | null;
    signal: string | null;
  };
  compile?: {
    stdout: string;
    stderr: string;
    output: string;
    code: number | null;
    signal: string | null;
  };
}

export async function execute(options: ExecuteOptions): Promise<Execution> {
  const response = await ky
    .post<Execution>("https://v2-api.nigga.church/code/execute", {
      json: options,
      headers: {
        Authorization: process.env.CODE_API_KEY!,
      },
      timeout: 60 * 1000, // 60 seconds
    })
    .json();

  return response;
}
