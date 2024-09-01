import continuebee from '../../src/client/javascript/continuebee.js';
import { should } from 'chai';
should();

console.log(continuebee);

const savedUser = {};
let keys = {};
const hash = 'firstHash';
const secondHash = 'secondHash';

it('should register a user', async () => {
  const uuid = await continuebee.createUser(hash, (k) => { keys = k; }, () => { return keys; });
  savedUser.uuid = uuid;
  savedUser.uuid.length.should.equal(36);
});

it('should check hash', async () => {
  const res = await continuebee.checkHash(savedUser.uuid, hash);
  res.should.equal(true);
});

it('should save hash', async () => {
  const res = await continuebee.updateHash(savedUser.uuid, hash, secondHash);
  res.should.equal(true);
});

it('should delete a user', async () => {
  const res = await continuebee.deleteUser(savedUser.uuid, secondHash);
  res.should.equal(true);
});
