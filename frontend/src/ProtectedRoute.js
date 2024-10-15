import React from 'react';
import { Navigate } from 'react-router-dom';

const ProtectedRoute = ({ isAuthenticated, children }) => {
  if (!isAuthenticated) {
    // 如果未登录，重定向到首页（登录页面）
    return <Navigate to="/" />;
  }

  // 如果已登录，显示子组件（即具体的页面）
  return children;
};

export default ProtectedRoute;
