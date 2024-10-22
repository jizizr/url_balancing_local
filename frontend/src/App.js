import React, { useEffect, useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Dashboard from './Dashboard';
import { AddUrl } from './Url';
import { checkLogin } from './api';
import Header from './Header';

const App = () => {
  const [user, setUser] = useState(null); // 用户信息
  const [loading, setLoading] = useState(true); // 添加一个加载状态
  useEffect(() => {
    const checkUserLogin = async () => {
      try {
        setLoading(true); // 开始加载
        const response = await checkLogin();
        if (response.data.code === 0) {
          setUser(response.data.data);
        }
      } catch (error) {
        console.error('Error checking login status', error);
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
              <Route path="/" element={<Dashboard user={user} />} />
              <Route path="add-url/:key" element={<AddUrl />} />
            </Routes>
          </>
        )}
      </div>
    </Router>
  );
};

export default App;
