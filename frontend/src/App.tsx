import { useState } from 'react';
import Container from "@mui/material/Container";
import MainContainer from './components/MainContainer';
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import Link from "@mui/material/Link";
import MessageDisplay from "./components/MessageDisplay";
import ChannelSelector from "./components/ChannelSelector";

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
    const [channelId, setChannelId] = useState<number | null>(null);

    const handleChannelSelect = (id: number) => {
        setChannelId(id);
    };
    return (
        <Container maxWidth="sm">
            <Box sx={{ my: 4 }}>
                <MainContainer>
                    <div>
                        <h2>Channel Selector:</h2>
                        <br></br>
                        <ChannelSelector onChannelSelect={handleChannelSelect} />
                        {channelId && (
                            <>
                                <h2>Messages from Channel {channelId}:</h2>
                                <MessageDisplay url={`http://localhost:8000/channels/${channelId}/messages`} />
                            </>
                        )}
                    </div>
                </MainContainer>
                <Copyright />
            </Box>
        </Container>
    );
}
