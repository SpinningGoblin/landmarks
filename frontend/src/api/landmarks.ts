import { getBackendUrl } from "../config";
import { CreateLandmark } from "./CreateLandmark";
import { LandmarkMetadata } from "../models/LandmarkMetadata";
import { User } from "./User";
import { userHeaders } from "./headers";

const serverUrl = getBackendUrl();

export const addLandmark = async (
  create: CreateLandmark,
  worldId?: string,
  user?: User,
): Promise<string> => {
  if (!user || !worldId) {
    return Promise.reject(new Error("No world or user"));
  }

  const response = await fetch(`${serverUrl}/worlds/${worldId}/landmarks`, {
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
