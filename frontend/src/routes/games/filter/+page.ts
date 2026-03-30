import type { PageLoad } from './$types';
import { fetchMembers } from '$lib/api';

export const load: PageLoad = async ({ fetch }) => {
  return { members: await fetchMembers(fetch) };
};
