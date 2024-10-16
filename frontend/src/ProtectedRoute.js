import React from 'react';
import { Navigate } from 'react-router-dom';
import { checkLogin } from './api';

const ProtectedRoute = ({ isAuthenticated, setIsAuthenticated, user, setUser, children }) => {
  if (!isAuthenticated) {
    isAuthenticated = localStorage.getItem('isAuthenticated');
  }
  if (!isAuthenticated) {
    // 如果未登录，重定向到首页（登录页面）
    return <Navigate to="/" />;
  }
  if (!user) {
    user = JSON.parse(sessionStorage.getItem('user'));
  }
  if (!user) {
    try {
      const response = checkLogin();
      if (response.data.code === 0) {
        setUser(response.data.data);
        sessionStorage.setItem('user', JSON.stringify(response.data.data));
        return children;
      }
      setIsAuthenticated(false);
      localStorage.removeItem('isAuthenticated');
    } catch (error) {
      setIsAuthenticated(false);
      localStorage.removeItem('isAuthenticated');
      return <Navigate to="/" />;
    }
  }
  return children;
};

export default ProtectedRoute;
