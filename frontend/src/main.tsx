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
    <QueryClientProvider client={queryClient}>
      <App startingUser={getStoredUser()} basePath={basePath} />
    </QueryClientProvider>
  </React.StrictMode>,
);
