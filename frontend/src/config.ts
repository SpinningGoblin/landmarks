const backendUrl = import.meta.env.VITE_LANDMARKS_URL;
const basePath = import.meta.env.VITE_FRONTEND_BASE_PATH;

export const getBackendUrl = () => backendUrl;
export const getBasePath = () => basePath;
