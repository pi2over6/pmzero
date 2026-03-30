import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';

const backendOrigin = env.BACKEND_ORIGIN ?? 'http://127.0.0.1:80';

export const handle: Handle = async ({ event, resolve }) => {
  const response = await resolve(event);
  response.headers.set('Cache-Control', 'no-store, no-cache, must-revalidate');
  return response;
};

export const handleFetch: HandleFetch = ({ request, fetch }) => {
  const url = new URL(request.url);
  if (url.pathname.startsWith('/api/')) {
    const backend = new URL(backendOrigin);
    url.hostname = backend.hostname;
    url.port = backend.port;
    url.protocol = backend.protocol;
    url.username = backend.username;
    url.password = backend.password;
    return fetch(new Request(url.toString(), request));
  }
  return fetch(request);
};
