import { FC } from "react";
import { useWorld } from "../hooks/worlds";
import { Chip, Paper, Stack, Typography } from "@mui/material";
import { useLandmarks } from "../hooks/landmarks";
import { LandmarkCard } from "../components/LandmarkCard";
import { Link, useParams } from "react-router-dom";

export interface WorldProps {
  worldId: string;
}

export const WorldContainer: FC = () => {
  const { worldId } = useParams();

  if (!worldId) {
    return <Paper>World ID needed for world page</Paper>;
  }

  return <World worldId={worldId} />;
};

export const World: FC<WorldProps> = ({ worldId }) => {
  const { world, isLoading: isWorldLoading } = useWorld(worldId);
  const { landmarks, isLoading: isLandmarksLoading } = useLandmarks(worldId);

  return (
    <Paper>
      {(isWorldLoading || isLandmarksLoading) && (
        <Typography>Loading ...</Typography>
      )}
      {!isWorldLoading && !isLandmarksLoading && (
        <Stack spacing={4}>
          <Typography variant="h2">{world?.name}</Typography>
          {(world?.tags ?? []).length > 0 && (
            <Stack direction="row" spacing={1} alignItems={"center"}>
              <Typography variant="subtitle1">Tags</Typography>
              <Stack>
                {world?.tags.map((tag, index) => (
                  <Chip key={index} label={tag} />
                ))}
              </Stack>
            </Stack>
          )}
          <Stack spacing={1}>
            <Stack direction="row" spacing={4} alignItems="center">
              <Typography variant="h4">Landmarks</Typography>
              <Link to={`/world/${worldId}/add_landmark`}>+ Add Landmark</Link>
            </Stack>
            <Stack spacing={2}>
              {landmarks?.map((landmark) => (
                <LandmarkCard key={landmark.id} landmark={landmark} />
              ))}
            </Stack>
          </Stack>
        </Stack>
      )}
    </Paper>
  );
};
