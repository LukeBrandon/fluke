import Box from '@mui/material/Box';

function MainContainer({ children }) {
  return (
    <Box
      sx={{
        height: '100vh',
        width: '100vw',
        overflow: 'hidden'
      }}
    >
      {children}
    </Box>
  );
}

export default MainContainer;
