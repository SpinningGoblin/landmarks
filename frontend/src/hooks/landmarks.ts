import { useQuery } from "@tanstack/react-query";
import { useUser } from "./auth";
import { fetchLandmarks } from "../api/landmarks";

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
