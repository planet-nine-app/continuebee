import sessionless from "npm:sessionless-node@^0.10.1";
import { getUser, saveUser } from "./src/persistence/user.ts";

const register = async (request: Request): object => {
  const payload = await request.json();
  const signature = payload.signature;

  const message = JSON.stringify({
    timestamp: payload.timestamp,
    pubKey: payload.pubKey
  });

  if(!signature || !sessionless.verifySignature(signature, message, payload.pubKey)) {
    return {
      status: 403, 
      error: 'Auth error'
    };
  }

  const uuid = sessionless.generateUUID();
  await saveUser(uuid, payload.pubKey, payload.hash);

  return { uuid };
};

const checkHash = async (request: Request): object => {
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
    return {
      status: 403,
      error: 'Auth error'
    };
  }

  if(user.hash === params.get('hash')) {
    return { userUUID: uuid };
  } 
  return {
    status: 406,
    error: 'Not acceptable'
  };
};

const saveHash = async (request: Request): object => {
  const payload = await request.json();
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
    return {
      status: 403,
      error: 'Auth error'
    };
  }

  await saveUser(user.uuid, user.pubKey, payload.hash);

  return {
    userUUID: user.uuid
  };
};

const deleteUser = async (request: Request): object => {
  return {
    status: 501,
    error: 'Not implemented'
  };
};

const dispatch = async (request: Request): object => {
  const url = request.url;
  if(request.method === 'PUT') {
    return await register(request);
  }

  if(request.method === 'GET') {
    return await checkHash(request);
  }

  if(request.method === 'POST') {
    return await saveHash(request);
  }

  if(request.method === 'DELETE') {
    return await deleteUser(request);
  }

  return {
    status: 404,
    body: {error: 'not found'}
  };
};

Deno.serve({port: 3000}, async (request: Request) => {
  const res = await dispatch(request);
  console.log(res);
  return new Response(JSON.stringify(res), {
    headers: {
      "content-type": "application/json; charset=utf-8",
    }
  });
});
