/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_LANDMARKS_URL: string;
  readonly VITE_FRONTEND_BASE_PATH: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
