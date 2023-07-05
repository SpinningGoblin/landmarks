import { getBackendUrl } from "../config";

const serverUrl = getBackendUrl();

export const fetchBiomes = async (): Promise<string[]> => {
  const response = await fetch(`${serverUrl}/biomes`);

  if (response.ok) {
    return response.json();
  }

  return Promise.reject(new Error("biome fetch failed"));
};
