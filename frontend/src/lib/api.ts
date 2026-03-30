export interface RankingEntry {
	name: string;
	point: string;
	point_avg: string;
	games: string;
	first: string;
	second: string;
	third: string;
	fourth: string;
	rank_avg: string;
	first_ratio: string;
	second_ratio: string;
	third_ratio: string;
	fourth_ratio: string;
}

export interface GameEntry {
	id: string;
	recorded_at: string;
	first_name: string;
	first_score: string;
	first_seat: string;
	first_point: string;
	second_name: string;
	second_score: string;
	second_seat: string;
	second_point: string;
	third_name: string;
	third_score: string;
	third_seat: string;
	third_point: string;
	fourth_name: string;
	fourth_score: string;
	fourth_seat: string;
	fourth_point: string;
	leftover_score: string;
	used_dora_count: string;
	remarks: string;
}

export interface Stats {
	game_count: string;
	bankrupt: string;
	lowest_score: string;
	highest_score: string;
	wins_east: string;
	wins_south: string;
	wins_west: string;
	wins_north: string;
	lasts_east: string;
	lasts_south: string;
	lasts_west: string;
	lasts_north: string;
	average_score_east: string;
	average_score_south: string;
	average_score_west: string;
	average_score_north: string;
}

type Fetch = typeof globalThis.fetch;

export async function fetchRanking(params?: Record<string, string>, f: Fetch = fetch): Promise<RankingEntry[]> {
	const qs = params && Object.keys(params).length ? '?' + new URLSearchParams(params).toString() : '';
	const res = await f(`/api/ranking${qs}`);
	if (!res.ok) throw new Error('Failed to fetch ranking');
	return res.json();
}

export async function fetchGames(params?: Record<string, string>, f: Fetch = fetch): Promise<GameEntry[]> {
	const qs = params && Object.keys(params).length ? '?' + new URLSearchParams(params).toString() : '';
	const res = await f(`/api/games${qs}`);
	if (!res.ok) throw new Error('Failed to fetch games');
	return res.json();
}

export async function fetchStats(f: Fetch = fetch): Promise<Stats> {
	const res = await f('/api/stats');
	if (!res.ok) throw new Error('Failed to fetch stats');
	return res.json();
}

export async function fetchMembers(f: Fetch = fetch): Promise<string[]> {
	const res = await f('/api/members');
	if (!res.ok) throw new Error('Failed to fetch members');
	return res.json();
}

export async function postGame(data: Record<string, string>): Promise<void> {
	const res = await fetch('/api/game', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data)
	});
	if (!res.ok) throw new Error('Failed to save game');
}

export async function postMember(name: string): Promise<void> {
	const res = await fetch('/api/member', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ new_member: name })
	});
	if (!res.ok) throw new Error('Failed to save member');
}

export async function fetchGame(id: number, f: Fetch = fetch): Promise<Record<string, string>> {
	const res = await f(`/api/game/${id}`);
	if (!res.ok) throw new Error('Game not found');
	return res.json();
}

export async function updateGame(id: number, data: Record<string, string>): Promise<void> {
	const res = await fetch(`/api/game/${id}`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data)
	});
	if (!res.ok) throw new Error('Failed to update game');
}

export async function deleteGame(id: number): Promise<void> {
	const res = await fetch(`/api/game/${id}`, { method: 'DELETE' });
	if (!res.ok) throw new Error('Failed to delete game');
}
