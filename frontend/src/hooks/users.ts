import { useQuery } from "@tanstack/react-query";
import { useUser } from "./auth";
import { fetchSlimUsers } from "../api/users";

export const useUsers = () => {
  const { currentUser } = useUser();

  const { data: users, isLoading } = useQuery({
    queryKey: ["users"],
    queryFn: () => fetchSlimUsers(currentUser),
    enabled: !!currentUser,
  });

  return { users, isLoading };
};
