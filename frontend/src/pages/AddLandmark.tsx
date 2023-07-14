import { FC } from "react";
import { useParams } from "react-router-dom";
import { useNavigateToWorld, useWorld } from "../hooks/worlds";
import { Container, Stack, Typography } from "@mui/material";
import { useAddLandmark } from "../hooks/landmarks";
import { LandmarkForm } from "../components/LandmarkForm";
import { buildEmptyEditingLandmark } from "../models/EditingLandmark";

export const AddLandmark: FC = () => {
  const { worldId } = useParams();
  const { world, isLoading } = useWorld(worldId);
  const navigateToWorld = useNavigateToWorld();
  const { addLandmark } = useAddLandmark(
    () => navigateToWorld(worldId),
    worldId,
  );

  const landmark = buildEmptyEditingLandmark();

  if (isLoading || addLandmark.isLoading) {
    return <Typography>Loading ...</Typography>;
  }

  if (addLandmark.isError) {
    console.error(addLandmark.error);
  }

  return (
    <>
      {!world && <Typography>Unknown world ID</Typography>}
      {world && (
        <Container>
          <Stack spacing={2}>
            <Typography variant="h5">Add Landmark to {world.name}</Typography>
            <LandmarkForm
              landmark={landmark}
              formTitle="New Landmark Details"
              onFormSubmit={addLandmark.mutate}
            />
          </Stack>
        </Container>
      )}
    </>
  );
};
