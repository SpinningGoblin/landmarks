import { Tag } from "./Tag";

export interface LandmarkFilters {
  dimension?: string;
  tags?: Tag[];
  farms?: string[];
}
