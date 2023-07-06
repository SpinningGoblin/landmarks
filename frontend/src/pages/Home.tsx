import { FC } from "react";
import { useWorlds } from "../hooks/worlds";
import { Container, Stack, Typography } from "@mui/material";
import { WorldCard } from "../components/WorldCard";
import { Link } from "react-router-dom";

export const Home: FC = () => {
  const { worlds, isLoading } = useWorlds();

  return (
    <Container>
      <Stack spacing={2}>
        {isLoading && <Typography>Loading ...</Typography>}
        <Stack direction="row" spacing={4} alignItems="center">
          <Typography variant="h4">Worlds</Typography>
          <Link to={"/add_world"}>+ Add World</Link>
        </Stack>
        {worlds?.map((world) => (
          <WorldCard key={world.id} world={world} />
        ))}
      </Stack>
    </Container>
  );
};
