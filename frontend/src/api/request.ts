import { User } from "./User";
import { userHeaders } from "./headers";

export const request = async <T, U>(
  url: string,
  method: string,
  onSuccess: (response: Response) => Promise<U>,
  user?: User,
  body?: T,
): Promise<U> => {
  const serializedBody =
    method === "POST" && body ? JSON.stringify(body) : undefined;
  const authHeaders = user ? userHeaders(user) : {};
  const response = await fetch(url, {
    method,
    headers: {
      ...authHeaders,
      "Content-Type": "application/json",
    },
    body: serializedBody,
  });

  if (!response.ok) {
    return Promise.reject(new Error(await response.text()));
  }

  return onSuccess(response);
};
