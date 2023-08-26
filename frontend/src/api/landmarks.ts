import { getBackendUrl } from "../config";
import { CreateLandmark } from "./CreateLandmark";
import { LandmarkMetadata } from "../models/LandmarkMetadata";
import { User } from "./User";
import { Landmark } from "../models/Landmark";
import { request } from "./request";
import { LinkLandmarks } from "./LinkLandmarks";

const serverUrl = getBackendUrl();

export const fetchLandmark = async (
  landmarkId?: string,
  user?: User,
): Promise<Landmark> => {
  if (!user || !landmarkId) {
    throw new Error("No landmark id or user");
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
    throw new Error("Improper args");
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/add_biome`;
  return request<unknown, void>(url, "POST", () => Promise.resolve(), user, {
    biome,
  });
};

export const addFarm = async (
  landmarkId?: string,
  farm?: string,
  user?: User,
): Promise<void> => {
  if (!landmarkId || !farm || !user) {
    throw new Error("Improper args");
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/add_farm`;
  return request<unknown, void>(url, "POST", () => Promise.resolve(), user, {
    farm,
  });
};

export const addTag = async (
  landmarkId?: string,
  tag?: string,
  user?: User,
): Promise<void> => {
  if (!landmarkId || !tag || !user) {
    throw new Error("Improper args");
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/add_tag`;
  return request<unknown, void>(url, "POST", () => Promise.resolve(), user, {
    tag,
  });
};

export const removeBiome = async (
  landmarkId?: string,
  biome?: string,
  user?: User,
): Promise<void> => {
  if (!landmarkId || !biome || !user) {
    throw new Error("Improper args");
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/remove_biome`;
  return request<unknown, void>(url, "POST", () => Promise.resolve(), user, {
    biome,
  });
};

export const removeFarm = async (
  landmarkId?: string,
  farm?: string,
  user?: User,
): Promise<void> => {
  if (!landmarkId || !farm || !user) {
    throw new Error("Improper args");
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/remove_farm`;
  return request<unknown, void>(url, "POST", () => Promise.resolve(), user, {
    farm,
  });
};

export const removeTag = async (
  landmarkId?: string,
  tag?: string,
  user?: User,
): Promise<void> => {
  if (!landmarkId || !tag || !user) {
    throw new Error("Improper args");
  }

  const url = `${serverUrl}/landmarks/${landmarkId}/remove_tag`;
  return request<unknown, void>(url, "POST", () => Promise.resolve(), user, {
    tag,
  });
};

export const addLandmark = async (
  create: CreateLandmark,
  worldId?: string,
  user?: User,
): Promise<string> => {
  if (!user || !worldId) {
    throw new Error("No world or user");
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

export const fetchLandmarkLinkTypes = async (): Promise<string[]> => {
  const url = `${serverUrl}/landmarks/link_types`;
  return request<unknown, string[]>(url, "GET", (response) => response.json());
};

export const linkLandmarks = async (
  linkRequest: LinkLandmarks,
  user?: User,
): Promise<string> => {
  if (!user || !linkRequest.landmark_id_1 || !linkRequest.landmark_id_2) {
    throw new Error("No user or landmark IDs");
  }

  const url = `${serverUrl}/landmarks/link`;
  return request<LinkLandmarks, string>(
    url,
    "POST",
    (response) => response.text(),
    user,
    linkRequest,
  );
};
