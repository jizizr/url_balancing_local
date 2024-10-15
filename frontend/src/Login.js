import React from 'react';

const Login = ({ handleLogin }) => {
  return (
    <div style={{ textAlign: 'center', marginTop: '50px' }}>
      <h2>Please log in to continue.</h2>
      <button 
        onClick={handleLogin}
        style={{
          padding: '10px 20px',
          fontSize: '18px',
          backgroundColor: '#007bff',
          color: '#fff',
          border: 'none',
          borderRadius: '5px',
          cursor: 'pointer',
          marginTop: '20px'
        }}
      >
        Login
      </button>
    </div>
  );
};

export default Login;
