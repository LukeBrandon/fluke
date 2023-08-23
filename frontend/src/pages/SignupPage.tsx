import React from 'react';
import SignupForm from '../components/SignupForm';
import { SignupData } from "../types";

const SignupPage: React.FC = () => {
  const handleSignup = (data: SignupData) => {
    console.log(data);
  };

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
      <SignupForm onSubmit={handleSignup} />
    </div>
  );
};

export default SignupPage;
