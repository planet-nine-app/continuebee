import { createClient } from './client.js';
import sessionless from 'sessionless-node';
  
const client = await createClient()
  .on('error', err => console.log('Redis Client Error', err))
  .connect();
    
const db = {
  getUser: async (uuid) => {
    const user = await client.get(`user:${uuid}`);
    const parsedUser = JSON.parse(user);
    return parsedUser; 
  },

  getUserByPublicKey: async (pubKey) => {
    const uuid = await client.get(`pub_key:${pubKey}`);
    const user = await db.getUser(uuid);
    return user;
  },

  putUser: async (user) => {
    const uuid = sessionless.generateUUID();
    user.userUUID = uuid;
    await client.set(`user:${uuid}`, JSON.stringify(user));
    await client.set(`pub_key:${user.pubKey}`, uuid);
    const userToReturn = JSON.parse(JSON.stringify(user));
    return userToReturn;
  },

  updateHash: async (existingUser, oldHash, newHash) => {
    const user = await db.getUser(existingUser.userUUID);
    user.hash = newHash;
    const updatedUser = await db.putUser(user);
    return updatedUser;
  },

  deleteUser: async (uuid) => {
    const resp = await client.del(`user:${uuid}`);

    return true;
  },

  saveKeys: async (keys) => {
    await client.set(`keys`, JSON.stringify(keys));
  },

  getKeys: async () => {
    const keyString = await client.get('keys');
    return JSON.parse(keyString);
  }

};

export default db;
