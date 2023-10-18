import { useQuery } from "@tanstack/react-query";
import { fetchDimensions } from "../api/dimensions";
import { fetchBiomes } from "../api/biomes";

export const useDimensions = () => {
  const { data: dimensions, isLoading } = useQuery({
    queryKey: ["dimensions"],
    queryFn: fetchDimensions,
  });

  return { dimensions, isLoading };
};

export const useBiomes = () => {
  const { data: biomes, isLoading } = useQuery({
    queryKey: ["biomes"],
    queryFn: fetchBiomes,
  });

  return { biomes, isLoading };
};
