<script lang="ts">
  import type { PageData } from './$types';
  import { page } from '$app/stores';

  let { data }: { data: PageData } = $props();
  const s = $derived(data.stat);

  function scoreColor(score: number) {
    if (score < 0) return 'text-red-500 font-bold';
    if (score >= 80000) return 'text-blue-500 font-bold';
    return 'text-gray-700';
  }

  function rankColor(rank: number) {
    if (rank === 1) return 'text-blue-600 font-bold';
    if (rank === 4) return 'text-red-500 font-bold';
    return 'text-gray-600';
  }

  function fmtDate(d: string) {
    return d.replace(/:\d{2}$/, '');
  }

  const trendPoints = $derived(
    (data.stat.point_trend as {point: string}[]).map(p => parseFloat(p.point))
  );
  const trendMin   = $derived(Math.min(...trendPoints));
  const trendMax   = $derived(Math.max(...trendPoints));
  const trendRange = $derived(trendMax - trendMin || 1);
  const trendW     = $derived(100 / (trendPoints.length - 1));
  const trendPolyline = $derived(
    trendPoints.map((v, i) => `${i * trendW},${40 - ((v - trendMin) / trendRange) * 38 + 1}`).join(' ')
  );
  const trendZeroY = $derived(40 - ((0 - trendMin) / trendRange) * 40);
</script>

