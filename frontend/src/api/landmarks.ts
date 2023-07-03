import { getBackendUrl } from "../config";
import { LandmarkMetadata } from "./LandmarkMetadata";
import { User } from "./User";
import { userHeaders } from "./headers";

const serverUrl = getBackendUrl();

export const fetchLandmarks = async (
  worldId?: string,
  user?: User,
): Promise<LandmarkMetadata[]> => {
  if (!user || !worldId) {
    return [];
  }

  return fetch(`${serverUrl}/worlds/${worldId}/landmarks`, {
    headers: userHeaders(user),
  }).then((response) => {
    if (response.ok) {
      return response.json();
    }

    return Promise.reject(new Error("landmark fetch failed"));
  });
};
