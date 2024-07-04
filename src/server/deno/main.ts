import { Application, Router } from "https://deno.land/x/oak/mod.ts";
import { oakCors } from "https://deno.land/x/cors/mod.ts";
import sessionless from "npm:sessionless-node@^0.10.1";
import { getUser, saveUser, deleteUser } from "./src/persistence/user.ts";

const register = async (context): object => {
  const request = context.request;
  const payload = await request.body.json();
  const signature = payload.signature;

  const message = payload.timestamp + payload.pubKey + payload.hash;

  if(!signature || !sessionless.verifySignature(signature, message, payload.pubKey)) {
    context.response.body = {
      status: 403, 
      error: 'Auth error'
    };
    return;
  }

  const uuid = sessionless.generateUUID();
  await saveUser(uuid, payload.pubKey, payload.hash);

  context.response.body = { userUUID: uuid };
};

const checkHash = async (context): object => {
  const request = context.request;
  const url = new URL(request.url);
  const params = url.searchParams;
  const pathname = url.pathname;
  
  const uuid = pathname.split('/')[2];
  const message = params.get('timestamp') + uuid + params.get('hash');
  const signature = params.get('signature');

  const user = await getUser(uuid);

  if(!signature || !sessionless.verifySignature(signature, message, user.pubKey)) {
    context.response.body = {
      status: 403,
      error: 'Auth error'
    }; 
    return;
  }

  if(user.hash === params.get('hash')) {
    context.response.status = 202;
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

  const message = payload.timestamp + payload.userUUID + payload.hash + payload.newHash

  const uuid = pathname.split('/')[2];
  const user = await getUser(payload.userUUID);

  if(!signature || !sessionless.verifySignature(signature, message, user.pubKey)) {
    context.response.body = {
      status: 403,
      error: 'Auth error'
    };
    return;
  }

  await saveUser(user.uuid, user.pubKey, payload.hash);

  context.response.status = 202;
};

const deleteSavedUser = async (context): object => {
  const request = context.request;
  const payload = await request.body.json();
  const signature = payload.signature;

  const message = payload.timestamp + payload.userUUID + payload.hash;
  const user = await getUser(payload.userUUID);

  if(!signature || !sessionless.verifySignature(signature, message, user.pubKey)) {
    context.response.body = {
      status: 403,
      error: 'Auth error'
    };
    return;
  }

  const deleted = await deleteUser(user);
  context.response.status = deleted ? 202 : 400;
};

const router = new Router();
router.post("/user/create", register);
router.get("/user/:uuid", checkHash);
router.put("/user/update-hash", saveHash);
router.delete("/user/delete", deleteSavedUser);

const app = new Application();
app.use(oakCors()); 
app.use(router.routes());

await app.listen({ port: 8080 });
