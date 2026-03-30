import type { PageLoad } from './$types';
import { fetchGame, fetchMembers } from '$lib/api';

export const load: PageLoad = async ({ params, fetch }) => {
  const id = parseInt(params.id);
  const [game, members] = await Promise.all([fetchGame(id, fetch), fetchMembers(fetch)]);
  return { id, game, members };
};
