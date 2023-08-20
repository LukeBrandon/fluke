import React from 'react';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import useFetch from '../composables/useFetch';

interface Message {
  id: string;
  message: string;
}

type Messages = Message[];

interface MessagesDisplayProps {
  url: string;
}

const MessagesDisplay: React.FC<MessagesDisplayProps> = ({ url }) => {
  const { data, loading, error } = useFetch<Messages>(url);

  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'start',
        gap: 2,
      }}
    >
      {loading && <Typography>Loading...</Typography>}
      {error && <Typography color="error">{error.message}</Typography>}
      {data && data.map((msg) => (
        <Box
          key={msg.id}
          sx={{
            wordWrap: 'break-word',
            backgroundColor: '#2C2F33',
            color: 'white',
            padding: 1,
            borderRadius: 1,
            maxWidth: 300,
            boxShadow: 1,
          }}
        >
          <Typography>{msg.message}</Typography>
        </Box>
      ))}
    </Box>
  );
}

export default MessagesDisplay;
