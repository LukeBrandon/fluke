import Container from "@mui/material/Container";
import MainContainer from './components/MainContainer';
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import Link from "@mui/material/Link";
import MessageDisplay from "./components/MessageDisplay"

function Copyright() {
  return (
    <Typography variant="body2" color="text.secondary" align="center">
      {"Copyright ©"}
      <Link color="inherit" href="https://mui.com/">
        Fluke
      </Link>{" "}
      {new Date().getFullYear()}.
    </Typography>
  );
}

export default function App() {
  return (
    <Container maxWidth="sm">
      <Box sx={{ my: 4 }}>
      <MainContainer>
      <MessageDisplay url="http://localhost:8000/channels/2/messages"/>
      </MainContainer>
        <Copyright />
      </Box>
    </Container>
  );
}
