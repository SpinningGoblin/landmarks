import { Coordinate } from "../models/Coordinate";
import { Name } from "../models/Name";

export interface CreateLandmark {
  coordinate: Coordinate;
  name: Name;
  biomes: string[];
  dimension: string;
  notes?: string;
  tags: string[];
  farms: string[];
}
