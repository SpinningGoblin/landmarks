import { FC } from "react";

export interface AddLandmarkProps {
  worldId: string;
}

export const AddLandmark: FC<AddLandmarkProps> = ({ worldId }) => (
  <div>Woooooooo {worldId}</div>
);
