import { FC } from "react";
import { useAddWorld, useNavigateToWorld } from "../hooks/worlds";
import { Paper, Stack, Typography } from "@mui/material";
import { WorldForm } from "../components/WorldForm";
import { buildEmptyEditingWorld } from "../models/EditingWorld";

export const AddWorld: FC = () => {
  const navigateToWorld = useNavigateToWorld();
  const { addWorld } = useAddWorld(navigateToWorld);

  if (addWorld.isPending) {
    return <Typography variant="subtitle1">Saving ...</Typography>;
  }

  const world = buildEmptyEditingWorld();

  return (
    <Paper>
      <Stack spacing={2}>
        <Typography variant="h5">Add New World</Typography>
        <WorldForm world={world} onFormSubmit={addWorld.mutate} />
      </Stack>
    </Paper>
  );
};
