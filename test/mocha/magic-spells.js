import { should } from 'chai';
should();
import sessionless from 'sessionless-node';
import fount from 'fount-js';
import { createHash } from 'crypto';

const baseURL = process.env.SUB_DOMAIN ? `https://${process.env.SUB_DOMAIN}.fount.allyabase.com/` : 'http://127.0.0.1:3006/';
fount.baseURL = baseURL;

// Helper to create password hash
const createPasswordHash = (email, password) => {
  const hash = createHash('sha256');
  hash.update(`${email}:${password}`);
  return hash.digest('hex');
};

let savedUser = {};
let keys = {};
let fountUser = {};
let testHash = '';
let newTestHash = '';

describe('Continuebee MAGIC Spell Tests', () => {

  before(async () => {
    // Generate keys for testing
    keys = await sessionless.generateKeys(() => { return keys; }, () => { return keys; });

    // Create fount user for spell casting
    fountUser = await fount.createUser(() => keys, () => keys);
    console.log('Created fount user:', fountUser.uuid);

    // Create test password hashes
    testHash = createPasswordHash('magic-test@continuebee.com', 'PASSWORD123');
    newTestHash = createPasswordHash('magic-test@continuebee.com', 'NEWPASSWORD456');
  });

  it('should create user via continuebeeUserCreate spell', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserCreate',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 0,
      components: {
        pubKey: keys.pubKey,
        hash: testHash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserCreate', spell);

    console.log('continuebeeUserCreate result:', result);

    result.should.have.property('success', true);
    result.should.have.property('user');
    result.user.should.have.property('uuid');
    result.user.uuid.length.should.equal(36);
    result.user.should.have.property('pubKey', keys.pubKey);
    result.user.should.have.property('hash', testHash);

    savedUser = result.user;
  });

  it('should return existing user when creating with same pubKey and hash', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserCreate',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 1,
      components: {
        pubKey: keys.pubKey,
        hash: testHash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserCreate', spell);

    console.log('continuebeeUserCreate (existing) result:', result);

    result.should.have.property('success', true);
    result.should.have.property('user');
    result.user.uuid.should.equal(savedUser.uuid);
  });

  it('should update user hash via continuebeeUserUpdateHash spell', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserUpdateHash',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 2,
      components: {
        userUUID: savedUser.uuid,
        hash: testHash,
        newHash: newTestHash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserUpdateHash', spell);

    console.log('continuebeeUserUpdateHash result:', result);

    result.should.have.property('success', true);
    result.should.have.property('user');
    result.user.should.have.property('hash', newTestHash);
  });

  it('should delete user via continuebeeUserDelete spell', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserDelete',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 3,
      components: {
        userUUID: savedUser.uuid,
        hash: newTestHash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserDelete', spell);

    console.log('continuebeeUserDelete result:', result);

    result.should.have.property('success', true);
  });

  it('should fail to create user with missing pubKey', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserCreate',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 4,
      components: {
        // Missing pubKey
        hash: testHash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserCreate', spell);

    console.log('continuebeeUserCreate (missing pubKey) result:', result);

    result.should.have.property('success', false);
    result.should.have.property('error');
  });

  it('should fail to create user with missing hash', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserCreate',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 5,
      components: {
        pubKey: keys.pubKey
        // Missing hash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserCreate', spell);

    console.log('continuebeeUserCreate (missing hash) result:', result);

    result.should.have.property('success', false);
    result.should.have.property('error');
  });

  it('should fail to update hash with wrong current hash', async () => {
    // Create a new user for this test
    const timestamp1 = Date.now().toString();
    const testHash2 = createPasswordHash('test2@continuebee.com', 'PASSWORD');

    const createSpell = {
      spell: 'continuebeeUserCreate',
      casterUUID: fountUser.uuid,
      timestamp: timestamp1,
      totalCost: 50,
      mp: true,
      ordinal: 6,
      components: {
        pubKey: keys.pubKey,
        hash: testHash2
      }
    };

    const createMessage = timestamp1 + createSpell.spell + createSpell.casterUUID + createSpell.totalCost + createSpell.mp + createSpell.ordinal;
    createSpell.casterSignature = await sessionless.sign(createMessage);

    const createResult = await fount.castSpell('continuebeeUserCreate', createSpell);
    const testUser = createResult.user;

    // Try to update with wrong hash
    const timestamp2 = Date.now().toString();
    const wrongHash = createPasswordHash('wrong@email.com', 'WRONG');

    const updateSpell = {
      spell: 'continuebeeUserUpdateHash',
      casterUUID: fountUser.uuid,
      timestamp: timestamp2,
      totalCost: 50,
      mp: true,
      ordinal: 7,
      components: {
        userUUID: testUser.uuid,
        hash: wrongHash,
        newHash: newTestHash
      }
    };

    const updateMessage = timestamp2 + updateSpell.spell + updateSpell.casterUUID + updateSpell.totalCost + updateSpell.mp + updateSpell.ordinal;
    updateSpell.casterSignature = await sessionless.sign(updateMessage);

    const result = await fount.castSpell('continuebeeUserUpdateHash', updateSpell);

    console.log('continuebeeUserUpdateHash (wrong hash) result:', result);

    result.should.have.property('success', false);
    result.should.have.property('error');
  });

  it('should fail to delete user with missing fields', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserDelete',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 8,
      components: {
        // Missing userUUID and hash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserDelete', spell);

    console.log('continuebeeUserDelete (missing fields) result:', result);

    result.should.have.property('success', false);
    result.should.have.property('error');
  });

  it('should fail to update hash with missing newHash', async () => {
    const timestamp = Date.now().toString();

    const spell = {
      spell: 'continuebeeUserUpdateHash',
      casterUUID: fountUser.uuid,
      timestamp,
      totalCost: 50,
      mp: true,
      ordinal: 9,
      components: {
        userUUID: 'some-uuid',
        hash: testHash
        // Missing newHash
      }
    };

    // Sign the spell
    const message = timestamp + spell.spell + spell.casterUUID + spell.totalCost + spell.mp + spell.ordinal;
    spell.casterSignature = await sessionless.sign(message);

    // Cast the spell
    const result = await fount.castSpell('continuebeeUserUpdateHash', spell);

    console.log('continuebeeUserUpdateHash (missing newHash) result:', result);

    result.should.have.property('success', false);
    result.should.have.property('error');
  });

});
