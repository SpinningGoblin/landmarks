import { getBackendUrl } from "../config";
import { CreateLandmark } from "./CreateLandmark";
import { LandmarkMetadata } from "../models/LandmarkMetadata";
import { User } from "./User";
import { Landmark } from "../models/Landmark";
import { request } from "./request";

const serverUrl = getBackendUrl();

export const fetchLandmark = async (
  landmarkId?: string,
  user?: User,
): Promise<Landmark> => {
  if (!user || !landmarkId) {
    return Promise.reject(new Error("No landmark id or user"));
  }

  const url = `${serverUrl}/landmarks/${landmarkId}`;
  return request<unknown, Landmark>(
    url,
    "GET",
    (response) => response.json(),
    user,
  );
};

export const addBiome = async (
  landmarkId?: string,
  biome?: string,
  user?: User,
): Promise<void> => {
  if (!landmarkId || !biome || !user) {
    return Promise.reject(new Error("Improper args"));
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/add_biome`;
  return request<unknown, void>(url, "GET", () => Promise.resolve(), user, {
    biome,
  });
};

export const addLandmark = async (
  create: CreateLandmark,
  worldId?: string,
  user?: User,
): Promise<string> => {
  if (!user || !worldId) {
    return Promise.reject(new Error("No world or user"));
  }

  const url = `${serverUrl}/worlds/${worldId}/landmarks`;

  return request<CreateLandmark, string>(
    url,
    "POST",
    (response) => response.text(),
    user,
    create,
  );
};

export const fetchLandmarks = async (
  worldId?: string,
  user?: User,
): Promise<LandmarkMetadata[]> => {
  if (!user || !worldId) {
    return [];
  }

  const url = `${serverUrl}/worlds/${worldId}/landmarks`;
  return request<unknown, LandmarkMetadata[]>(
    url,
    "GET",
    (response) => response.json(),
    user,
  );
};
