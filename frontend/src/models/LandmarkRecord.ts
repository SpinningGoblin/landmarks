import { Coordinate } from "../api/Coordinate";
import { Name } from "../api/Name";

export interface LandmarkRecord {
  coordinate: Coordinate;
  name: Name;
  biomes: string[];
  dimension: string;
  notes?: string;
  tags: string[];
  farms: string[];
}
