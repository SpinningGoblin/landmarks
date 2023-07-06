import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useUser } from "./auth";
import { addWorld, fetchWorlds } from "../api/worlds";
import { useCallback, useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { CreateWorld } from "../api/CreateWorld";

export const useNavigateToWorld = () => {
  const navigate = useNavigate();

  return useCallback(
    (worldId?: string) => {
      if (worldId) {
        const url = `/worlds/${worldId}`;
        navigate(url);
      }
    },
    [navigate],
  );
};

export const useWorlds = () => {
  const { currentUser } = useUser();

  const { data: worlds, isLoading } = useQuery(
    ["worlds"],
    () => fetchWorlds(currentUser),
    {
      enabled: !!currentUser,
    },
  );

  return { worlds, isLoading };
};

export const useWorld = (worldId?: string) => {
  const { worlds, isLoading } = useWorlds();

  const world = useMemo(
    () => worlds?.find((world) => world.id === worldId),
    [worldId, worlds],
  );

  return { world, isLoading };
};

export const useAddWorld = (onSuccess: (newId: string) => void) => {
  const queryClient = useQueryClient();
  const { currentUser } = useUser();

  const mutation = useMutation({
    mutationFn: (create: CreateWorld) => addWorld(create, currentUser),
    onSuccess: (data) => {
      queryClient.invalidateQueries(["worlds"]);
      onSuccess(data);
    },
  });

  return { addWorld: mutation };
};
