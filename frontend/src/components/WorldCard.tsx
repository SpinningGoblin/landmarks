import { FC } from "react";
import { WorldMetadata } from "../api/WorldMetadata";
import { Card, CardContent, Typography } from "@mui/material";
import { bullet } from "./bullet";
import { Link } from "react-router-dom";

export interface WorldCardProps {
  world: WorldMetadata;
}

export const WorldCard: FC<WorldCardProps> = ({ world }) => (
  <Card sx={{ minWidth: 275 }} variant="outlined">
    <CardContent>
      <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
        {world.name}
      </Typography>
      <Typography variant="h6" component="div">
        {bullet}
        Seed - {world.seed}
      </Typography>
      <Typography variant="subtitle1">Saved by: {world.creator}</Typography>
      <Link to={`/worlds/${world.id}`}>View World</Link>
    </CardContent>
  </Card>
);
