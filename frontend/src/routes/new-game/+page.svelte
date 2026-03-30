<script lang="ts">
  import type { PageData } from './$types';
  import { postGame, postMember } from '$lib/api';
  import { goto } from '$app/navigation';

  let { data }: { data: PageData } = $props();

  let members = $state<string[]>([]);

  let east   = $state('');
  let south  = $state('');
  let west   = $state('');
  let north  = $state('');
  let se = $state(''), ss = $state(''), sw = $state(''), sn = $state('');
  let leftover = $state('0');
  let dora     = $state('4');
  let remarks  = $state('');
  let error    = $state('');
  let sending  = $state(false);

  type PlayerRow = {
    label: string;
    nameVal: string;
    setName: (v: string) => void;
    scoreVal: string;
    setScore: (v: string) => void;
    listId: string;
  };

  function setEast(v: string) { east = v; }
  function setSouth(v: string) { south = v; }
  function setWest(v: string) { west = v; }
  function setNorth(v: string) { north = v; }
  function setSe(v: string) { se = v; }
  function setSs(v: string) { ss = v; }
  function setSw(v: string) { sw = v; }
  function setSn(v: string) { sn = v; }

  const playerRows = $derived<PlayerRow[]>([
    { label: '동', nameVal: east, setName: setEast, scoreVal: se, setScore: setSe, listId: 'members-e' },
    { label: '남', nameVal: south, setName: setSouth, scoreVal: ss, setScore: setSs, listId: 'members-s' },
    { label: '서', nameVal: west, setName: setWest, scoreVal: sw, setScore: setSw, listId: 'members-w' },
    { label: '북', nameVal: north, setName: setNorth, scoreVal: sn, setScore: setSn, listId: 'members-n' }
  ]);

  $effect(() => {
    members = data.members;
  });

  async function ensureNewMembers(names: string[]): Promise<boolean> {
    const newNames = names.filter(n => n.trim() && !members.includes(n.trim()));
    if (newNames.length === 0) return true;
    const list = newNames.map(n => `「${n}」`).join(', ');
    if (!confirm(`새 회원 ${list}을(를) 추가하시겠습니까?`)) return false;
    for (const name of newNames) {
      await postMember(name.trim());
      members = [...members, name.trim()];
    }
    return true;
  }

  const total = $derived(() => {
    const sum = [se, ss, sw, sn].reduce((acc, v) => acc + (parseFloat(v) || 0), 0);
    return sum + (parseFloat(leftover) || 0);
  });

  const totalOk = $derived(() => Math.abs(total() - 100000) < 0.001);

  const totalColor = $derived(() =>
    totalOk() ? 'text-green-600' : 'text-red-500'
  );

  async function submit(e: SubmitEvent) {
    e.preventDefault();
    error = '';
    if (!totalOk()) { error = '점수 합계가 100000이어야 합니다.'; return; }
    sending = true;
    try {
      const ok = await ensureNewMembers([east, south, west, north]);
      if (!ok) { sending = false; return; }
      await postGame({
        eastName: east, southName: south, westName: west, northName: north,
        eastScore: se, southScore: ss, westScore: sw, northScore: sn,
        leftover: String(leftover), used_dora_count: String(dora), remarks
      });
      goto('/games');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : '오류가 발생했습니다.';
    } finally {
      sending = false;
    }
  }
</script>

<div class="bg-white rounded-xl shadow border border-gray-200 p-6 max-w-xl mx-auto">
  {#if error}
  <div class="mb-4 p-3 bg-red-50 border border-red-200 text-red-700 rounded-lg text-sm">{error}</div>
  {/if}

  <form onsubmit={submit} class="space-y-5">
    <!-- Players & Scores -->
    <div>
      <div class="flex items-center justify-between mb-2">
        <span class="text-sm font-medium text-gray-700">자리별 이름 / 점수</span>
        <span class="text-sm font-semibold {totalColor()}">합계: {total().toFixed(0)}</span>
      </div>
      <div class="space-y-2">
        {#each playerRows as { label, nameVal, setName, scoreVal, setScore, listId }}
        <div class="flex items-center gap-2">
          <span class="text-xs font-semibold text-gray-500 w-6 shrink-0">{label}</span>
          <input type="text" list={listId} value={nameVal}
                 oninput={(e) => setName((e.target as HTMLInputElement).value)}
                 required placeholder="선택 또는 입력"
                 class="flex-1 min-w-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
          <datalist id={listId}>
            {#each members as m}
            <option value={m}></option>
            {/each}
          </datalist>
          <input type="number" step="100" value={scoreVal}
                 oninput={(e) => setScore((e.target as HTMLInputElement).value)}
                 required placeholder="점수"
                 class="w-24 shrink-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
        </div>
        {/each}
      </div>
    </div>

    <div class="grid grid-cols-2 gap-4">
      <div>
        <label for="game-leftover" class="block text-sm font-medium text-gray-700 mb-1">잔여 공탁</label>
        <input id="game-leftover" type="number" step="100" bind:value={leftover}
               class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
      </div>
      <div>
        <label for="game-dora" class="block text-sm font-medium text-gray-700 mb-1">도라 수</label>
        <input id="game-dora" type="number" bind:value={dora}
               class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
      </div>
    </div>

    <div>
      <label for="game-remarks" class="block text-sm font-medium text-gray-700 mb-1">비고</label>
      <input id="game-remarks" type="text" bind:value={remarks} placeholder="선택사항"
             class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
    </div>

    <button type="submit" disabled={sending}
            class="w-full bg-[#001c54] text-white py-2.5 rounded-lg text-sm font-medium hover:bg-[#002880] transition-colors disabled:opacity-50">
      {sending ? '저장 중...' : '저장'}
    </button>
  </form>
</div>
