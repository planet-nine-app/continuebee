import serverless from 'serverless-http';
import { connectLambda } from '@netlify/blobs';
import app from '../../continuebee.js';

const rawHandler = serverless(app);

export const handler = async (event, context) => {
  if (event.blobs) {
    connectLambda(event);
  }

  return rawHandler(event, context);
};
