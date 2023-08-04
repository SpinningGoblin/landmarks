import { Name } from "../models/Name";
import { Platform } from "./Platform";
import { Seed } from "./Seed";
import { SlimUser } from "./SlimUser";
import { Tag } from "./Tag";

export interface WorldMetadata {
  id: string;
  seed: Seed;
  name: Name;
  tags: Tag[];
  platform: Platform;
  notes?: string;
  creator: SlimUser;
  shared_users: SlimUser[];
}
