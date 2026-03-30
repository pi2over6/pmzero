<script lang="ts">
  import type { PageData } from './$types';
  import { updateGame, deleteGame } from '$lib/api';
  import { goto } from '$app/navigation';

  let { data }: { data: PageData } = $props();

  let east   = $state('');
  let south  = $state('');
  let west   = $state('');
  let north  = $state('');
  let se = $state('');
  let ss = $state('');
  let sw = $state('');
  let sn = $state('');
  let leftover = $state('0');
  let dora     = $state('0');
  let remarks  = $state('');
  let error    = $state('');
  let sending  = $state(false);
  let deleting = $state(false);

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
    { label: '동', nameVal: east, setName: setEast, scoreVal: se, setScore: setSe, listId: 'em-1' },
    { label: '남', nameVal: south, setName: setSouth, scoreVal: ss, setScore: setSs, listId: 'em-2' },
    { label: '서', nameVal: west, setName: setWest, scoreVal: sw, setScore: setSw, listId: 'em-3' },
    { label: '북', nameVal: north, setName: setNorth, scoreVal: sn, setScore: setSn, listId: 'em-4' }
  ]);

  $effect(() => {
    east = data.game.eastName ?? '';
    south = data.game.southName ?? '';
    west = data.game.westName ?? '';
    north = data.game.northName ?? '';
    se = data.game.eastScore ?? '';
    ss = data.game.southScore ?? '';
    sw = data.game.westScore ?? '';
    sn = data.game.northScore ?? '';
    leftover = data.game.leftover ?? '0';
    dora = data.game.used_dora_count ?? '0';
    remarks = data.game.remarks ?? '';
  });

  async function handleDelete() {
    if (!confirm(`대국 #${data.id}를 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다.`)) return;
    deleting = true;
    try {
      await deleteGame(data.id);
      goto('/games');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : '삭제 중 오류가 발생했습니다.';
    } finally {
      deleting = false;
    }
  }

  const total = $derived(() => {
    const sum = [se, ss, sw, sn].reduce((acc, v) => acc + (parseFloat(v) || 0), 0);
    return sum + (parseFloat(leftover) || 0);
  });
  const totalOk = $derived(() => Math.abs(total() - 100000) < 0.001);
  const totalColor = $derived(() => totalOk() ? 'text-green-600' : 'text-red-500');

  async function submit(e: SubmitEvent) {
    e.preventDefault();
    error = '';
    if (!totalOk()) { error = '점수 합계가 100000이어야 합니다.'; return; }
    sending = true;
    try {
      await updateGame(data.id, {
        eastName: east, southName: south, westName: west, northName: north,
        eastScore: se, southScore: ss, westScore: sw, northScore: sn,
        leftover: String(leftover), used_dora_count: String(dora), remarks,
      });
      goto('/games');
    } catch (e: unknown) {
      error = e instanceof Error ? e.message : '오류가 발생했습니다.';
    } finally {
      sending = false;
    }
  }
</script>

<div class="bg-white rounded-xl shadow border border-gray-200 p-6 max-w-xl w-full">
  <div class="flex items-center justify-between mb-5">
    <h2 class="text-sm font-semibold text-gray-700">대국 수정 <span class="text-gray-400 font-normal">#{data.id}</span></h2>
    <span class="text-xs text-gray-400">{data.game.recorded_at}</span>
  </div>

  {#if error}
  <div class="mb-4 p-3 bg-red-50 border border-red-200 text-red-700 rounded-lg text-sm">{error}</div>
  {/if}

  <form onsubmit={submit} class="space-y-5">
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
                 required placeholder="이름"
                 class="flex-1 min-w-0 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
          <datalist id={listId}>
            {#each data.members as m}<option value={m}></option>{/each}
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
        <label for="edit-leftover" class="block text-sm font-medium text-gray-700 mb-1">잔여 공탁</label>
        <input id="edit-leftover" type="number" step="100" bind:value={leftover}
               class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
      </div>
      <div>
        <label for="edit-dora" class="block text-sm font-medium text-gray-700 mb-1">도라 수</label>
        <input id="edit-dora" type="number" bind:value={dora}
               class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
      </div>
    </div>

    <div>
      <label for="edit-remarks" class="block text-sm font-medium text-gray-700 mb-1">비고</label>
      <input id="edit-remarks" type="text" bind:value={remarks} placeholder="선택사항"
             class="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#43c1c3]" />
    </div>

    <div class="flex gap-3">
      <button type="submit" disabled={sending}
              class="flex-1 bg-[#001c54] text-white py-2.5 rounded-lg text-sm font-medium hover:bg-[#002880] transition-colors disabled:opacity-50">
        {sending ? '저장 중...' : '저장'}
      </button>
      <a href="/games"
         class="flex-1 border border-gray-300 text-gray-600 py-2.5 rounded-lg text-sm font-medium text-center hover:bg-gray-50 transition-colors">
        취소
      </a>
    </div>
    <button type="button" onclick={handleDelete} disabled={deleting}
            class="w-full border border-red-300 text-red-600 py-2.5 rounded-lg text-sm font-medium hover:bg-red-50 transition-colors disabled:opacity-50">
      {deleting ? '삭제 중...' : '대국 삭제'}
    </button>
  </form>
</div>
