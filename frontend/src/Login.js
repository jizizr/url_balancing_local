import React from 'react';

const handleLogin = async () => {
  window.location.href = "api/auth/linuxdo";
};

const Login = () => {
  return (
    <div className="mdui-container" style={{ display: 'flex', justifyContent: 'center', marginTop: '50px' }}>
      <div
        className="mdui-card mdui-shadow-3 mdui-color-theme"
        style={{
          padding: '30px',
          borderRadius: '15px',
          boxShadow: '0 4px 8px rgba(0, 0, 0, 0.2)',
          backgroundColor: '#f5f5f5',
          textAlign: 'center',
          maxWidth: '400px',
        }}
      >
        <h2>请点击下方按钮使用linuxdo登录</h2>
        <mdui-button
          onClick={handleLogin}
          style={{
            padding: '10px 20px',
            fontSize: '18px',
            marginTop: '20px',
          }}
        >
          登录
        </mdui-button>
      </div>
    </div>
  );
};
export default Login;
