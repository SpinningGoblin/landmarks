import { Redirect, Route, Router, useLocation } from "wouter";
import { getBasePath } from "./config";
import { AddLandmark, Home, SignIn, World } from "./pages";
import { FC, useEffect, useState } from "react";
import { AuthProvider } from "./hooks/auth";
import { User } from "./api/User";

export interface AppProps {
  startingUser?: User;
}

export const App: FC<AppProps> = ({ startingUser }) => {
  const basePath = getBasePath();
  const [user, setUser] = useState<User | undefined>(startingUser);
  const [_, navigate] = useLocation();

  useEffect(() => {
    if (user) {
      localStorage.setItem("landmark-user", JSON.stringify(user));
    }
  }, [user]);

  return (
    <>
      <AuthProvider value={user}>
        <Router base={basePath}>
          <Route path="/sign-in">
            <SignIn
              userChanged={(user) => {
                setUser(user);
                navigate(`/${basePath}`);
              }}
            />
          </Route>
          <Route path="/world/:worldId/add_landmark">
            {(params) => <AddLandmark worldId={params.worldId}></AddLandmark>}
          </Route>
          <Route path="/world/:worldId">
            {(params) => <World worldId={params.worldId}></World>}
          </Route>
          <Route path="">{user ? <Home /> : <Redirect to="/sign-in" />}</Route>
        </Router>
      </AuthProvider>
    </>
  );
};
