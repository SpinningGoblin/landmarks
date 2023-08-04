import { getBackendUrl } from "../config";
import { CreateWorld } from "./CreateWorld";
import { ShareWorld } from "./ShareWorld";
import { User } from "./User";
import { WorldMetadata } from "./WorldMetadata";
import { request } from "./request";

const serverUrl = getBackendUrl();

export const addWorld = async (
  create: CreateWorld,
  user?: User,
): Promise<string> => {
  if (!user) {
    throw new Error("No user");
  }

  const url = `${serverUrl}/worlds`;
  return request<CreateWorld, string>(
    url,
    "POST",
    (response) => response.text(),
    user,
    create,
  );
};

export const shareWorldWithUser = async (
  share: ShareWorld,
  worldId: string,
  user?: User,
): Promise<string> => {
  if (!user) {
    throw new Error("No user");
  }

  const url = `${serverUrl}/worlds/${worldId}/share`;
  return request<ShareWorld, string>(
    url,
    "POST",
    (response) => response.text(),
    user,
    share,
  );
};

export const fetchWorlds = async (user?: User): Promise<WorldMetadata[]> => {
  if (!user) {
    return [];
  }

  const url = `${serverUrl}/worlds`;
  return request<unknown, WorldMetadata[]>(
    url,
    "GET",
    (response) => response.json(),
    user,
  );
};
