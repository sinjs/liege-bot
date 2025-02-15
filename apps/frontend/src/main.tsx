import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./app.tsx";
import { AuthProvider } from "./components/auth-provider.tsx";
import { ErrorBoundary } from "react-error-boundary";
import { Error as ErrorPage } from "./pages/error.tsx";
import { ThemeProvider } from "./components/theme-provider.tsx";
import { Loading } from "./pages/loading.tsx";

import { loader } from "@monaco-editor/react";

import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import { ConfigProvider } from "./components/config-provider.tsx";
import { initDev } from "./lib/dev.ts";

initDev();

self.MonacoEnvironment = {
  getWorker(_, label) {
    if (label === "json") {
      return new jsonWorker();
    }
    if (label === "css" || label === "scss" || label === "less") {
      return new cssWorker();
    }
    if (label === "html" || label === "handlebars" || label === "razor") {
      return new htmlWorker();
    }
    if (label === "typescript" || label === "javascript") {
      return new tsWorker();
    }
    return new editorWorker();
  },
};

loader.config({ monaco });

loader.init();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ErrorBoundary FallbackComponent={ErrorPage}>
      <ThemeProvider defaultTheme="dark" storageKey="ui-theme">
        <ConfigProvider fallback={<Loading message="Configuring" />}>
          <AuthProvider fallback={<Loading message="Authenticating" />}>
            <App />
          </AuthProvider>
        </ConfigProvider>
      </ThemeProvider>
    </ErrorBoundary>
  </StrictMode>
);
