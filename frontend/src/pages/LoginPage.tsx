import React from 'react';
import LoginForm from '../components/LoginForm';
import { LoginData } from "../types";

const SignupPage: React.FC = () => {
  const handleSignup = (data: LoginData) => {
    console.log(data);
  };

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
      <LoginForm onSubmit={handleSignup} />
    </div>
  );
};

export default SignupPage;
