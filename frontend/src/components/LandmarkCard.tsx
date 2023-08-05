import { FC } from "react";
import { LandmarkMetadata } from "../models/LandmarkMetadata";
import { Card, CardContent, Stack, Typography } from "@mui/material";
import { Link } from "react-router-dom";
import { Coordinate } from "./Coordinate";

export interface LandmarkCardProps {
  landmark: LandmarkMetadata;
  worldId: string;
}

export const LandmarkCard: FC<LandmarkCardProps> = ({ landmark, worldId }) => (
  <Card sx={{ minWidth: 275 }} variant="outlined">
    <CardContent>
      <Typography variant="h6" color="text.secondary" gutterBottom>
        {landmark.name}
      </Typography>
      <Stack spacing={1}>
        <Typography variant="subtitle1" component="div">
          <Coordinate coordinate={landmark.coordinate} />
        </Typography>
        <Link to={`/worlds/${worldId}/landmarks/${landmark.id}`}>
          View Landmark
        </Link>
      </Stack>
    </CardContent>
  </Card>
);
