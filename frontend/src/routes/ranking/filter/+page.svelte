<script lang="ts">
  import type { PageData } from './$types';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { get } from 'svelte/store';

  let { data }: { data: PageData } = $props();

  let startDate = $state(get(page).url.searchParams.get('start') ?? '');
  let endDate   = $state(get(page).url.searchParams.get('end') ?? '');

  function apply() {
    const p = new URLSearchParams();
    if (startDate) p.set('start', startDate);
    if (endDate)   p.set('end', endDate);
    const q = p.toString();
    goto('/ranking' + (q ? '?' + q : ''));
  }
</script>

<div class="bg-white rounded-xl shadow border border-gray-200 p-6 max-w-md">
  <div class="space-y-4">
    <div>
      <label for="r-start" class="block text-sm font-medium text-gray-700 mb-1">시작일</label>
      <input id="r-start" type="date" bind:value={startDate}
             class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
    </div>
    <div>
      <label for="r-end" class="block text-sm font-medium text-gray-700 mb-1">종료일</label>
      <input id="r-end" type="date" bind:value={endDate}
             class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
    </div>
    <div class="flex gap-3 pt-2">
      <button onclick={apply}
              class="flex-1 bg-[#001c54] text-white py-2 rounded-lg text-sm font-medium hover:bg-[#002880] transition-colors">
        적용
      </button>
      <a href="/ranking"
         class="flex-1 border border-gray-300 text-gray-600 py-2 rounded-lg text-sm font-medium text-center hover:bg-gray-50 transition-colors">
        초기화
      </a>
    </div>
  </div>
</div>
