import { FC } from "react";
import { LandmarkMetadata } from "../api/LandmarkMetadata";
import { Card, CardContent, Typography } from "@mui/material";
import { bullet } from "./bullet";
import { Link } from "react-router-dom";

export interface LandmarkCardProps {
  landmark: LandmarkMetadata;
}

export const LandmarkCard: FC<LandmarkCardProps> = ({ landmark }) => (
  <Card sx={{ minWidth: 275 }} variant="outlined">
    <CardContent>
      <Typography variant="h6" color="text.secondary" gutterBottom>
        {landmark.name}
      </Typography>
      <Typography variant="subtitle1" component="div">
        {bullet}
        Coordinates {"->"} X: {landmark.coordinate.x}, Y:{" "}
        {landmark.coordinate.y}, Z: {landmark.coordinate.z}
      </Typography>
      <Link to={`/landmarks/${landmark.id}`}>View landmark</Link>
    </CardContent>
  </Card>
);
