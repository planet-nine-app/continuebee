import sessionless from 'sessionless-node';
import fetch from 'node-fetch';

const get = async (url) => {
  return await fetch(url);
};

const post = async (url, payload) => {
  return await fetch(url, {
    method: 'post',
    body: JSON.stringify(payload),
    headers: {'Content-Type': 'application/json'}
  });
};

const put = async (url, payload) => {
  return await fetch(url, {
    method: 'put',
    body: JSON.stringify(payload),
    headers: {'Content-Type': 'application/json'}
  });
};

const _delete = async (url, payload) => {
  return await fetch(url, {
    method: 'delete',
    body: JSON.stringify(payload),
    headers: {'Content-Type': 'application/json'}
  });
};

const continuebee = {
  baseURL: 'https://dev.continuebee.allyabase.com/',

  createUser: async (hash, saveKeys, getKeys) => {
    const keys = await sessionless.generateKeys(saveKeys, getKeys);

    const payload = {
      timestamp: new Date().getTime() + '',
      pubKey: keys.pubKey,
      hash
    };

    payload.signature = await sessionless.sign(payload.timestamp + payload.pubKey + payload.hash);

    const res = await post(`${continuebee.baseURL}user/create`, payload);
    const user = await res.json();
    const uuid = user.userUUID;

    return uuid;
  },

  updateHash: async (uuid, hash, newHash) => {
    const timestamp = new Date().getTime() + '';

    const signature = await sessionless.sign(timestamp + uuid + hash + newHash);
    const payload = {timestamp, userUUID: uuid, hash, newHash, signature};


    const res = await put(`${continuebee.baseURL}user/update-hash`, payload);
    return res.status === 202;
  },

  checkHash: async (uuid, hash) => {
    const timestamp = new Date().getTime() + '';

    const signature = await sessionless.sign(timestamp + uuid + hash);

    const res = await get(`${continuebee.baseURL}user/${uuid}?timestamp=${timestamp}&hash=${hash}&signature=${signature}`);
    return res.status === 200;
  },

  deleteUser: async (uuid, hash) => {
    const timestamp = new Date().getTime() + '';

    const signature = await sessionless.sign(timestamp + uuid + hash);
    const payload = {timestamp, userUUID: uuid, hash, signature};


    const res = await _delete(`${continuebee.baseURL}user/delete`, payload);
console.log(res.status);
console.log(res.status === 200);
    return res.status === 200;
  }
};

export default continuebee;
