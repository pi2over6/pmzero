import type { PageLoad } from './$types';
import { fetchStats } from '$lib/api';

export const load: PageLoad = async ({ fetch }) => {
  return { stats: await fetchStats(fetch) };
};
