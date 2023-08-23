import Header from "../components/Header";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import Container from "@mui/material/Container";
import {
    Outlet
} from "react-router-dom";

export default function MainLayout() {
  const darkTheme = createTheme({
    palette: {
      mode: "dark",
    },
  });
  return (
    <div>
      <ThemeProvider theme={darkTheme}>
        <CssBaseline />
        <Header title="Fluke" />
        <Container
          maxWidth="md"
          sx={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            minHeight: "100vh",
            paddingTop: "2rem",
          }}
        >
        <Outlet />
        </Container>
      </ThemeProvider>
    </div>
  );
}
