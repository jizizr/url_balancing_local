import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Dashboard from './Dashboard';
import AddUrl from './AddUrl';
import AuthCallback from './AuthCallback';
import { checkLogin } from './api';
import Header from './Header';
import ProtectedRoute from './ProtectedRoute';
import Login from './Login';

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false); // 是否登录
  const [user, setUser] = useState(null); // 用户信息

  // 初次加载时检查登录状态
  useEffect(() => {
    const checkUserLogin = async () => {
      try {
        const response = await checkLogin();
        if (response.data.code === 0) {
          setIsAuthenticated(true);
          setUser(response.data.data); // 保存用户信息
        } else {
          setIsAuthenticated(false);
        }
      } catch (error) {
        console.error('Error checking login status', error);
        setIsAuthenticated(false);
      }
    };

    checkUserLogin();
  }, []);

  const handleLogin = async () => {
    window.location.href = "http://127.0.0.1:8080/auth/linuxdo";
  };

  return (
    <Router>
      <div>
        {/* 全局导航栏，显示用户头像或登录按钮 */}
        <Header user={user} handleLogin={handleLogin} />

        {/* 路由配置 */}
        <Routes>
          <Route path="/" element={isAuthenticated ? <Dashboard user={user} /> : <Login handleLogin={handleLogin} />} />
          <Route
            path="/auth/callback"
            element={<AuthCallback setIsAuthenticated={setIsAuthenticated} setUser={setUser} />}
          />
          <Route
            path="/*"
            element={
              <ProtectedRoute isAuthenticated={isAuthenticated}>
                <Routes>
                  {/* 需要检查登录 */}
                  <Route path="add-url/:key" element={<AddUrl />} />
                </Routes>
              </ProtectedRoute>
            }
          />
        </Routes>
      </div>
    </Router>
  );
};

export default App;
