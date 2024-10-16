import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Dashboard from './Dashboard';
import { AddUrl } from './Url';
import AuthCallback from './AuthCallback';
import { checkLogin } from './api';
import Header from './Header';
import ProtectedRoute from './ProtectedRoute';
import Login from './Login';

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false); // 是否登录
  const [user, setUser] = useState(null); // 用户信息
  const [loading, setLoading] = useState(true); // 添加一个加载状态
  useEffect(() => {
    const checkUserLogin = async () => {
      try {
        setLoading(true); // 开始加载
        const response = await checkLogin();
        if (response.data.code === 0) {
          setIsAuthenticated(true);
          setUser(response.data.data);
          localStorage.setItem('isAuthenticated', true);
          sessionStorage.setItem('user', JSON.stringify(response.data.data));
        } else {
          setIsAuthenticated(false);
          localStorage.removeItem('isAuthenticated');
          sessionStorage.removeItem('user');
        }
      } catch (error) {
        console.error('Error checking login status', error);
        setIsAuthenticated(false);
        localStorage.removeItem('isAuthenticated');
        sessionStorage.removeItem('user');
      } finally {
        setLoading(false); // 无论是否成功，都停止加载
      }
    };

    checkUserLogin();
  }, []);

  return (
    <Router>
      <div>
        {/* 全局导航栏，显示用户头像或登录按钮 */}
        {loading ? (
          <div>Loading...</div> // 或者显示一个加载指示器
        ) : (
          <>
            <Header user={user} />
            <Routes>
              <Route path="/" element={isAuthenticated && user ? <Dashboard user={user} /> : <Login />} />
              <Route
                path="/auth/callback"
                element={<AuthCallback setIsAuthenticated={setIsAuthenticated} setUser={setUser} />}
              />
              <Route
                path="/*"
                element={
                  <ProtectedRoute
                    isAuthenticated={isAuthenticated}
                    setIsAuthenticated={setIsAuthenticated}
                    user={user}
                    setUser={setUser}
                  >
                    <Routes>
                      {/* 需要检查登录 */}
                      <Route path="add-url/:key" element={<AddUrl />} />
                    </Routes>
                  </ProtectedRoute>
                }
              />
            </Routes>
          </>
        )}
      </div>
    </Router>
  );
};

export default App;
