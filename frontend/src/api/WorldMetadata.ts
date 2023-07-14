import { Name } from "../models/Name";
import { Platform } from "./Platform";
import { Seed } from "./Seed";
import { Tag } from "./Tag";

export interface WorldMetadata {
  id: string;
  seed: Seed;
  name: Name;
  tags: Tag[];
  platform: Platform;
  notes?: string;
  creator: string;
}
