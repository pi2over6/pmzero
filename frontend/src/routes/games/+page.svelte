<script lang="ts">
  import { onMount } from 'svelte';
  import type { PageData } from './$types';
  import type { GameEntry } from '$lib/api';
  import { fetchGames } from '$lib/api';

  let { data }: { data: PageData } = $props();
  let selected = $state<GameEntry | null>(null);
  let games = $state<GameEntry[]>([]);
  let loading = $state(false);
  let exhausted = $state(false);
  let sentinel: HTMLElement;

  $effect(() => {
    games = data.games;
    exhausted = data.games.length < data.pageSize;
  });

  async function loadMore() {
    if (loading || exhausted) return;
    loading = true;
    const params = { ...data.params, offset: String(games.length), limit: String(data.pageSize) };
    const more = await fetchGames(params);
    if (more.length < data.pageSize) exhausted = true;
    games = [...games, ...more];
    loading = false;
  }

  onMount(() => {
    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting) loadMore();
    }, { rootMargin: '200px' });
    observer.observe(sentinel);
    return () => observer.disconnect();
  });

  function fmtTime(t: string) {
    return t.replace(/:\d{2}$/, '');
  }

  function scoreColor(score: string) {
    const n = parseInt(score);
    if (n < 0) return 'text-red-500 font-bold';
    if (n >= 80000) return 'text-blue-500 font-bold';
    return 'text-gray-500';
  }
</script>

<div class="flex justify-end mb-3">
  <a href="/games/filter" class="text-sm text-[#001c54] hover:text-[#43c1c3] transition-colors font-medium">조건 지정 →</a>
</div>

