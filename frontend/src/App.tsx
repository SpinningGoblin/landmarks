import { AddLandmark, AuthenticatedPage, Home, SignIn, World } from "./pages";
import { FC, useEffect, useState } from "react";
import { AuthProvider } from "./hooks/auth";
import { User } from "./api/User";
import { Route, Routes, useNavigate } from "react-router-dom";

export interface AppProps {
  basePath: string;
  startingUser?: User;
}

export const App: FC<AppProps> = ({ startingUser, basePath }) => {
  const [user, setUser] = useState<User | undefined>(startingUser);
  const navigate = useNavigate();

  console.log("I'm here?");

  useEffect(() => {
    if (user) {
      localStorage.setItem("landmark-user", JSON.stringify(user));
    }
  }, [user]);

  return (
    <>
      <AuthProvider value={user}>
        <Routes>
          <Route
            element={
              <SignIn
                userChanged={(user) => {
                  setUser(user);
                  navigate(`/${basePath}`);
                }}
              />
            }
            path="/sign-in"
          />
          <Route
            element={
              <AuthenticatedPage>
                <AddLandmark />
              </AuthenticatedPage>
            }
            path="/world/:worldId/add_landmark"
          />
          <Route
            element={
              <AuthenticatedPage>
                <World />
              </AuthenticatedPage>
            }
            path="/world/:worldId"
          />
          <Route
            element={
              <AuthenticatedPage>
                <Home />
              </AuthenticatedPage>
            }
            path=""
          />
        </Routes>
      </AuthProvider>
    </>
  );
};
