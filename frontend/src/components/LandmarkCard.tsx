import { FC } from "react";
import { LandmarkMetadata } from "../api/LandmarkMetadata";
import { Button, Card, CardContent, Typography } from "@mui/material";
import { bullet } from "./bullet";

export interface LandmarkCardProps {
  landmark: LandmarkMetadata;
  onClickLandmark: (landmarkId: string) => void;
}

export const LandmarkCard: FC<LandmarkCardProps> = ({
  landmark,
  onClickLandmark,
}) => (
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
      <Button onClick={() => onClickLandmark(landmark.id)}>
        View Landmark
      </Button>
    </CardContent>
  </Card>
);
