import { FC } from "react";
import { WorldMetadata } from "../api/WorldMetadata";
import { Card, CardContent, Stack, Typography } from "@mui/material";
import { bullet } from "./bullet";
import { Link } from "react-router-dom";

export interface WorldCardProps {
  world: WorldMetadata;
}

export const WorldCard: FC<WorldCardProps> = ({ world }) => (
  <Card sx={{ minWidth: 275 }} variant="outlined">
    <CardContent>
      <Stack direction="row" alignItems="center" spacing={2}>
        <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
          {world.name}
        </Typography>
        <Link to={`/worlds/${world.id}/share`}>Share</Link>
      </Stack>
      <Typography variant="h6" component="div">
        {bullet}
        Seed - {world.seed}
      </Typography>
      <Typography variant="subtitle1">
        Saved by: {world.creator.name}
      </Typography>
      <Link to={`/worlds/${world.id}`}>View World</Link>
    </CardContent>
  </Card>
);
