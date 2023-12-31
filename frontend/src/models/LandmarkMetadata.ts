import { Coordinate } from "./Coordinate";
import { Name } from "./Name";

export interface LandmarkMetadata {
  id: string;
  coordinate: Coordinate;
  name: Name;
  notes?: string;
}
