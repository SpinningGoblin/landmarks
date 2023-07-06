import { Platform } from "./Platform";

export interface CreateWorld {
  seed: string;
  tags: string[];
  platform: Platform;
  name?: string;
  notes?: string;
}
