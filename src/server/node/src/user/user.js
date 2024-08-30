import db from '../persistence/db.js';

const user = {
  getUser: async (uuid) => {
    const foundUser = await db.getUser(uuid);
    return foundUser;
  }, 

  putUser: async (newUser) => {
    const userToReturn = await db.putUser(newUser);

    return userToReturn;
  },

  updateHash: async (user, oldHash, newHash) => {
    const updatedUser = db.updateHash(user, oldHash, newHash);
    return updatedUser;
  },
  
  deleteUser: async (userToDelete) => {
    return (await db.deleteUser(userToDelete));
  }
};

export default user;
