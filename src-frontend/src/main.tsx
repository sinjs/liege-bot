import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./app.tsx";
import { AuthProvider } from "./components/auth-provider.tsx";
import { ErrorBoundary } from "react-error-boundary";
import { Error as ErrorPage } from "./pages/error.tsx";
import { ThemeProvider } from "./components/theme-provider.tsx";
import { Loading } from "./pages/loading.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ErrorBoundary FallbackComponent={ErrorPage}>
      <ThemeProvider defaultTheme="dark" storageKey="ui-theme">
        <AuthProvider fallback={<Loading message="Authenticating" />}>
          <App />
        </AuthProvider>
      </ThemeProvider>
    </ErrorBoundary>
  </StrictMode>
);
