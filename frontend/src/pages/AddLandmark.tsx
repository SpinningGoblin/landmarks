import { FC } from "react";
import { useParams } from "react-router-dom";

export const AddLandmark: FC = () => {
  const { worldId } = useParams();
  return <div>Woooooooo {worldId}</div>;
};
