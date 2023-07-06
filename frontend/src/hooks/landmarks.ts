import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useUser } from "./auth";
import { addLandmark, fetchLandmarks } from "../api/landmarks";
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
