<script lang="ts">
  import type { PageData } from './$types';
  import type { RankingEntry } from '$lib/api';

  let { data }: { data: PageData } = $props();

  type SortDir = 'asc' | 'desc';
  let sortCol = $state<keyof RankingEntry | null>(null);
  let sortDir = $state<SortDir>('asc');

  function sort(col: keyof RankingEntry) {
    if (sortCol === col) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortCol = col;
      sortDir = col === 'name' ? 'asc' : 'desc';
    }
  }

  const STR_COLS: (keyof RankingEntry)[] = ['name'];

  let sorted = $derived(() => {
    if (!sortCol) return data.ranking;
    const col = sortCol;
    return [...data.ranking].sort((a, b) => {
      const av = a[col], bv = b[col];
      const cmp = STR_COLS.includes(col)
        ? av.localeCompare(bv, 'ko')
        : parseFloat(av) - parseFloat(bv);
      return sortDir === 'asc' ? cmp : -cmp;
    });
  });

  function arrow(col: keyof RankingEntry) {
    if (sortCol !== col) return '';
    return sortDir === 'asc' ? ' ▲' : ' ▼';
  }

  const thClass =
    'px-3 py-2.5 whitespace-nowrap cursor-pointer select-none hover:bg-[#37aaac] transition-colors text-left';
  const divClass = 'px-1 border-l border-white/30';
</script>

<div class="flex justify-end mb-3">
  <a href="/ranking/filter" class="text-sm text-[#001c54] hover:text-[#43c1c3] transition-colors font-medium">조건 지정 →</a>
</div>

<div class="overflow-x-auto rounded-xl shadow border border-gray-200 bg-white">
  <table class="w-full text-sm border-collapse">
    <thead>
      <tr class="bg-[#43c1c3] text-white">
        <th class={thClass} onclick={() => sort('name')}>이름{arrow('name')}</th>
        <th class={thClass} onclick={() => sort('point')}>승점{arrow('point')}</th>
        <th class={thClass} onclick={() => sort('point_avg')}>평균 승점{arrow('point_avg')}</th>
        <th class={thClass} onclick={() => sort('games')}>대국수{arrow('games')}</th>
        <th class="{divClass} hidden sm:table-cell"></th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('first')}>1위{arrow('first')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('second')}>2위{arrow('second')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('third')}>3위{arrow('third')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('fourth')}>4위{arrow('fourth')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('rank_avg')}>평균 순위{arrow('rank_avg')}</th>
        <th class="{divClass} hidden sm:table-cell"></th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('first_ratio')}>1위율{arrow('first_ratio')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('second_ratio')}>2위율{arrow('second_ratio')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('third_ratio')}>3위율{arrow('third_ratio')}</th>
        <th class="{thClass} hidden sm:table-cell" onclick={() => sort('fourth_ratio')}>4위율{arrow('fourth_ratio')}</th>
      </tr>
    </thead>
    <tbody>
      {#each sorted() as entry, i}
      <tr onclick={() => location.href = `/members/${encodeURIComponent(entry.name)}`}
          class="border-t border-gray-100 hover:bg-blue-50/40 transition-colors cursor-pointer {i % 2 === 1 ? 'bg-gray-50/60' : 'bg-white'}">
        <td class="px-3 py-2 whitespace-nowrap font-medium">
          <a href="/members/{encodeURIComponent(entry.name)}"
             class="hover:text-[#43c1c3] transition-colors">{entry.name}</a>
        </td>
        <td class="px-3 py-2 whitespace-nowrap">{entry.point}</td>
        <td class="px-3 py-2 whitespace-nowrap">{entry.point_avg}</td>
        <td class="px-3 py-2 whitespace-nowrap">{entry.games}</td>
        <td class="border-l border-gray-200 hidden sm:table-cell"></td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.first}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.second}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.third}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.fourth}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.rank_avg}</td>
        <td class="border-l border-gray-200 hidden sm:table-cell"></td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.first_ratio}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.second_ratio}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.third_ratio}</td>
        <td class="px-3 py-2 whitespace-nowrap hidden sm:table-cell">{entry.fourth_ratio}</td>
      </tr>
      {/each}
      {#if sorted().length === 0}
      <tr><td colspan="15" class="px-4 py-8 text-center text-gray-400">데이터가 없습니다</td></tr>
      {/if}
    </tbody>
  </table>
</div>


