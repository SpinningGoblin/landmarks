import { FC } from "react";
import { Coordinate as CoordinateModel } from "../models/Coordinate";
import { Typography } from "@mui/material";

export interface CoordinateProps {
  coordinate: CoordinateModel;
}

export const Coordinate: FC<CoordinateProps> = ({ coordinate }) => (
  <Typography variant="subtitle1" component="div">
    Coordinate {"->"} X: {coordinate.x}, Y: {coordinate.y}, Z: {coordinate.z}
  </Typography>
);
