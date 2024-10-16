import React, { useState, useRef, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { addUrl, getUrls, deleteUrl } from './api';

export const AddUrl = () => {
    const { key } = useParams();
    const [newUrl, setNewUrl] = useState('');
    const [urls, setUrls] = useState([]);
    const TextRef = useRef(null);
    useEffect(() => {
        TextRef.current.addEventListener('change', (e) => { setNewUrl(e.target.value) });
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
        console.log('Adding URL', newUrl);
        try {
            await addUrl(key, newUrl);
            setNewUrl('');
            setUrls(urls.concat(newUrl));
        } catch (error) {
            console.error('Failed to add URL', error);
        }
    };

    return (
        <div className="mdui-prose"
            style={{
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                flexDirection: 'column',
                marginTop: '3vh'
            }}>
            <div>
                <h3 className="mdui-typo">Manage URLs for Key:
                    <code><u>{key}</u></code></h3>
            </div>
            <div
                style={{
                    maxHeight: "60vh",
                    overflow: "auto",
                    marginTop: '1vh',
                }}
            >
                <mdui-list>
                    {urls.map((url, index) => (
                        <mdui-list-item key={index} nonclickable >
                            <code><u style={{ fontSize: "16px" }}>{url}</u></code>
                            <mdui-button
                                slot="end-icon"
                                onClick={() => DeleteUrl(key, url, urls, setUrls)}
                                icon="link_off"
                                variant="outlined"
                            >删除</mdui-button>
                        </mdui-list-item>
                    ))}
                </mdui-list>
            </div>
            <div
                style={{
                    marginTop: '1vh',
                }}>
                <mdui-text-field
                    ref={TextRef}
                    input='url'
                    variant="filled"
                    value={newUrl}
                    autosize
                    autofocus
                    icon="link"
                    label="输入负载均衡的 URL"
                    style={{
                        width: "70vw",
                        maxWidth: '900px',
                        minWidth: '200px',
                    }}
                />
            </div>
            <mdui-button
                onClick={handleAddUrl}
                style={{ marginTop: '5vh' }}>
                Add URL
            </mdui-button>
        </div>
    );
};

const DeleteUrl = async (key, url, urls, setUrls) => {
    try {
        const response = await deleteUrl(key, url);
        if (response.data.code === 0) {
            setUrls(urls.filter((u) => u !== url));
        }
    } catch (error) {
        console.error('Failed to delete URL', error);
    }
}
