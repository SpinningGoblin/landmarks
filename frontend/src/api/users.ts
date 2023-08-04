import { getBackendUrl } from "../config";
import { SlimUser } from "./SlimUser";
import { User } from "./User";
import { request } from "./request";

const serverUrl = getBackendUrl();

export const fetchSlimUsers = async (user?: User): Promise<SlimUser[]> => {
  if (!user) {
    return [];
  }

  const url = `${serverUrl}/users`;
  return request<unknown, SlimUser[]>(
    url,
    "GET",
    (response) => response.json(),
    user,
  );
};
