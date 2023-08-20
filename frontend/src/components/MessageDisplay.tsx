import React, { useState, useEffect } from 'react';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import useFetch from '../composables/useFetch';

interface Message {
  id: string;
  message: string;
  user_id: string;
}

type Messages = Message[];

interface User {
  id: number;
  first_name: string;
  last_name: string;
  email: string;
  password: string;
  deleted: boolean;
}

const MessagesDisplay: React.FC<{ url: string }> = ({ url }) => {
  const { data: messages, loading, error } = useFetch<Messages>(url);
  const [users, setUsers] = useState<{ [key: string]: User }>({});

  useEffect(() => {
    if (messages) {
      messages.forEach((msg) => {
        if (!users[msg.user_id]) {
          fetch(`http://localhost:8000/users/${msg.user_id}`)
            .then((res) => res.json())
            .then((userData) => {
              setUsers((prevUsers) => ({ ...prevUsers, [msg.user_id]: userData }));
            })
            .catch((error) => console.error("Error fetching user:", error));
        }
      });
    }
  }, [messages, users]);

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
      {messages && messages.map((msg) => (
        <Box
          key={msg.id}
          sx={{
            display: 'flex',
            alignItems: 'center',
            gap: 1,
            wordWrap: 'break-word',
            backgroundColor: '#2C2F33',
            color: 'white',
            padding: 1,
            borderRadius: 1,
            maxWidth: 300,
            boxShadow: 1,
          }}
        >
          <Typography variant="body2" color="gray">{users[msg.user_id]?.first_name}</Typography>
          <Typography>{msg.message}</Typography>
        </Box>
      ))}
    </Box>
  );
}

export default MessagesDisplay;
