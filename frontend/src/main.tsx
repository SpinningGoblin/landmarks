import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
// import "./index.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { User } from "./api/User";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import { getBasePath } from "./config";
import { HashRouter } from "react-router-dom";

const getStoredUser = (): User | undefined => {
  const stored = localStorage.getItem("landmark-user");
  return stored ? JSON.parse(stored) : undefined;
};

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: false,
      cacheTime: Infinity,
    },
  },
});

const basePath = getBasePath();

console.log(basePath);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <HashRouter>
      <QueryClientProvider client={queryClient}>
        <App startingUser={getStoredUser()} basePath={basePath} />
      </QueryClientProvider>
    </HashRouter>
  </React.StrictMode>,
);
