import React, { useState, useEffect } from 'react';
import { createKey, getKeys } from './api';
import { useNavigate } from 'react-router-dom';

const Dashboard = ({ user }) => {
  const [keys, setKeys] = useState([]);
  const navigate = useNavigate();

  useEffect(() => {
    const fetchKeys = async () => {
      try {
        const response = await getKeys();
        setKeys(response.data.data);
      } catch (error) {
        console.error('Failed to fetch keys', error);
      }
    };
    fetchKeys();
  }, []);

  const handleCreateKey = async () => {
    try {
      const response = await createKey();  // 创建新 key
      const newKey = response.data.key;
      setKeys([...keys, newKey]);  // 更新 keys 列表
    } catch (error) {
      console.error('Failed to create key', error);
    }
  };

  const handleManageUrls = (key) => {
    navigate(`/add-url/${key}`);  // 跳转到URL管理界面
  };

  return (
    <div>
      <h2>Welcome, {user.name}</h2>
      <img src={user.avatar_url} alt="Avatar" width={50} />

      <h3>Your Keys</h3>
      <ul>
        {keys.map((key, index) => (
          <li key={index}>
            {key}
            <button onClick={() => handleManageUrls(key)} style={{ marginLeft: '10px' }}>
              Manage URLs
            </button>
          </li>
        ))}
      </ul>

      <button onClick={handleCreateKey} style={{ marginTop: '20px' }}>Create New Key</button>
    </div>
  );
};

export default Dashboard;
