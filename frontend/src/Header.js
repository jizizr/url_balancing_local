import React from 'react';

const Header = ({ user }) => {
    return (
        <header style={{ display: 'flex', justifyContent: 'flex-end', padding: '10px' }}>
            {user ? (
                <div>
                    <img
                        src={user.avatar_url}
                        alt={user.name}
                        style={{ width: '50px', borderRadius: '50%', marginRight: '10px' }}
                    />
                    <span>{user.name}</span>
                </div>
            ) : null}
        </header>
    );
};

export default Header;
