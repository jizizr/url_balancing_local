import React, { useState, useEffect } from 'react';
import { createKey, getKeys } from './api';
import { useNavigate } from 'react-router-dom';
// import { snackbar } from "mdui/functions/snackbar.js";

const Dashboard = ({ user }) => {
  const [keys, setKeys] = useState([]);
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

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
      setLoading(true);
      const response = await createKey();
      const newKey = response.data.data;
      setKeys([...keys, newKey]);
    } catch (error) {
      console.error('Failed to create key', error);
    } finally {
      setLoading(false);
    }
  };

  const handleManageUrls = (key) => {
    navigate(`/add-url/${key}`);
  };

  const handleCopy = (key) => {
    navigator.clipboard.writeText("https://" + window.location.host + "/api/" + key)
      .then(() => {
        handleCopyClick();
      })
      .catch(err => {
        console.error('复制失败: ', err);
      });
  };

  const handleCopyClick = () => {
    window.mdui.snackbar({
      closeOnOutsideClick: true,
      message: "复制链接成功",
    });
  }

  return (
    <div className="mdui-prose" style={{
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      flexDirection: 'column',
    }}>
      <div>
        <h2>Your Keys</h2>
      </div>
      <div
        style={{
          maxHeight: "70vh",
          overflow: "auto"
        }}
      >
        <mdui-list>
          {keys.map((key, index) => (
            <mdui-list-item key={index} nonclickable>
              <code><u style={{ fontSize: "16px" }}>{key}</u></code>
              <mdui-button
                slot="end-icon"
                onClick={() => handleManageUrls(key)}
                icon="settings"
                iconPosition="right"
                variant="outlined"
              >管理</mdui-button>
              <mdui-button
                slot="end-icon"
                onClick={() => handleCopy(key)}
                icon="content_copy"
                iconPosition="right"
                variant="outlined"
              >复制链接</mdui-button>
              {/* 
              <mdui-divider></mdui-divider> */}
            </mdui-list-item>

          ))}
        </mdui-list>
      </div>
      <div style={{
        marginTop: '5vh',
      }}>
        {
          loading
            ?
            (<mdui-button
              onClick={() => handleCreateKey()}
              icon="vpn_key"
              loading
            >
              Create New Key
            </mdui-button>)
            :
            (<mdui-button
              onClick={() => handleCreateKey()}
              icon="vpn_key"
            >
              创建新的 Key
            </mdui-button>)
        }
      </div>
    </div >

  );
};

export default Dashboard;
