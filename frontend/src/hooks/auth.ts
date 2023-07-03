import { createContext, useContext } from "react";
import { User } from "../api/User";

const AuthContext = createContext<User | undefined>(undefined);
export const AuthProvider = AuthContext.Provider;

export const useUser = () => {
  const currentUser = useContext(AuthContext);

  return { currentUser };
};
