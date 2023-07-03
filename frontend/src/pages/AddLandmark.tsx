import { FC } from "react";

interface AddLandmarkProps {
  worldId: string;
}

export const AddLandmark: FC<AddLandmarkProps> = ({ worldId }) => (
  <div>Woooooooo {worldId}</div>
);
