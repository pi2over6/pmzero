import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
  const res = await fetch(`/api/member/${encodeURIComponent(params.name)}`);
  if (!res.ok) throw new Error('Member not found');
  const stat = await res.json();
  return { stat };
};
