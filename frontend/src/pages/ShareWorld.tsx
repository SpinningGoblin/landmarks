import {
  Button,
  Chip,
  Container,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Typography,
} from "@mui/material";
import { FC, useMemo, useState } from "react";
import { useParams } from "react-router-dom";
import { useUsers } from "../hooks/users";
import { useShareWorld, useWorld } from "../hooks/worlds";

export const ShareWorld: FC = () => {
  const { worldId } = useParams();
  const { users, isLoading } = useUsers();
  const { world, isLoading: worldLoading } = useWorld(worldId);
  const [newUser, setNewUser] = useState<string>("");
  const { shareWorld } = useShareWorld();

  const sharedUsers = useMemo(() => {
    if (!world) {
      return [];
    }
    return Array.from(
      new Set([
        ...world.shared_users.map((u) => u.name),
        world.creator.name,
      ]).values(),
    );
  }, [world]);

  const handleSaveClick = () => {
    if (!worldId || !newUser) {
      return;
    }

    shareWorld.mutate({ user: newUser, worldId });
  };

  const anyLoading = isLoading || worldLoading || shareWorld.isPending;

  return (
    <Container>
      {anyLoading && <Typography>Loading ...</Typography>}
      {!anyLoading && sharedUsers.length > 0 && (
        <Stack spacing={2}>
          <Stack>
            <Typography variant="h5">Currently Shared With</Typography>
            <Stack spacing={2} direction="row" alignItems="center">
              {sharedUsers.map((user, index) => (
                <Chip key={index} label={user} color="primary" />
              ))}
            </Stack>
          </Stack>
          <FormControl>
            <InputLabel id="share-with-user-label">Share With User</InputLabel>
            <Select
              labelId="share-with-user-label"
              id="share-with-user-select"
              value={newUser}
              label="User"
              onChange={(event) => setNewUser(event.target.value)}
            >
              {(users ?? [])
                .map((u) => u.name)
                .filter((name) => !sharedUsers.includes(name))
                .map((name) => (
                  <MenuItem key={name} value={name}>
                    {name}
                  </MenuItem>
                ))}
            </Select>
          </FormControl>
          <Button
            disabled={!newUser}
            onClick={handleSaveClick}
            variant="contained"
          >
            Share with user
          </Button>
        </Stack>
      )}
    </Container>
  );
};
