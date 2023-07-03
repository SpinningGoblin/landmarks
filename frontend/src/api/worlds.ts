import { getBackendUrl } from "../config";
import { User } from "./User";
import { WorldMetadata } from "./WorldMetadata";
import { userHeaders } from "./headers";

const serverUrl = getBackendUrl();

export const fetchWorlds = async (user?: User): Promise<WorldMetadata[]> => {
  if (!user) {
    return [];
  }

  return fetch(`${serverUrl}/worlds`, {
    headers: userHeaders(user),
  }).then((response) => {
    if (response.ok) {
      return response.json();
    }

    return Promise.reject(new Error("world fetch failed"));
  });
};
