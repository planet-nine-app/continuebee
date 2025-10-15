import sessionless from 'sessionless-node';
import user from '../user/user.js';
import db from '../persistence/db.js';

sessionless.getKeys = async () => {
  return await db.getKeys();
};

const fountURL = 'http://localhost:3006/';

const MAGIC = {
  joinup: async (spell) => {
    const gateway = await MAGIC.gatewayForSpell(spell.spellName);
    spell.gateways.push(spell);

    const spellbook = await db.get('spellbook');
    const nextIndex = spellbook.destinations.indexOf(spellbook.destinations.find(($) => $.stopName === 'continuebee'));
    const nextDestination = spellbook.destinations[nextIndex].stopURL + spell.spellName;

    const res = await MAGIC.forwardSpell(spell, nextDestination);
    const body = await res.json();
 
    if(!body.success) {
      return body;
    }

    const foundUser = await user.putUser(spell.user);
    if(!body.uuids) {
      body.uuids = [];
    }
    body.uuids.push({
      service: 'continuebee',
      uuid: foundUser.uuid
    });

    return body;
  },

  linkup: async (spell) => {
    const foundUser = await user.getUser(spell.casterUUID);

    if(coordinatingKeys.filter(keys => keys).length !== spell.gateways.length) {
      throw new Error('missing coordinating key');
    }

    const gateway = await MAGIC.gatewayForSpell(spell.spellName);
    gateway.coordinatingKey = {
      serviceURL: 'http://localhost:2999/', // Once hedy is built, this will be dynamic
      uuid: spell.casterUUID,
      pubKey: foundUser.pubKey
    };
    spell.gateways.push(gateway);

    const res = await MAGIC.forwardSpell(spell, fountURL);
    const body = await res.json();
    return body;
  },

  gatewayForSpell: async (spellName) => {
    const continuebee = await db.getUser('continuebee');
    const gateway = {
      timestamp = new Date().getTime() + '',
      uuid: continuebee.fountUUID, 
      minimumCost: 20,
      ordinal: continuebee.ordinal
    };      

    const message = gateway.timestamp + gateway.uuid + gateway.minimumCost + gateway.ordinal;

    gateway.signature = await sessionless.sign(message);

    return gateway;
  },

  forwardSpell: async (spell, destination) => {
    return await fetch(destination, {
      method: 'post',
      body: JSON.stringify(spell),
      headers: {'Content-Type': 'application/json'}
    });
  },

  // 🪄 MAGIC-ROUTED ENDPOINTS (No auth needed - resolver authorizes)

  continuebeeUserCreate: async (spell) => {
    try {
      const { pubKey, hash } = spell.components;

      if (!pubKey || !hash) {
        return {
          success: false,
          error: 'Missing required fields: pubKey, hash'
        };
      }

      // Check if user already exists
      try {
        const userCheck = await db.getUserByPublicKey(pubKey);
        if (userCheck && userCheck.hash === hash) {
          return {
            success: true,
            user: userCheck
          };
        }
      } catch (err) {
        // User doesn't exist, continue with creation
      }

      const userToPut = {
        pubKey,
        hash
      };

      const foundUser = await user.putUser(userToPut);

      return {
        success: true,
        user: foundUser
      };
    } catch (err) {
      console.error('continuebeeUserCreate error:', err);
      return {
        success: false,
        error: err.message
      };
    }
  },

  continuebeeUserUpdateHash: async (spell) => {
    try {
      const { userUUID, hash, newHash } = spell.components;

      if (!userUUID || !hash || !newHash) {
        return {
          success: false,
          error: 'Missing required fields: userUUID, hash, newHash'
        };
      }

      const foundUser = await user.getUser(userUUID);

      if (!foundUser) {
        return {
          success: false,
          error: 'User not found'
        };
      }

      // Verify current hash matches
      if (foundUser.hash !== hash) {
        return {
          success: false,
          error: 'Current hash does not match'
        };
      }

      const updatedUser = await user.updateHash(foundUser, hash, newHash);

      return {
        success: true,
        user: updatedUser
      };
    } catch (err) {
      console.error('continuebeeUserUpdateHash error:', err);
      return {
        success: false,
        error: err.message
      };
    }
  },

  continuebeeUserDelete: async (spell) => {
    try {
      const { userUUID, hash } = spell.components;

      if (!userUUID || !hash) {
        return {
          success: false,
          error: 'Missing required fields: userUUID, hash'
        };
      }

      const foundUser = await user.getUser(userUUID);

      if (!foundUser) {
        return {
          success: false,
          error: 'User not found'
        };
      }

      // Verify hash matches
      if (foundUser.hash !== hash) {
        return {
          success: false,
          error: 'Hash does not match'
        };
      }

      const success = await user.deleteUser(userUUID);

      return {
        success: success
      };
    } catch (err) {
      console.error('continuebeeUserDelete error:', err);
      return {
        success: false,
        error: err.message
      };
    }
  }
};

export default MAGIC;
