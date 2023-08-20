import * as React from "react";
import Toolbar from "@mui/material/Toolbar";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import Typography from "@mui/material/Typography";
import { Link } from "react-router-dom";

interface HeaderProps {
  sections: ReadonlyArray<{
    title: string;
    url: string;
  }>;
  title: string;
}

export default function Header(props: HeaderProps) {
  const { title } = props;

  return (
    <React.Fragment>
      <Toolbar sx={{ justifyContent: "space-between", width: "100%" }}>
        <Typography
          variant="h4"
          component="h1"
          gutterBottom
          component={Link}
          to="/"
          sx={{ color: 'white', textDecoration: 'none' }}
        >
          {title}
        </Typography>
        <ButtonGroup variant="text" aria-label="outlined button group">
          <Button component={Link} to="/messages">
            Messages
          </Button>
          <Button component={Link} to="/login">
            Log In
          </Button>
          <Button component={Link} to="/signup">
            Sign Up
          </Button>
        </ButtonGroup>
      </Toolbar>
    </React.Fragment>
  );
}
