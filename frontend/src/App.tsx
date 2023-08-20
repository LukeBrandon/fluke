import { useState } from 'react';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import Typography from '@mui/material/Typography';
import CssBaseline from '@mui/material/CssBaseline';
import Container from "@mui/material/Container";
import Box from "@mui/material/Box";
import MessageDisplay from "./components/MessageDisplay";
import ChannelSelector from "./components/ChannelSelector";

export default function App() {
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });

    const [channelId, setChannelId] = useState<number | null>(null);

    const handleChannelSelect = (id: number) => {
        setChannelId(id);
    };

    return (
        <ThemeProvider theme={darkTheme}>
            <CssBaseline />
            <Container maxWidth="md" sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center', // centers content horizontally
                minHeight: '100vh', // ensures the container spans the entire height
                paddingTop: '2rem', // provides some spacing at the top
            }}>
                {/* Header */}
                <Box sx={{ mb: 5, width: '100%' }}>
                    <Typography variant="h4" component="h1" align="center" gutterBottom>
                        Fluke
                    </Typography>
                </Box>

                {/* Channel Selector - centered and made wider */}
                <Box sx={{ width: '80%', textAlign: 'center', mb: 3 }}>
                    <Typography variant="h6">Channel Selector:</Typography>
                    <ChannelSelector onChannelSelect={handleChannelSelect} />
                </Box>

                {/* Messages Display */}
                {channelId && (
                    <Box sx={{ width: '100%', mt: 3 }}>
                        <Typography variant="h6" align="center">Messages from Channel {channelId}:</Typography>
                        <MessageDisplay url={`http://localhost:8000/channels/${channelId}/messages`} />
                    </Box>
                )}
            </Container>
        </ThemeProvider>
    );
}
