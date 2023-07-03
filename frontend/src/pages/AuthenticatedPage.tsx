import { FC, ReactNode } from "react";
import { useUser } from "../hooks/auth";
import { NotSignedIn } from "./NotSignedIn";

export interface AuthenticatedPageProps {
  children: ReactNode;
}

export const AuthenticatedPage: FC<AuthenticatedPageProps> = ({ children }) => {
  const { currentUser } = useUser();

  return currentUser ? children : <NotSignedIn />;
};
