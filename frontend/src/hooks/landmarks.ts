import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useUser } from "./auth";
import {
  addBiome,
  addFarm,
  addLandmark,
  addTag,
  fetchLandmark,
  fetchLandmarks,
  removeBiome,
  removeFarm,
  removeTag,
} from "../api/landmarks";
import { CreateLandmark } from "../api/CreateLandmark";

export const useLandmarks = (worldId?: string) => {
  const { currentUser } = useUser();
  const { data: landmarks, isLoading } = useQuery(
    ["landmarks", worldId],
    () => fetchLandmarks(worldId, currentUser),
    {
      enabled: !!worldId && !!currentUser,
    },
  );

  return { landmarks, isLoading };
};

export const useLandmark = (landmarkId?: string) => {
  const { currentUser } = useUser();

  const { data: landmark, isLoading } = useQuery(
    ["landmarks", landmarkId],
    () => fetchLandmark(landmarkId, currentUser),
    {
      enabled: !!landmarkId && !!currentUser,
    },
  );

  return { landmark, isLoading };
};

export const useAddBiome = (onSuccess: () => void, landmarkId?: string) => {
  const { currentUser } = useUser();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (biome: string) => addBiome(landmarkId, biome, currentUser),
    onSuccess: () => {
      queryClient.invalidateQueries(["landmarks", landmarkId]);
      onSuccess();
    },
  });

  return { addBiome: mutation };
};

export const useRemoveBiome = (landmarkId?: string) => {
  const { currentUser } = useUser();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (biome: string) => removeBiome(landmarkId, biome, currentUser),
    onSuccess: () => queryClient.invalidateQueries(["landmarks", landmarkId]),
  });

  return { removeBiome: mutation };
};

export const useAddFarm = (onSuccess: () => void, landmarkId?: string) => {
  const { currentUser } = useUser();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (farm: string) => addFarm(landmarkId, farm, currentUser),
    onSuccess: () => {
      queryClient.invalidateQueries(["landmarks", landmarkId]);
      onSuccess();
    },
  });

  return { addFarm: mutation };
};

export const useRemoveFarm = (landmarkId?: string) => {
  const { currentUser } = useUser();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (farm: string) => removeFarm(landmarkId, farm, currentUser),
    onSuccess: () => queryClient.invalidateQueries(["landmarks", landmarkId]),
  });

  return { removeFarm: mutation };
};

export const useAddTag = (onSuccess: () => void, landmarkId?: string) => {
  const { currentUser } = useUser();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (tag: string) => addTag(landmarkId, tag, currentUser),
    onSuccess: () => {
      queryClient.invalidateQueries(["landmarks", landmarkId]);
      onSuccess();
    },
  });

  return { addTag: mutation };
};

export const useRemoveTag = (landmarkId?: string) => {
  const { currentUser } = useUser();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (tag: string) => removeTag(landmarkId, tag, currentUser),
    onSuccess: () => queryClient.invalidateQueries(["landmarks", landmarkId]),
  });

  return { removeTag: mutation };
};

export const useAddLandmark = (onSuccess: () => void, worldId?: string) => {
  const queryClient = useQueryClient();
  const { currentUser } = useUser();

  const mutation = useMutation({
    mutationFn: (create: CreateLandmark) =>
      addLandmark(create, worldId, currentUser),
    onSuccess: () => {
      queryClient.invalidateQueries(["landmarks", worldId]);
      onSuccess();
    },
  });

  return { addLandmark: mutation };
};
