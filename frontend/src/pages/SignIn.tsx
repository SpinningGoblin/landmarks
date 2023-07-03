import { ChangeEventHandler, FC, useCallback, useState } from "react";
import { User } from "../api/User";
import { Button, Container, Stack, TextField } from "@mui/material";

export interface SignInProps {
  userChanged: (user: User) => void;
}

export const SignIn: FC<SignInProps> = ({ userChanged }) => {
  const [name, setName] = useState<string>("");
  const [pass, setPass] = useState<string>("");

  const onNameChange: ChangeEventHandler<HTMLInputElement> = useCallback(
    (event) => {
      setName(event.currentTarget.value);
    },
    [setName],
  );

  const onPassChange: ChangeEventHandler<HTMLInputElement> = useCallback(
    (event) => {
      setPass(event.currentTarget.value);
    },
    [setPass],
  );

  return (
    <Container>
      <Stack spacing={2}>
        <TextField value={name} onChange={onNameChange} placeholder="Name" />
        <TextField
          value={pass}
          onChange={onPassChange}
          placeholder="Password"
          type="password"
        />
        <Button
          disabled={!pass || !name}
          onClick={() => userChanged({ name: name!, token: pass! })}
          variant="contained"
        >
          Sign In
        </Button>
      </Stack>
    </Container>
  );
};