<div class="space-y-5 max-w-2xl">

  <!-- Header -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
    <div class="flex items-end justify-between">
      <div>
        <p class="text-xs text-gray-400 mb-0.5">플레이어</p>
        <h1 class="text-2xl font-bold text-[#001c54]">{s.name}</h1>
      </div>
      <div class="text-right">
        <p class="text-xs text-gray-400">총 승점</p>
        <p class="text-2xl font-bold {parseFloat(s.point) >= 0 ? 'text-[#001c54]' : 'text-red-500'}">{s.point}</p>
      </div>
    </div>
  </div>

  <!-- Core stats -->
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
    {#each [
      ['대국수', s.games, ''],
      ['평균 승점', s.point_avg, ''],
      ['평균 순위', s.rank_avg, ''],
      ['평균 점수', s.avg_score, ''],
    ] as [label, val, _]}
    <div class="bg-white rounded-xl shadow border border-gray-200 p-4 text-center">
      <p class="text-xs text-gray-400 mb-1">{label}</p>
      <p class="text-xl font-bold text-[#001c54]">{val}</p>
    </div>
    {/each}
  </div>

  <!-- Rank distribution -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
    <h2 class="text-sm font-semibold text-gray-700 mb-3">순위 분포</h2>
    <div class="grid grid-cols-4 gap-3 text-center">
      {#each [
        ['1위', s.first, s.first_ratio, 'bg-blue-50 border-blue-200 text-blue-700'],
        ['2위', s.second, s.second_ratio, 'bg-gray-50 border-gray-200 text-gray-700'],
        ['3위', s.third, s.third_ratio, 'bg-gray-50 border-gray-200 text-gray-700'],
        ['4위', s.fourth, s.fourth_ratio, 'bg-red-50 border-red-200 text-red-600'],
      ] as [label, count, ratio, cls]}
      <div class="rounded-lg border p-3 {cls}">
        <p class="text-xs font-medium mb-1">{label}</p>
        <p class="text-lg font-bold">{count}</p>
        <p class="text-xs opacity-70">{ratio}%</p>
      </div>
      {/each}
    </div>
  </div>

  <!-- Seat breakdown (wins / lasts / avg score) -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
    <h2 class="text-sm font-semibold text-gray-700 mb-3">자리별 상세</h2>
    <div class="overflow-x-auto">
      <table class="w-full text-sm text-center">
        <thead>
          <tr class="text-xs text-gray-400 border-b border-gray-100">
            <th class="py-1.5 px-2 text-left">자리</th>
            <th class="py-1.5 px-2">대국</th>
            <th class="py-1.5 px-2">1위</th>
            <th class="py-1.5 px-2">4위</th>
            <th class="py-1.5 px-2">1위율</th>
            <th class="py-1.5 px-2">평균 점수</th>
          </tr>
        </thead>
        <tbody>
          {#each s.seat_stats as seat}
          <tr class="border-t border-gray-50">
            <td class="py-2 px-2 text-left font-semibold text-[#001c54]">{seat.seat}</td>
            <td class="py-2 px-2 text-gray-600">{seat.played}</td>
            <td class="py-2 px-2 text-blue-600 font-medium">{seat.first}</td>
            <td class="py-2 px-2 text-red-500 font-medium">{seat.last}</td>
            <td class="py-2 px-2 font-medium">{seat.first_ratio}{seat.first_ratio !== '-' ? '%' : ''}</td>
            <td class="py-2 px-2 {scoreColor(Number(seat.avg_score))}">{seat.avg_score}</td>
          </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  <!-- Inline stats (mirrors main stats page) -->
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
    {#each [
      ['들통', s.bankrupt, 'text-red-500'],
      ['최고 점수', s.best_score, 'text-green-600'],
      ['최저 점수', s.worst_score, 'text-orange-500'],
      ['평균 점수', s.avg_score, 'text-[#001c54]'],
    ] as [label, val, cls]}
    <div class="bg-white rounded-xl shadow border border-gray-200 p-4 text-center">
      <p class="text-xs text-gray-400 mb-1">{label}</p>
      <p class="text-xl font-bold {cls}">{val}</p>
    </div>
    {/each}
  </div>

  <div class="grid sm:grid-cols-2 gap-5">
    <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
      <h2 class="text-xs font-semibold text-gray-400 uppercase mb-3">자리별 1위 횟수</h2>
      <div class="grid grid-cols-2 gap-3">
        {#each s.seat_stats as seat}
        <div class="flex items-center gap-2">
          <span class="w-8 h-8 rounded-full bg-[#43c1c3]/20 text-[#006e70] text-sm font-bold flex items-center justify-center">{seat.seat}</span>
          <span class="text-lg font-bold text-[#001c54]">{seat.first}</span>
        </div>
        {/each}
      </div>
    </div>
    <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
      <h2 class="text-xs font-semibold text-gray-400 uppercase mb-3">자리별 4위 횟수</h2>
      <div class="grid grid-cols-2 gap-3">
        {#each s.seat_stats as seat}
        <div class="flex items-center gap-2">
          <span class="w-8 h-8 rounded-full bg-red-100 text-red-600 text-sm font-bold flex items-center justify-center">{seat.seat}</span>
          <span class="text-lg font-bold text-[#001c54]">{seat.last}</span>
        </div>
        {/each}
      </div>
    </div>
  </div>

  <div class="grid sm:grid-cols-2 gap-5">
    <!-- Most played with -->
    <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
      <h2 class="text-sm font-semibold text-gray-700 mb-3">자주 함께한 플레이어</h2>
      {#if s.co_players.length === 0}
      <p class="text-sm text-gray-400">데이터 없음</p>
      {:else}
      <div class="space-y-2 text-sm">
        {#each s.co_players as p}
        <div class="flex justify-between items-center">
          <a href="/members/{encodeURIComponent(p.name)}"
             class="text-[#001c54] hover:text-[#43c1c3] font-medium transition-colors">{p.name}</a>
          <span class="text-gray-500">{p.count}국</span>
        </div>
        {/each}
      </div>
      {/if}
    </div>

    <!-- Recent games -->
    <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
      <h2 class="text-sm font-semibold text-gray-700 mb-3">최근 대국 (최대 10)</h2>
      {#if s.recent.length === 0}
      <p class="text-sm text-gray-400">데이터 없음</p>
      {:else}
      <div class="space-y-2 text-sm">
        {#each s.recent as g}
        <div class="flex justify-between items-center">
          <span class="text-gray-400 text-xs">{fmtDate(g.date)}</span>
          <span class="font-semibold {rankColor(g.rank)}">{g.rank}위</span>
          <span class="{scoreColor(g.score)}">{g.score}</span>
          <span class="text-gray-500 w-12 text-right">{g.point}</span>
        </div>
        {/each}
      </div>
      {/if}
    </div>
  </div>

  <!-- Point trend -->
  {#if s.point_trend.length > 1}
  <div class="bg-white rounded-xl shadow border border-gray-200 p-5">
    <h2 class="text-sm font-semibold text-gray-700 mb-3">승점 추이</h2>
    <svg viewBox="0 0 100 40" preserveAspectRatio="none" class="w-full h-24">
      {#if trendMin < 0 && trendMax > 0}
      <line x1="0" y1={trendZeroY} x2="100" y2={trendZeroY} stroke="#e5e7eb" stroke-width="0.5" />
      {/if}
      <polyline
        points={trendPolyline}
        fill="none" stroke="#43c1c3" stroke-width="1.5" stroke-linejoin="round" stroke-linecap="round"
      />
    </svg>
    <div class="flex justify-between text-xs text-gray-400 mt-1">
      <span>{s.point_trend[0].date.slice(0, 10)}</span>
      <span>{s.point_trend.at(-1).date.slice(0, 10)}</span>
    </div>
  </div>
  {/if}

</div>
