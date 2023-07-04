import { FC } from "react";
import { useWorlds } from "../hooks/worlds";
import { Container, Stack, Typography } from "@mui/material";
import { WorldCard } from "../components/WorldCard";

export const Home: FC = () => {
  const { worlds, isLoading } = useWorlds();

  return (
    <Container>
      <Stack spacing={2}>
        {isLoading && <Typography>Loading ...</Typography>}
        {worlds?.map((world) => (
          <WorldCard key={world.id} world={world} />
        ))}
      </Stack>
    </Container>
  );
};
