import { getBackendUrl } from "../config";
import { request } from "./request";

const serverUrl = getBackendUrl();

export const fetchDimensions = async (): Promise<string[]> => {
  const url = `${serverUrl}/dimensions`;

  return request<unknown, string[]>(url, "GET", (response) => response.json());
};
