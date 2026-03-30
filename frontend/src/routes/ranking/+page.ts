import type { PageLoad } from './$types';
import { fetchRanking } from '$lib/api';

export const load: PageLoad = async ({ url, fetch }) => {
	const params: Record<string, string> = {};
	for (const [k, v] of url.searchParams) params[k] = v;
	const ranking = await fetchRanking(params, fetch);
	return { ranking };
};
