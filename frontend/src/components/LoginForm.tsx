import * as React from "react";
import { Box, Button, TextField, Typography } from "@mui/material";
import { LoginData } from "../types";
import { useSignIn } from "react-auth-kit";
import useFetch from '../composables/useFetch';
// import { useNavigate } from "react-router-dom";
import * as ROUTES from "../routes";

interface FormProps {
  onSubmit: (data: LoginData) => void;
}

const SignupForm: React.FC<FormProps> = ({ onSubmit: parentOnSubmit }) => {
  const [formData, setFormData] = React.useState<LoginData>({
    email: "",
    password: "",
  });

  const { data: authData } = useFetch<{
    token: string,
    expiresIn: number,
    tokenType: string,
    authState: object,
    refreshToken?: string,
    refreshTokenExpireIn?: number
  }>('http://localhost:8000/users/login');

  const signIn = useSignIn();

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    parentOnSubmit(formData);

    if (authData) {
      if (
        signIn({
          token: authData.token,
          expiresIn: authData.expiresIn,
          tokenType: authData.tokenType,
          authState: authData.authState,
        })
      ) {
        navigate(ROUTES.MESSAGES);

        } else {
          console.log("invalid credentials")
      }
    }
  };

  return (
    <Box
      component="form"
      onSubmit={handleSubmit}
      sx={{
        display: "flex",
        flexDirection: "column",
        gap: 2,
        width: "100%",
        maxWidth: 400,
        mt: 3,
      }}
    >
      <Typography variant="h5">Log In</Typography>
      <TextField
        label="Email"
        variant="outlined"
        type="email"
        name="email"
        value={formData.email}
        onChange={handleInputChange}
        required
      />
      <TextField
        label="Password"
        variant="outlined"
        type="password"
        name="password"
        value={formData.password}
        onChange={handleInputChange}
        required
      />
      <Button type="submit" variant="contained" color="primary">
        Submit
      </Button>
    </Box>
  );
};

export default SignupForm;

