import { Platform } from "../api/Platform";

export interface EditingWorld {
  seed: string;
  tags: string[];
  platform: Platform | "";
  name?: string;
  notes?: string;
}

export const buildEmptyEditingWorld = (): EditingWorld => ({
  seed: "",
  tags: [],
  platform: "",
  name: "",
  notes: "",
});
