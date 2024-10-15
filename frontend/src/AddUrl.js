import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { addUrl, getUrls } from './api';

const AddUrl = () => {
    const { key } = useParams();  // 从路由参数中获取 key
    const [newUrl, setNewUrl] = useState('');
    const [urls, setUrls] = useState([]);

    useEffect(() => {
        const fetchUrls = async () => {
            try {
                const response = await getUrls(key);
                setUrls(response.data.data);
            } catch (error) {
                console.error('Failed to fetch URLs', error);
            }
        };
        fetchUrls();
    }, [key]);

    const handleAddUrl = async () => {
        try {
            await addUrl(key, newUrl);
            setNewUrl('');
            const response = await getUrls(key);
            setUrls(response.data.data);
        } catch (error) {
            console.error('Failed to add URL', error);
        }
    };

    return (
        <div>
            <h2>Manage URLs for Key: {key}</h2>
            <ul>
                {urls.map((url, index) => (
                    <li key={index}>{url}</li>
                ))}
            </ul>
            <input
                type="text"
                value={newUrl}
                onChange={(e) => setNewUrl(e.target.value)}
                placeholder="Enter new URL"
            />
            <button onClick={handleAddUrl}>Add URL</button>
        </div>
    );
};

export default AddUrl;
