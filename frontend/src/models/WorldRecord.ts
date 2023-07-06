import { Platform } from "../api/Platform";

export interface WorldRecord {
  seed: string;
  tags: string[];
  platform: Platform;
  name?: string;
  notes?: string;
}
