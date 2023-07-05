import { useQuery } from "@tanstack/react-query";
import { fetchDimensions } from "../api/dimensions";
import { fetchBiomes } from "../api/biomes";

export const useDimensions = () => {
  const { data: dimensions, isLoading } = useQuery(
    ["dimensions"],
    fetchDimensions,
  );

  return { dimensions, isLoading };
};

export const useBiomes = () => {
  const { data: biomes, isLoading } = useQuery(["biomes"], fetchBiomes);

  return { biomes, isLoading };
};
