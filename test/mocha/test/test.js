import { should } from 'chai';
should();
import sessionless from 'sessionless-node';
import sa from 'superagent';

const baseURL = 'http://localhost:8080/';

const get = async function(path) {
  //console.info("Getting " + path);
  return await sa.get(path).set('Content-Type', 'application/json');
};

const put = async function(path, body) {
  //console.info("Putting " + path);
  return await sa.put(path).send(body).set('Content-Type', 'application/json');
};

const post = async function(path, body) {
  //console.info("Posting " + path);
  //console.log(body);
  return await sa.post(path).send(body).set('Content-Type', 'application/json');
};

const del = async function(path, body) {
  //console.info("Deleting " + path);
  return await sa.delete(path).send(body).set('Content-Type', 'application/json');
};

let uuid;
let keys;
let lastState = 'lastState';
let currentState = 'currentState';

it('should register a pubKey', async () => {
  keys = await sessionless.generateKeys(() => { return keys; }, () => {return keys;});
  const hash = lastState;
  const signature = await sessionless.sign(hash);

  const payload = {
    timestamp: new Date().getTime() + '',
    signature
  };

  const res = await sa.put(baseURL + `${keys.pubKey}`)
    .send(payload);

//console.log(res.body);

  res.body.userUuid.should.not.be.null;
  uuid = res.body.userUuid;
});

it('should check current hash for user', async () => {
  const hash = lastState;
  const signature = await sessionless.sign(hash);

  const res = await sa.get(baseURL + `${uuid}?hash=${hash}&timestamp=${new Date().getTime() + ''}&signature=${signature}`);
  res.body.uuid.should.not.be.null;
});

it('should save a hash', async () => {
  const hash = lastState;
  const signature = await sessionless.sign(hash);

  const payload = {
    timestamp: new Date().getTime() + '',
    hash,
    signature
  };

  const res = await sa.post(baseURL + `save-hash`)
    .set('content-type', 'application/json')
    .send(user);
//console.log(res);

  res.body.uuid.should.not.be.null;
});

it('should not delete a user', async () => {
  const hash = lastState;
  const signature = await sessionless.sign(hash);

  const res = await sa.delete(baseURL + `${uuid}`);
//console.log(res);
  res.body.should.equal('Bad Request');
});

it('should delete a user', async () => {
  const hash = lastState;
  const signature = await sessionless.sign(hash);

  const res = await sa.delete(baseURL + `${uuid}`);
//console.log(res);
  res.should.be.true;
});
