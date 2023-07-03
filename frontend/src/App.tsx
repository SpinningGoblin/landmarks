import { AddLandmark, Home, SignIn, World } from "./pages";
import { FC, useEffect, useState } from "react";
import { AuthProvider } from "./hooks/auth";
import { User } from "./api/User";

export interface AppProps {
  basePath: string;
  startingUser?: User;
}

export const App: FC<AppProps> = ({ startingUser }) => {
  const [user, setUser] = useState<User | undefined>(startingUser);
  const [worldId, setWorldId] = useState<string | undefined>();
  const [isAddingLandmark, setIsAddingLandmark] = useState<boolean>();

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
            }}
          />
        )}
        {signedIn && (
          <>
            {isAddingLandmark && worldId && <AddLandmark worldId={worldId} />}
            {worldId ? (
              <World
                worldId={worldId}
                onClickAddLandmark={() => setIsAddingLandmark(true)}
                onClickLandmark={(landmarkId) => console.log(landmarkId)}
              />
            ) : (
              <Home onClickWorld={setWorldId} />
            )}
          </>
        )}
      </AuthProvider>
    </>
  );
};
