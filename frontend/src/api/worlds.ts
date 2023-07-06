import { getBackendUrl } from "../config";
import { CreateWorld } from "./CreateWorld";
import { User } from "./User";
import { WorldMetadata } from "./WorldMetadata";
import { userHeaders } from "./headers";

const serverUrl = getBackendUrl();

export const addWorld = async (
  create: CreateWorld,
  user?: User,
): Promise<string> => {
  if (!user) {
    return Promise.reject(new Error("No user"));
  }

  const response = await fetch(`${serverUrl}/worlds`, {
    headers: {
      ...userHeaders(user),
      "Content-Type": "application/json",
    },
    method: "POST",
    body: JSON.stringify(create),
  });

  if (response.ok) {
    return response.text();
  }

  return Promise.reject(new Error(await response.text()));
};

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
