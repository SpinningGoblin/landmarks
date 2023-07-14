import { Name } from "./Name";
import { EditingCoordinate } from "./EditingCoordinate";

export interface EditingLandmark {
  coordinate: EditingCoordinate;
  name: Name;
  biomes: string[];
  dimension: string;
  notes?: string;
  tags: string[];
  farms: string[];
}

export const buildEmptyEditingLandmark = (): EditingLandmark => ({
  coordinate: {
    x: "",
    y: "",
    z: "",
  },
  name: "",
  biomes: [],
  dimension: "",
  notes: "",
  tags: [],
  farms: [],
});
