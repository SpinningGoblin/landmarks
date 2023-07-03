import { FC } from "react";
import { WorldMetadata } from "../api/WorldMetadata";
import { Card, CardContent, Typography } from "@mui/material";
import { bullet } from "./bullet";
import { Link } from "wouter";

export interface WorldCardProps {
  world: WorldMetadata;
}

export const WorldCard: FC<WorldCardProps> = ({ world }) => (
  <Card sx={{ minWidth: 275 }} variant="outlined">
    <CardContent>
      <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
        World - {world.name}
      </Typography>
      <Typography variant="h6" component="div">
        {bullet}
        Seed - {world.seed}
      </Typography>
      <Link to={`/world/${world.id}`}>View world</Link>
    </CardContent>
  </Card>
);
