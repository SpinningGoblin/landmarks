import { Typography } from "@mui/material";
import { FC } from "react";

export interface DimensionProps {
  dimension: string;
}

export const Dimension: FC<DimensionProps> = ({ dimension }) => (
  <Typography variant="h6" textTransform="capitalize">
    Dimension: {dimension}
  </Typography>
);
