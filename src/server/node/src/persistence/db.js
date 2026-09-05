import sessionless from 'sessionless-node';
  
// esbuild's CJS output target (used by Netlify's function bundler) doesn't
// support top-level await, so the client is now a lazily-resolved promise -
// call sites now do `(await client).get(...)` instead of `client.get(...)`.
const client = (async () => {
  const { createClient } = process.env.PERSISTENCE_BACKEND === 'netlify-blobs'
    ? await import('./client.netlify-blobs.js')
    : await import('./client.js');

  return createClient()
    .on('error', err => console.log('Redis Client Error', err))
    .connect();
})();
    
const db = {
  getUser: async (uuid) => {
    const user = await (await client).get(`user:${uuid}`);
    const parsedUser = JSON.parse(user);
    return parsedUser; 
  },

  getUserByPublicKey: async (pubKey) => {
    const uuid = await (await client).get(`pub_key:${pubKey}`);
    const user = await db.getUser(uuid);
    return user;
  },

  putUser: async (user) => {
    const uuid = sessionless.generateUUID();
    user.userUUID = uuid;
    await (await client).set(`user:${uuid}`, JSON.stringify(user));
    await (await client).set(`pub_key:${user.pubKey}`, uuid);
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
    const resp = await (await client).del(`user:${uuid}`);

    return true;
  },

  saveKeys: async (keys) => {
    await (await client).set(`keys`, JSON.stringify(keys));
  },

  getKeys: async () => {
    const keyString = await (await client).get('keys');
    return JSON.parse(keyString);
  }

};

export default db;
