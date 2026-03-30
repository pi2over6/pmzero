<script lang="ts">
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
  const s = $derived(data.stats);

  function card(label: string, value: string) {
    return { label, value };
  }
</script>

<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
  <!-- General -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-4 col-span-2 sm:col-span-3 lg:col-span-4">
    <h2 class="text-xs font-semibold text-gray-400 uppercase mb-3">전체</h2>
    <div class="flex flex-wrap gap-6">
      <div><span class="text-2xl font-bold text-[#001c54]">{s.game_count}</span><p class="text-xs text-gray-500 mt-0.5">총 대국수</p></div>
      <div><span class="text-2xl font-bold text-red-500">{s.bankrupt}</span><p class="text-xs text-gray-500 mt-0.5">들통</p></div>
      <div><span class="text-2xl font-bold text-orange-500">{s.lowest_score}</span><p class="text-xs text-gray-500 mt-0.5">최저 점수</p></div>
      <div><span class="text-2xl font-bold text-green-600">{s.highest_score}</span><p class="text-xs text-gray-500 mt-0.5">최고 점수</p></div>
    </div>
  </div>

  <!-- Wins by seat -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-4 col-span-2">
    <h2 class="text-xs font-semibold text-gray-400 uppercase mb-3">자리별 1위</h2>
    <div class="grid grid-cols-2 gap-3">
      {#each [['동', s.wins_east], ['남', s.wins_south], ['서', s.wins_west], ['북', s.wins_north]] as [seat, val]}
      <div class="flex items-center gap-2">
        <span class="w-8 h-8 rounded-full bg-[#43c1c3]/20 text-[#006e70] text-sm font-bold flex items-center justify-center">{seat}</span>
        <span class="text-lg font-bold text-[#001c54]">{val}</span>
      </div>
      {/each}
    </div>
  </div>

  <!-- Lasts by seat -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-4 col-span-2">
    <h2 class="text-xs font-semibold text-gray-400 uppercase mb-3">자리별 4위</h2>
    <div class="grid grid-cols-2 gap-3">
      {#each [['동', s.lasts_east], ['남', s.lasts_south], ['서', s.lasts_west], ['북', s.lasts_north]] as [seat, val]}
      <div class="flex items-center gap-2">
        <span class="w-8 h-8 rounded-full bg-red-100 text-red-600 text-sm font-bold flex items-center justify-center">{seat}</span>
        <span class="text-lg font-bold text-[#001c54]">{val}</span>
      </div>
      {/each}
    </div>
  </div>

  <!-- Average score by seat -->
  <div class="bg-white rounded-xl shadow border border-gray-200 p-4 col-span-2 sm:col-span-3 lg:col-span-4">
    <h2 class="text-xs font-semibold text-gray-400 uppercase mb-3">자리별 평균 점수</h2>
    <div class="flex flex-wrap gap-6">
      {#each [['동', s.average_score_east], ['남', s.average_score_south], ['서', s.average_score_west], ['북', s.average_score_north]] as [seat, val]}
      <div>
        <span class="text-xl font-bold text-[#001c54]">{val}</span>
        <p class="text-xs text-gray-500 mt-0.5">{seat}</p>
      </div>
      {/each}
    </div>
  </div>
</div>
