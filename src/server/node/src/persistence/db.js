import config from '../../config/local.js';
import { createClient } from 'redis';
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

  putUser: async (user) => {
    const uuid = sessionless.generateUUID();
    user.uuid = uuid;
    await client.set(`user:${user.uuid}`, JSON.stringify(user));
    const userToReturn = JSON.parse(JSON.stringify(user));
    return userToReturn;
  },

  updateHash: async (existingUser, oldHash, newHash) => {
    const user = await db.getUser(existingUser.uuid);
    user.hash = newHash;
    const updatedUser = await db.putUser(user);
    return updatedUser;
  },

  deleteUser: async (uuid) => {
    const resp = await client.sendCommand(['DEL', `user:${uuid}`]);

    return true;
  }

};

export default db;
