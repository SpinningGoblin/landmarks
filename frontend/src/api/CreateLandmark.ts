import { Coordinate } from "./Coordinate";
import { Name } from "./Name";

export interface CreateLandmark {
  coordinate: Coordinate;
  name: Name;
  biomes: string[];
  dimension: string;
  notes?: string;
  tags: string[];
  farms: string[];
}
