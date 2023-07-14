import { FC } from "react";
import { useParams } from "react-router-dom";
import { useLandmark } from "../hooks/landmarks";
import { Chip, Container, Stack, Typography } from "@mui/material";
import { Coordinate } from "../components/Coordinate";

export const Landmark: FC = () => {
  const { landmarkId } = useParams();

  const { landmark, isLoading } = useLandmark(landmarkId);

  return (
    <Container>
      {isLoading && <Typography>Loading ...</Typography>}
      {landmark && (
        <Stack spacing={4}>
          <Typography variant="h2">{landmark.metadata.name}</Typography>
          <Coordinate coordinate={landmark.metadata.coordinate} />
          {(landmark.biomes ?? []).length > 0 && (
            <Stack direction="row" spacing={1} alignItems={"center"}>
              <Typography variant="subtitle1">Biomes</Typography>
              <Stack direction="row" spacing={0.5}>
                {landmark.biomes.map((biome, index) => (
                  <Chip
                    key={index}
                    label={biome.replaceAll("_", " ")}
                    color="primary"
                  />
                ))}
              </Stack>
            </Stack>
          )}
          {(landmark.tags ?? []).length > 0 && (
            <Stack direction="row" spacing={1} alignItems={"center"}>
              <Typography variant="subtitle1">Tags</Typography>
              <Stack direction="row" spacing={0.5}>
                {landmark.tags.map((tag, index) => (
                  <Chip key={index} label={tag} />
                ))}
              </Stack>
            </Stack>
          )}
          {landmark.farms.length > 0 && (
            <Stack direction="row" spacing={1} alignItems={"center"}>
              <Typography variant="subtitle1">Farms</Typography>
              <Stack direction="row" spacing={0.5}>
                {landmark.farms.map((farm, index) => (
                  <Chip key={index} label={farm} color="secondary" />
                ))}
              </Stack>
            </Stack>
          )}
        </Stack>
      )}
    </Container>
  );
};
