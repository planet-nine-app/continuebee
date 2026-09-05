import { BlobsServer } from '@netlify/blobs/server';

const port = Number(process.env.BLOBS_LOCAL_PORT) || 8999;
const token = process.env.BLOBS_LOCAL_TOKEN || 'local-dev-token';
const directory = process.env.BLOBS_LOCAL_DIR || 'data/netlify-blobs';

const server = new BlobsServer({ directory, port, token });
await server.start();

process.env.BLOBS_LOCAL_URL = `http://localhost:${port}`;
process.env.BLOBS_LOCAL_TOKEN = token;

const { createClient } = await import('../src/persistence/client.netlify-blobs.js');
const client = await createClient().on('error', () => {}).connect();

console.log('SET ->', await client.set('test_key', JSON.stringify({ hello: 'world' })));
console.log('GET ->', await client.get('test_key'));
console.log('GET missing ->', await client.get('missing_key'));
console.log('DEL ->', await client.del('test_key'));
console.log('GET after DEL ->', await client.get('test_key'));

await server.stop();
process.exit(0);
