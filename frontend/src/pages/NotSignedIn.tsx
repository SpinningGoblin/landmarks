import { Container, Typography } from "@mui/material";
import { FC } from "react";
import { Link } from "react-router-dom";

export const NotSignedIn: FC = () => (
  <Container style={{ height: "100vh" }}>
    <Typography variant="subtitle1">
      You need to be signed in to use the app.
    </Typography>
    <Typography variant="body1">
      <Link to="/sign-in">Sign In</Link>
    </Typography>
  </Container>
);
