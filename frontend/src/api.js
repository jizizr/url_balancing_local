import axios from 'axios';

const API_BASE_URL = "/api";

export const createKey = () => axios.post(`${API_BASE_URL}/key`);
export const addUrl = (key, url) => axios.post(`${API_BASE_URL}/${key}/url`, { url });
export const deleteUrl = (key, url) => axios.delete(`${API_BASE_URL}/${key}/url`, { data: { url } });
export const getUrls = (key) => axios.get(`${API_BASE_URL}/${key}/urls`);
export const checkLogin = () => axios.get(`${API_BASE_URL}/user`);
export const getKeys = () => axios.get(`${API_BASE_URL}/key`);