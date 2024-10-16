import axios from 'axios';

export const createKey = () => axios.post(`/key`);
export const addUrl = (key, url) => axios.post(`/${key}/url`, { url });
export const deleteUrl = (key, url) => axios.delete(`/${key}/url`, { data: { url } });
export const getUrls = (key) => axios.get(`/${key}/urls`);
export const linuxdoAuthorized = (params) => axios.get(`/auth/authorized`, { params });
export const checkLogin = () => axios.get(`/user`);
export const getKeys = () => axios.get(`/key`);