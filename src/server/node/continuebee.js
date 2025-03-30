import config from './config/local.js';
import express from 'express';
import cors from 'cors';
import { createHash } from 'node:crypto';
import user from './src/user/user.js';
import fount from 'fount-js';
import bdo from 'bdo-js';
import sessionless from 'sessionless-node';
import db from './src/persistence/db.js';

const app = express();
app.use(cors());
app.use(express.json());

const SUBDOMAIN = process.env.SUBDOMAIN || 'dev';
fount.baseURL = process.env.LOCALHOST ? 'http://localhost:3006/' : `${SUBDOMAIN}.fount.allyabase.com/`;
bdo.baseURL = process.env.LOCALHOST ? 'http://localhost:3003/' : `${SUBDOMAIN}.bdo.allyabase.com/`;

const bdoHashInput = `${SUBDOMAIN}continuebee`;

const bdoHash = createHash('sha256').update(bdoHashInput).digest('hex');

const repeat = (func) => {
  setTimeout(func, 2000);
};

const bootstrap = async () => {
  try {
    const fountUser = await fount.createUser(db.saveKeys, db.getKeys);
    const bdoUUID = await bdo.createUser(bdoHash, {}, () => {}, db.getKeys);
    const spellbooks = await bdo.getSpellbooks(bdoUUID, bdoHash);
    const continuebee = {
      uuid: 'continuebee',
      fountUUID: fountUser.uuid,
      fountPubKey: fountUser.pubKey,
      bdoUUID,
      spellbooks
    };

    if(!continuebee.fountUUID || !continuebee.bdoUUID || !spellbooks) {
console.log(continuebee.fountUUID, continuebee.bdoUUID, spellbooks);
      throw new Error('bootstrap failed');
    }

    await db.putUser(continuebee);
  } catch(err) {
console.warn(err);
    repeat(bootstrap);
  }
};

repeat(bootstrap);

app.use((req, res, next) => {
  const requestTime = +req.query.timestamp || +req.body.timestamp;
  const now = new Date().getTime();
  if(Math.abs(now - requestTime) > config.allowedTimeDifference) {
    return res.send({error: 'no time like the present'});
  }
  next();
});

app.post('/user/create', async (req, res) => {
  try {
    const body = req.body;
console.log(body);
    const pubKey = body.pubKey;
    const hash = body.hash;
    const message = body.timestamp + pubKey + hash;

    const signature = req.body.signature;
   
    if(!signature || !sessionless.verifySignature(signature, message, pubKey)) {
console.log("auth error");
      res.status(403);
      return res.send({error: 'auth error'});
    }

    const userCheck = await db.getUserByPublicKey(pubKey);
    if(userCheck && userCheck.hash === hash) {
      return res.send(userCheck);
    }

console.log('putting user');
    const userToPut = {
      pubKey,
      hash
    };

    const foundUser = await user.putUser(userToPut);
console.log('sending user', foundUser);
    res.send(foundUser);
  } catch(err) {
    res.status(404);
    res.send({ error: 'Not Found' });
  }
});

app.get('/user/:uuid', async (req, res) => {
  try {
    const uuid = req.params.uuid;
    const hash = req.query.hash;
    const timestamp = req.query.timestamp;
    const signature = req.query.signature;
    const message = timestamp + uuid + hash;
   
    const foundUser = await user.getUser(uuid);

console.log('foundUser', foundUser);

    if(!signature || !sessionless.verifySignature(signature, message, foundUser.pubKey)) {
      res.status(403);
      return res.send({error: 'auth error'});
    }

    if(foundUser.hash === hash) {
      res.status(200);
      return res.send();
    }

    res.status(406);
    res.send({error: 'Not acceptable'});
  } catch(err) {
    res.status(404);
    res.send({ error: 'Not Found' });
  }
});

app.put('/user/update-hash', async (req, res) => {
  try {
    const body = req.body;
    const uuid = body.userUUID;
    const timestamp = body.timestamp;
    const hash = body.hash;
    const newHash = body.newHash;
    const signature = body.signature;
    const message = timestamp + uuid + hash + newHash;

    const foundUser = await user.getUser(uuid);

    if(!signature || !sessionless.verifySignature(signature, message, foundUser.pubKey)) {
      res.status(403);
      return res.send({error: 'auth error'});
    }

    const updatedUser = await user.updateHash(foundUser, hash, newHash);

    res.status(202);
    res.send(updatedUser);
  } catch(err) {
    res.status(404);
    res.send({ error: 'Not Found' });
  }
});

app.delete('/user/delete', async (req, res) => {
  try {
    const body = req.body;

console.log(body);
    const uuid = body.userUUID;
    const timestamp = body.timestamp;
    const hash = body.hash;
    const signature = body.signature;
    const message = timestamp + uuid + hash;
console.log("vars consted");

    const foundUser = await user.getUser(uuid);

    if(!signature || !sessionless.verifySignature(signature, message, foundUser.pubKey)) {
      res.status(403);
      return res.send({error: 'auth error'});
    }
console.log('about to delete');
    const success = await user.deleteUser(uuid);
console.log('success: ', success);
    res.send({ success });
  } catch(err) {
console.warn(err);
    res.status(404);
    res.send({ error: 'Not Found' });
  }
});

app.listen(2999);

console.log('continue bee time!');
