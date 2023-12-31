import {
  AddLandmark,
  AddWorld,
  Home,
  Landmark,
  ShareWorld,
  SignIn,
  World,
} from "./pages";
import { FC, useEffect, useState } from "react";
import { AuthProvider } from "./hooks/auth";
import { User } from "./api/User";
import { Route, Routes, useNavigate } from "react-router-dom";

export interface AppProps {
  basePath: string;
  startingUser?: User;
}

export const App: FC<AppProps> = ({ startingUser }) => {
  const [user, setUser] = useState<User | undefined>(startingUser);
  const navigate = useNavigate();

  useEffect(() => {
    if (user) {
      localStorage.setItem("landmark-user", JSON.stringify(user));
    }
  }, [user]);

  const signedIn = !!user;

  return (
    <AuthProvider value={user}>
      {!signedIn && (
        <SignIn
          userChanged={(user) => {
            setUser(user);
            navigate("");
          }}
        />
      )}
      {signedIn && (
        <Routes>
          <Route
            path="/worlds/:worldId/add_landmark"
            element={<AddLandmark />}
          />
          <Route path="/add_world" element={<AddWorld />} />
          <Route path="/worlds/:worldId/share" element={<ShareWorld />} />
          <Route path="/worlds/:worldId" element={<World />} />
          <Route
            path="worlds/:worldId/landmarks/:landmarkId"
            element={<Landmark />}
          />
          <Route path="/worlds" element={<Home />} />
          <Route path="" element={<Home />} />
        </Routes>
      )}
    </AuthProvider>
  );
};
