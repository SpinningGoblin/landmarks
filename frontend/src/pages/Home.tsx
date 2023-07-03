import { FC } from "react";
import { useWorlds } from "../hooks/worlds";
import { Container, Stack } from "@mui/material";
import { WorldCard } from "../components/WorldCard";

interface HomeProps {
  onClickWorld: (worldId: string) => void;
}

export const Home: FC<HomeProps> = ({ onClickWorld }) => {
  const { worlds } = useWorlds();

  return (
    <Container>
      <Stack spacing={2}>
        {worlds?.map((world) => (
          <WorldCard key={world.id} world={world} onClickWorld={onClickWorld} />
        ))}
      </Stack>
    </Container>
  );
};