<!-- Mobile: card list -->
<div class="sm:hidden space-y-3">
  {#each games as g}
  <button onclick={() => selected = g}
          class="w-full bg-white rounded-xl shadow border border-gray-200 p-4 text-sm text-left">
    <div class="grid grid-cols-4 gap-2 text-center">
      {#each [['1위', g.first_name, g.first_score], ['2위', g.second_name, g.second_score], ['3위', g.third_name, g.third_score], ['4위', g.fourth_name, g.fourth_score]] as [rank, name, score]}
      <div>
        <p class="text-xs text-gray-400">{rank}</p>
        <p class="font-semibold text-[#001c54]">{name}</p>
        <p class="text-xs {scoreColor(score)}">{score}</p>
      </div>
      {/each}
    </div>
  </button>
  {/each}
  {#if data.games.length === 0}
  <p class="text-center text-gray-400 py-8">대국 기록이 없습니다</p>
  {/if}
</div>

<!-- Mobile detail popup -->
{#if selected}
<button class="sm:hidden fixed inset-0 z-40 bg-black/40 cursor-default border-0 p-0"
        aria-label="닫기" onclick={() => selected = null}></button>
<div class="sm:hidden fixed bottom-0 inset-x-0 z-50 bg-white rounded-t-2xl shadow-xl p-5">
  <div class="flex items-center justify-between mb-4">
    <span class="text-sm text-gray-400">{fmtTime(selected.recorded_at)}</span>
    <button onclick={() => selected = null}
            class="text-gray-400 hover:text-gray-600 p-1" aria-label="닫기">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
      </svg>
    </button>
  </div>
  <div class="grid grid-cols-4 gap-2 text-center text-sm mb-4">
    {#each [
      ['1위', selected.first_name,  selected.first_seat,  selected.first_score,  selected.first_point],
      ['2위', selected.second_name, selected.second_seat, selected.second_score, selected.second_point],
      ['3위', selected.third_name,  selected.third_seat,  selected.third_score,  selected.third_point],
      ['4위', selected.fourth_name, selected.fourth_seat, selected.fourth_score, selected.fourth_point],
    ] as [rank, name, seat, score, point]}
    <div class="bg-gray-50 rounded-lg p-2">
      <p class="text-xs text-gray-400">{rank} ({seat})</p>
      <p class="font-semibold text-[#001c54]">{name}</p>
      <p class="text-xs {scoreColor(score)}">{score}</p>
      <p class="text-xs {scoreColor(score)}">({point})</p>
    </div>
    {/each}
  </div>
  <div class="grid grid-cols-2 gap-3 text-sm">
    <div class="bg-gray-50 rounded-lg p-3">
      <p class="text-xs text-gray-400 mb-1">잔여 공탁</p>
      <p class="font-semibold text-[#001c54]">{selected.leftover_score}</p>
    </div>
    <div class="bg-gray-50 rounded-lg p-3">
      <p class="text-xs text-gray-400 mb-1">도라 수</p>
      <p class="font-semibold text-[#001c54]">{selected.used_dora_count}</p>
    </div>
    {#if selected.remarks}
    <div class="bg-gray-50 rounded-lg p-3 col-span-2">
      <p class="text-xs text-gray-400 mb-1">비고</p>
      <p class="font-semibold text-[#001c54]">{selected.remarks}</p>
    </div>
    {/if}
  </div>
  <a href="/games/{selected.id}/edit"
     class="mt-4 block w-full text-center border border-gray-300 text-gray-600 py-2.5 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors">
    수정
  </a>
</div>
{/if}

<!-- Desktop: table -->
<div class="hidden sm:block overflow-x-auto rounded-xl shadow border border-gray-200 bg-white">
  <table class="w-full text-sm min-w-max border-collapse">
    <thead>
      <tr class="bg-[#43c1c3] text-white">
        <th class="px-3 py-2.5 whitespace-nowrap text-left">날짜</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">1위</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">2위</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">3위</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">4위</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">잔여 공탁</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">도라 수</th>
        <th class="px-3 py-2.5 whitespace-nowrap text-left">비고</th>
        <th class="px-2 py-2.5"></th>
      </tr>
    </thead>
    <tbody>
      {#each games as g, i}
      <tr class="border-t border-gray-100 hover:bg-blue-50/40 transition-colors {i % 2 === 1 ? 'bg-gray-50/60' : 'bg-white'}">
        <td class="px-3 py-2 whitespace-nowrap">{fmtTime(g.recorded_at)}</td>
        <td class="px-3 py-2 whitespace-nowrap font-medium">{g.first_name} <span class="text-xs text-gray-400">({g.first_seat})</span> <span class="text-xs {scoreColor(g.first_score)}">{g.first_score}</span> <span class="text-xs {scoreColor(g.first_score)}">({g.first_point})</span></td>
        <td class="px-3 py-2 whitespace-nowrap">{g.second_name} <span class="text-xs text-gray-400">({g.second_seat})</span> <span class="text-xs {scoreColor(g.second_score)}">{g.second_score}</span> <span class="text-xs {scoreColor(g.second_score)}">({g.second_point})</span></td>
        <td class="px-3 py-2 whitespace-nowrap">{g.third_name} <span class="text-xs text-gray-400">({g.third_seat})</span> <span class="text-xs {scoreColor(g.third_score)}">{g.third_score}</span> <span class="text-xs {scoreColor(g.third_score)}">({g.third_point})</span></td>
        <td class="px-3 py-2 whitespace-nowrap">{g.fourth_name} <span class="text-xs text-gray-400">({g.fourth_seat})</span> <span class="text-xs {scoreColor(g.fourth_score)}">{g.fourth_score}</span> <span class="text-xs {scoreColor(g.fourth_score)}">({g.fourth_point})</span></td>
        <td class="px-3 py-2 whitespace-nowrap">{g.leftover_score}</td>
        <td class="px-3 py-2 whitespace-nowrap">{g.used_dora_count}</td>
        <td class="px-3 py-2">{g.remarks}</td>
        <td class="px-2 py-2 whitespace-nowrap">
          <a href="/games/{g.id}/edit" class="text-xs text-gray-400 hover:text-[#001c54] transition-colors">수정</a>
        </td>
      </tr>
      {/each}
      {#if games.length === 0}
      <tr><td colspan="8" class="px-4 py-8 text-center text-gray-400">대국 기록이 없습니다</td></tr>
      {/if}
    </tbody>
  </table>
</div>

<!-- Infinite scroll sentinel -->
<div bind:this={sentinel} class="h-1"></div>
{#if loading}
<p class="text-center text-sm text-gray-400 py-4">불러오는 중...</p>
{/if}

<!-- Infinite scroll sentinel -->
<div bind:this={sentinel} class="h-1"></div>
{#if loading}
<p class="text-center text-sm text-gray-400 py-4">불러오는 중...</p>
{/if}
