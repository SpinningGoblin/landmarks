import { getBackendUrl } from "../config";

const serverUrl = getBackendUrl();

export const fetchDimensions = async (): Promise<string[]> => {
  const response = await fetch(`${serverUrl}/dimensions`);

  if (response.ok) {
    return response.json();
  }

  return Promise.reject(new Error("dimension fetch failed"));
};
