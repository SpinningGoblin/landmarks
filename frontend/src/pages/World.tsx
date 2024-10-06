import { FC } from "react";
import { useWorld } from "../hooks/worlds";
import { Chip, Container, Stack, Typography } from "@mui/material";
import { useLandmarks } from "../hooks/landmarks";
import { LandmarkCard } from "../components/LandmarkCard";
import { Link, useParams } from "react-router-dom";

export const World: FC = () => {
  const { worldId } = useParams();
  const { world, isLoading: isWorldLoading } = useWorld(worldId);
  const { landmarks, isLoading: isLandmarksLoading } = useLandmarks(worldId);

  return (
    <Container>
      {(isWorldLoading || isLandmarksLoading) && (
        <Typography>Loading ...</Typography>
      )}
      {!isWorldLoading && !isLandmarksLoading && (
        <Stack spacing={4}>
          <Typography variant="h2">{world?.name}</Typography>
          {(world?.tags ?? []).length > 0 && (
            <Stack direction="row" spacing={1} alignItems={"center"}>
              <Typography variant="subtitle1">Tags</Typography>
              <Stack direction="row">
                {world?.tags.map((tag, index) => (
                  <Chip key={index} label={tag} />
                ))}
              </Stack>
            </Stack>
          )}
          <Stack spacing={1}>
            <Stack direction="row" spacing={4} alignItems="center">
              <Typography variant="h4">Landmarks</Typography>
              <Link to={`/worlds/${worldId}/add_landmark`}>+ Add Landmark</Link>
            </Stack>
            <Stack spacing={2}>
              {worldId &&
                landmarks?.map((landmark) => (
                  <LandmarkCard
                    key={landmark.id}
                    landmark={landmark}
                    worldId={worldId}
                  />
                ))}
            </Stack>
          </Stack>
        </Stack>
      )}
    </Container>
  );
};
