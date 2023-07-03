import { FC } from "react";
import { useWorlds } from "../hooks/worlds";
import { Container, Stack } from "@mui/material";
import { WorldCard } from "../components/WorldCard";

export const Home: FC = () => {
  const { worlds } = useWorlds();

  return (
    <Container>
      <Stack spacing={2}>
        {worlds?.map((world) => (
          <WorldCard key={world.id} world={world} />
        ))}
      </Stack>
    </Container>
  );
};
