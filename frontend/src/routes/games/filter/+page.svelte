<script lang="ts">
  import type { PageData } from './$types';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { get } from 'svelte/store';

  let { data }: { data: PageData } = $props();

  let startDate = $state(get(page).url.searchParams.get('start') ?? '');
  let endDate   = $state(get(page).url.searchParams.get('end') ?? '');
  let ind       = $state(get(page).url.searchParams.get('ind') ?? '');
  let vs        = $state(get(page).url.searchParams.get('vs') ?? '');
  let firstMin  = $state(get(page).url.searchParams.get('first_min') ?? '');
  let firstMax  = $state(get(page).url.searchParams.get('first_max') ?? '');
  let lastMin   = $state(get(page).url.searchParams.get('last_min') ?? '');
  let lastMax   = $state(get(page).url.searchParams.get('last_max') ?? '');

  function isSet(v: string | number) { return v !== '' && Number.isFinite(Number(v)); }

  function apply() {
    const p = new URLSearchParams();
    if (startDate)    p.set('start', startDate);
    if (endDate)      p.set('end', endDate);
    if (ind)          p.set('ind', ind);
    if (vs)           p.set('vs', vs);
    if (isSet(firstMin)) p.set('first_min', String(firstMin));
    if (isSet(firstMax)) p.set('first_max', String(firstMax));
    if (isSet(lastMin))  p.set('last_min',  String(lastMin));
    if (isSet(lastMax))  p.set('last_max',  String(lastMax));
    const q = p.toString();
    goto('/games' + (q ? '?' + q : ''));
  }
</script>

<div class="bg-white rounded-xl shadow border border-gray-200 p-6 max-w-md">
  <div class="space-y-4">
    <div>
      <label for="g-start" class="block text-sm font-medium text-gray-700 mb-1">시작일</label>
      <input id="g-start" type="date" bind:value={startDate}
             class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
    </div>
    <div>
      <label for="g-end" class="block text-sm font-medium text-gray-700 mb-1">종료일</label>
      <input id="g-end" type="date" bind:value={endDate}
             class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
    </div>
    <div>
      <label for="g-ind" class="block text-sm font-medium text-gray-700 mb-1">개인</label>
      <select id="g-ind" bind:value={ind} onchange={() => { if (!ind) vs = ''; }}
              class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]">
        <option value="">전체</option>
        {#each data.members as m}
        <option value={m}>{m}</option>
        {/each}
      </select>
    </div>
    <div>
      <label for="g-vs" class="block text-sm font-medium text-gray-700 mb-1">상대</label>
      <select id="g-vs" bind:value={vs}
              disabled={!ind}
              class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3] disabled:bg-gray-100 disabled:text-gray-400 disabled:cursor-not-allowed">
        <option value="">전체</option>
        {#each data.members as m}
        <option value={m}>{m}</option>
        {/each}
      </select>
    </div>
    <fieldset class="space-y-1">
      <legend class="block text-sm font-medium text-gray-700 mb-1">1위 점수</legend>
      <div class="flex items-center gap-2">
        <input id="g-first-min" type="number" step="100" bind:value={firstMin} placeholder="최솟값"
               class="flex-1 min-w-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
        <span class="text-gray-400 text-sm shrink-0">~</span>
        <input id="g-first-max" type="number" step="100" bind:value={firstMax} placeholder="최댓값"
               class="flex-1 min-w-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
      </div>
    </fieldset>
    <fieldset class="space-y-1">
      <legend class="block text-sm font-medium text-gray-700 mb-1">4위 점수</legend>
      <div class="flex items-center gap-2">
        <input id="g-last-min" type="number" step="100" bind:value={lastMin} placeholder="최솟값"
               class="flex-1 min-w-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
        <span class="text-gray-400 text-sm shrink-0">~</span>
        <input id="g-last-max" type="number" step="100" bind:value={lastMax} placeholder="최댓값"
               class="flex-1 min-w-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
      </div>
    </fieldset>
    <div class="flex gap-3 pt-2">
      <button onclick={apply}
              class="flex-1 bg-[#001c54] text-white py-2 rounded-lg text-sm font-medium hover:bg-[#002880] transition-colors">
        적용
      </button>
      <a href="/games"
         class="flex-1 border border-gray-300 text-gray-600 py-2 rounded-lg text-sm font-medium text-center hover:bg-gray-50 transition-colors">
        초기화
      </a>
    </div>
  </div>
</div>
