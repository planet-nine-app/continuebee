import { Application, Router } from "https://deno.land/x/oak/mod.ts";
import { oakCors } from "https://deno.land/x/cors/mod.ts";
import sessionless from "npm:sessionless-node@^0.10.1";
import { getUser, saveUser } from "./src/persistence/user.ts";

const register = async (context): object => {
  const request = context.request;
  const payload = await request.body.json();
  const signature = payload.signature;

  const message = JSON.stringify({
    timestamp: payload.timestamp,
    pubKey: payload.pubKey
  });

console.log('verifying: ' + message);

  if(!signature || !sessionless.verifySignature(signature, message, payload.pubKey)) {
    context.response.body = {
      status: 403, 
      error: 'Auth error'
    };
    return;
  }

  const uuid = sessionless.generateUUID();
  await saveUser(uuid, payload.pubKey, payload.hash);

  context.response.body = { uuid };
};

const checkHash = async (context): object => {
  const request = context.request;
  const url = new URL(request.url);
  const params = url.searchParams;
  const pathname = url.pathname;
  const message = JSON.stringify({
    timestamp: params.get('timestamp'),
    hash: params.get('hash')
  });
  const signature = params.get('signature');

  const uuid = pathname.split('/')[2];

  const user = await getUser(uuid);

  if(!signature || !sessionless.verifySignature(signature, message, user.pubKey)) {
    context.response.body = {
      status: 403,
      error: 'Auth error'
    }; 
    return;
  }

  if(user.hash === params.get('hash')) {
    context.response.body = { userUUID: uuid };
    return;
  } 
  context.response.body = {
    status: 406,
    error: 'Not acceptable'
  };
};

const saveHash = async (context): object => {
  const request = context.request;
  const payload = await request.body.json();
  const signature = payload.signature;
  const pathname = new URL(request.url).pathname;

  const message = JSON.stringify({
    timestamp: payload.timestamp,
    oldHash: payload.oldHash,
    hash: payload.hash
  });

  const uuid = pathname.split('/')[2];
  const user = await getUser(uuid);

  if(!signature || !sessionless.verifySignature(signature, message, user.pubKey)) {
    context.response.body = {
      status: 403,
      error: 'Auth error'
    };
    return;
  }

  await saveUser(user.uuid, user.pubKey, payload.hash);

  context.response.body = {
    userUUID: user.uuid
  };
};

const deleteUser = async (request: Request): object => {
  context.response.body = {
    status: 501,
    error: 'Not implemented'
  };
};

const router = new Router();
router.put("/user/create", register);
router.get("/user/:uuid", checkHash);
router.post("/user/:uuid/save-hash", saveHash);
router.delete("/user/:uuid", deleteUser);

const app = new Application();
app.use(oakCors()); 
app.use(router.routes());

await app.listen({ port: 3000 });
