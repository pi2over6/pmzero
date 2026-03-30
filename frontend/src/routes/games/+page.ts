import type { PageLoad } from './$types';
import { fetchGames } from '$lib/api';

const PAGE_SIZE = 30;

export const load: PageLoad = async ({ url, fetch }) => {
  const params: Record<string, string> = {};
  for (const [k, v] of url.searchParams.entries()) {
    params[k] = v;
  }
  params['limit'] = String(PAGE_SIZE);
  params['offset'] = '0';
  return { games: await fetchGames(params, fetch), params, pageSize: PAGE_SIZE };
};
