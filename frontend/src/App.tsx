import { AddLandmark, Home, SignIn } from "./pages";
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
  const [worldId, setWorldId] = useState<string | undefined>();
  const [isAddingLandmark, _] = useState<boolean>();
  const navigate = useNavigate();

  console.log("I'm here?");

  useEffect(() => {
    if (user) {
      localStorage.setItem("landmark-user", JSON.stringify(user));
    }
  }, [user]);

  const signedIn = !!user;

  return (
    <>
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
          <>
            <Routes>
              <Route path="" element={<Home onClickWorld={setWorldId} />} />
            </Routes>
            {isAddingLandmark && worldId && <AddLandmark worldId={worldId} />}
            {/* {worldId ? (
              <World
                worldId={worldId}
                onClickAddLandmark={() => setIsAddingLandmark(true)}
                onClickLandmark={(landmarkId) => console.log(landmarkId)}
              />
            ) : (
              <Home onClickWorld={setWorldId} />
            )} */}
          </>
        )}
      </AuthProvider>
    </>
  );
};
