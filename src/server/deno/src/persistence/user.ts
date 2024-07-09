const kv = await Deno.openKv();

export const saveUser = async (uuid, pubKey, hash) => {
  await kv.set([uuid], { 
    uuid,
    pubKey,
    hash,
    associatedKeys: {}
  });
  await kv.set([pubKey], uuid);
};

export const getUser = async (uuid) => {
  const user = await kv.get([uuid]);
  return user.value;
};

export const setValue = async (uuid, value) => {
  await kv.set([uuid, 'value'], value);
};

export const getValue = async (uuid)  => {
  return await kv.get([uuid, 'value']);
};

export const deleteUser = async (user): boolean => {
  await kv.delete([user.uuid]);
  return true;
};
