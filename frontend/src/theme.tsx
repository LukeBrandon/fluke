import { createTheme } from "@mui/material/styles";
import { red } from "@mui/material/colors";

// A central place to define a theme we can easily incorperate later
const theme = createTheme({
  palette: {
    primary: {
      main: "#556cd6",
    },
    secondary: {
      main: "#19857b",
    },
    error: {
      main: red.A400,
    },
  },
});

export default theme;
