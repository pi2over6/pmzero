<script lang="ts">
  import '../app.css';
  import { navigating } from '$app/stores';
  import type { LayoutData } from './$types';

  let { data, children }: { data: LayoutData, children: any } = $props();

  let mobileOpen = $state(false);

  const nav = [
    { href: '/ranking',    label: '순위' },
    { href: '/new-game',   label: '대국 입력' },
    { href: '/games',      label: '대국 기록' },
    { href: '/stats',      label: '통계' },
  ];

  import { page } from '$app/stores';

  function isActive(href: string) {
    return $page.url.pathname === href || $page.url.pathname.startsWith(href + '/');
  }

  const pageTitle = $derived(
    nav.find(item => isActive(item.href))?.label ?? '±0 마작'
  );
</script>

<div class="min-h-screen bg-gray-50 flex">
  {#if data.showNav}
  <!-- Sidebar (desktop) -->
  <aside class="hidden md:flex flex-col w-52 bg-[#001c54] text-white shrink-0">
    <div class="p-5 border-b border-white/10">
      <span class="text-xl font-bold tracking-tight">±0 마작</span>
    </div>
    <nav class="flex-1 py-4 space-y-1 px-3">
      {#each nav as item}
      <a href={item.href}
         class="flex items-center px-3 py-2 rounded-lg text-sm transition-colors
                {isActive(item.href)
                  ? 'bg-[#43c1c3] text-white font-semibold'
                  : 'text-white/70 hover:bg-white/10 hover:text-white'}">
        {item.label}
      </a>
      {/each}
    </nav>
  </aside>

  <!-- Mobile topbar -->
  <div class="md:hidden fixed top-0 inset-x-0 z-30 bg-[#001c54] text-white flex items-center justify-between px-4 h-14 shadow">
    <span class="text-lg font-bold">{pageTitle}</span>
    <button onclick={() => mobileOpen = !mobileOpen}
            class="p-2 rounded-lg hover:bg-white/10 transition-colors" aria-label="메뉴">
      {#if mobileOpen}
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
      </svg>
      {:else}
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
      </svg>
      {/if}
    </button>
  </div>

  <!-- Mobile drawer -->
  {#if mobileOpen}
  <button class="md:hidden fixed inset-0 z-20 bg-black/40 cursor-default border-0 p-0"
          aria-label="메뉴 닫기"
          onclick={() => mobileOpen = false}></button>
  <div class="md:hidden fixed top-14 right-0 bottom-0 z-20 w-52 bg-[#001c54] text-white py-4 space-y-1 px-3 overflow-y-auto">
    {#each nav as item}
    <a href={item.href} onclick={() => mobileOpen = false}
       class="flex items-center px-3 py-2 rounded-lg text-sm transition-colors
              {isActive(item.href)
                ? 'bg-[#43c1c3] text-white font-semibold'
                : 'text-white/70 hover:bg-white/10 hover:text-white'}">
      {item.label}
    </a>
    {/each}
  </div>
  {/if}

  {/if}

  <!-- Page loading overlay -->
  {#if $navigating}
  <div class="fixed inset-0 z-50 bg-white/60 flex items-center justify-center pointer-events-none">
    <div class="flex flex-col items-center gap-3">
      <svg class="w-8 h-8 animate-spin text-[#43c1c3]" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"/>
      </svg>
      <span class="text-sm text-[#001c54] font-medium">불러오는 중...</span>
    </div>
  </div>
  {/if}

  <!-- Main content -->
  <main class="flex-1 p-4 sm:p-6 md:p-8 {data.showNav ? 'pt-18 md:pt-6' : ''} min-w-0">
    {@render children()}
  </main>
</div>
