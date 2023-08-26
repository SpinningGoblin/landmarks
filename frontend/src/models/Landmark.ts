import { LandmarkLink } from "./LandmarkLink";
import { LandmarkMetadata } from "./LandmarkMetadata";

export interface Landmark {
  metadata: LandmarkMetadata;
  farms: string[];
  tags: string[];
  biomes: string[];
  dimension: string;
  links: LandmarkLink[];
}
