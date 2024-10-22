import React from 'react';

const Header = ({ user }) => {
    return (
        <header style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            padding: '10px 20px',
            backgroundColor: '#f1f1f1', // 设置背景颜色 
            boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)' // 添加阴影效果
        }}>
            {/* 左侧菜单按钮 */}
            <div style={{ display: 'flex', alignItems: 'center' }}>
                <span style={{ marginLeft: '10px', fontSize: '18px' }}>Deeplx Balancing</span>
            </div>
            <div style={{ display: 'flex', alignItems: 'center' }}>
                {user ? (
                    <>
                        <span style={{ marginRight: '10px', fontSize: '16px' }}>{user.name}</span>
                    </>
                ) : (
                    <span>Guest</span>
                )}
            </div>
        </header>
    );
};

export default Header;
