import React from "react";
import { useState } from "react";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import MessageDisplay from "../components/MessageDisplay";
import ChannelSelector from "../components/ChannelSelector";

export default function ChannelPage() {
  const [channelId, setChannelId] = useState<number | null>(null);
  const handleChannelSelect = (id: number) => {
    setChannelId(id);
  };

  return (
    <React.Fragment>
      <Box sx={{ width: "80%", textAlign: "center", mb: 3 }}>
        <Typography variant="h6">Channel Selector:</Typography>
        <ChannelSelector onChannelSelect={handleChannelSelect} />
      </Box>

      {channelId && (
        <Box sx={{ width: "100%", mt: 3 }}>
          <Typography variant="h6" align="center">
            Messages from Channel {channelId}:
          </Typography>
          <MessageDisplay
            url={`http://localhost:8000/channels/${channelId}/messages`}
          />
        </Box>
      )}
    </React.Fragment>
  );
}
