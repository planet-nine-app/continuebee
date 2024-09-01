import config from './config/local.js';
import express from 'express';
import cors from 'cors';
import user from './src/user/user.js';
import sessionless from 'sessionless-node';

const app = express();
app.use(cors());
app.use(express.json());

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
console.log('putting user');
    const userToPut = {
      pubKey,
      hash
    };

    const foundUser = await user.putUser(userToPut);
console.log(foundUser);
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
