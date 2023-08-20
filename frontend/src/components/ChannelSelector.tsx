import { useState, useEffect, useCallback } from 'react';
import { Select, MenuItem, FormControl, InputLabel, SelectChangeEvent } from '@mui/material';
import Box from '@mui/material/Box';

interface Channel {
    id: number;
    name: string;
    created_at: string;
}

type Channels = Channel[];

interface ChannelSelectorProps {
    onChannelSelect: (channelId: number) => void;
}

const ChannelSelector: React.FC<ChannelSelectorProps> = ({ onChannelSelect }) => {
    const [channels, setChannels] = useState<Channels>([]);
    const [selectedChannel, setSelectedChannel] = useState<number>(0);
    useEffect(() => {
        fetch("http://localhost:8000/channels")
            .then(response => response.json())
            .then(data => setChannels(data))
            .catch(error => console.error("Error fetching channels:", error));
    }, []);

    const handleChannelChange = useCallback((event: SelectChangeEvent<number>) => {
        const channelId = event.target.value as number;
        setSelectedChannel(channelId);
        onChannelSelect(channelId);
    }, [onChannelSelect]);

    return (
        <Box
            display="flex"
            justifyContent="center"
            alignItems="center"
            padding={3}
            width="100%"
            height="100%"
        >
            <FormControl variant="outlined" sx={{ width: '90%', maxWidth: '400px' }}>
                <InputLabel id="channel-selector-label">Channel</InputLabel>
                <Select
                    labelId="channel-selector-label"
                    value={selectedChannel}
                    onChange={handleChannelChange}
                    label="Channel"
                >
                    {channels.map((channel: Channel) => (
                        <MenuItem key={channel.id} value={channel.id}>
                            {channel.name}
                        </MenuItem>
                    ))}
                </Select>
            </FormControl>
        </Box>
    );
}

export default ChannelSelector;
