import { FC } from "react";
import { WorldMetadata } from "../api/WorldMetadata";
import { Button, Card, CardContent, Typography } from "@mui/material";
import { bullet } from "./bullet";

export interface WorldCardProps {
  world: WorldMetadata;
  onClickWorld: (worldId: string) => void;
}

export const WorldCard: FC<WorldCardProps> = ({ world, onClickWorld }) => (
  <Card sx={{ minWidth: 275 }} variant="outlined">
    <CardContent>
      <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
        World - {world.name}
      </Typography>
      <Typography variant="h6" component="div">
        {bullet}
        Seed - {world.seed}
      </Typography>
      <Button onClick={() => onClickWorld(world.id)}>View World</Button>
    </CardContent>
  </Card>
);
