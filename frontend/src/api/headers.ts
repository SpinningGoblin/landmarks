import { User } from "./User";

export type UserHeaders = {
  "Landmark-User": string;
  Authorization: string;
};

export const userHeaders = (user: User): UserHeaders => ({
  "Landmark-User": user.name,
  Authorization: `Bearer ${user.token}`,
});
