import { BrowserRouter, Routes, Route } from "react-router";
import { Layout } from "@/layout";
import { Index } from "@/pages";
import { AIChat } from "@/pages/ai/chat";
import { AIImage } from "@/pages/ai/image";
import { ExecutionCode } from "@/pages/execution/code";
import { ExecutionMath } from "@/pages/execution/math";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Index />} />

          <Route path="ai">
            <Route path="chat" element={<AIChat />} />
            <Route path="image" element={<AIImage />} />
          </Route>

          <Route path="execution">
            <Route path="code" element={<ExecutionCode />} />
            <Route path="math" element={<ExecutionMath />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
