import * as React from "react";
import { Box, Button, TextField, Typography } from "@mui/material";
import { SignupData } from "../types";

interface FormProps {
    onSubmit?: (data: SignupData) => void;
}

const SignupForm: React.FC<FormProps> = ({ onSubmit: parentOnSubmit }) => {
    const [formData, setFormData] = React.useState<SignupData>({
        firstName: "",
        lastName: "",
        email: "",
        password: "",
        confirmPassword: "",
    });

    const [, setError] = React.useState<string | null>(null);

    const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = event.target;
        setFormData((prevData) => ({
            ...prevData,
            [name]: value,
        }));
    };

    const handleSubmit = async (event: React.FormEvent) => {
        event.preventDefault();
        if (parentOnSubmit) {
            parentOnSubmit(formData);
        }
        try {
            const response = await fetch('http://localhost:8000/users', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    first_name: formData.firstName,
                    last_name: formData.lastName,
                    email: formData.email,
                    password: formData.password
                })
            });

            if (response.ok) {
                const result = await response.json();
                if (result.email) {
                    window.location.href = '/login'; // Redirect to the login page
                } else {
                    setError('Unexpected server response');
                }
            } else {
                const result = await response.json();
                setError(result.error || 'Failed to create user');
            }
        } catch (err) {
            setError('An error occurred. Please try again.');
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
            <Typography variant="h5">Sign Up</Typography>
            <TextField
                label="First Name"
                variant="outlined"
                name="firstName"
                value={formData.firstName}
                onChange={handleInputChange}
                required
            />
            <TextField
                label="Last Name"
                variant="outlined"
                name="lastName"
                value={formData.lastName}
                onChange={handleInputChange}
                required
            />
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
            <TextField
                label="Confirm Password"
                variant="outlined"
                type="password"
                name="confirmPassword"
                value={formData.confirmPassword}
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
