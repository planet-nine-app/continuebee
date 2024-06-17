import { should } from 'chai';
should();
import sessionless from 'sessionless-node';
import superAgent from 'superagent';

//const baseURL = 'http://127.0.0.1:3000/';
const baseURL = 'https://thirsty-gnu-80.deno.dev/';

const get = async function(path) {
  //console.info("Getting " + path);
  return await superAgent.get(path).set('Content-Type', 'application/json');
};

const put = async function(path, body) {
  //console.info("Posting " + path);
  return await superAgent.put(path).send(body).set('Content-Type', 'application/json');
};

const post = async function(path, body) {
  //console.info("Putting " + path);
  return await superAgent.post(path).send(body).set('Content-Type', 'application/json');
};

let savedUser = {};
let keys = {};

it('should register a user', async () => {
  keys = await sessionless.generateKeys(() => { return keys; }, () => {return keys;});
console.log(keys);
  keys = {
    privateKey: 'd6bfebeafa60e27114a40059a4fe82b3e7a1ddb3806cd5102691c3985d7fa591',
    pubKey: '03f60b3bf11552f5a0c7d6b52fcc415973d30b52ab1d74845f1b34ae8568a47b5f'
  };
  const payload = {
    timestamp: new Date().getTime() + '',
    pubKey: keys.pubKey
  };

  payload.signature = await sessionless.sign(JSON.stringify(payload));

  payload.hash = 'first-hash';

  const res = await put(`${baseURL}user/create`, payload);
  savedUser = res.body;
console.log(savedUser);
  res.body.uuid.length.should.equal(36);
});

it('should check hash', async () => {
  const message = {
    timestamp: new Date().getTime() + '',
    hash: 'first-hash'
  };

  const signature = await sessionless.sign(JSON.stringify(message));

  const res = await get(`${baseURL}user/${savedUser.uuid}?timestamp=${message.timestamp}&hash=${message.hash}&signature=${signature}`);
//console.log(res);
  res.body.userUUID.length.should.equal(36);
});

it('should save hash', async () => {
  const message = {
    timestamp: new Date().getTime() + '',
    oldHash: 'first-hash',
    hash: 'second-hash'
  };

  message.signature = await sessionless.sign(JSON.stringify(message));

  const res = await post(`${baseURL}user/${savedUser.uuid}/save-hash`, message);
  res.body.userUUID.length.should.equal(36);
});
