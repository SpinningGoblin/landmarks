import { useQuery } from "@tanstack/react-query";
import { useUser } from "./auth";
import { fetchWorlds } from "../api/worlds";
import { useMemo } from "react";

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
