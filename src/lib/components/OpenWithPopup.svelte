<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Loader2 } from 'lucide-svelte';

  let {
    path,
    open,
    onclose,
  }: {
    path: string;
    open: boolean;
    onclose: () => void;
  } = $props();

  let apps = $state<Array<{ name: string; exec: string; path: string }>>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function loadApps() {
    loading = true;
    error = null;
    try {
      apps = await invoke('app_get_open_with_apps', { path });
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
    loading = false;
  }

  async function openWithApp(execTemplate: string) {
    try {
      await invoke('app_open_with_file', { path, execTemplate });
      onclose();
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }

  $effect(() => {
    if (!open) return;

    loadApps();

    const handleKeydown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onclose();
    };

    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (target.closest('.open-with-popup')) return;
      onclose();
    };

    const raf = requestAnimationFrame(() => {
      document.addEventListener('keydown', handleKeydown);
      document.addEventListener('click', handleClickOutside);
    });

    return () => {
      cancelAnimationFrame(raf);
      document.removeEventListener('keydown', handleKeydown);
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/30"
    role="dialog"
    aria-label="Open with"
  >
    <div
      class="open-with-popup min-w-[320px] max-w-[400px] rounded-xl border border-border bg-popover p-4 shadow-xl"
    >
      <div class="flex items-center justify-between mb-3">
        <h3 class="text-sm font-semibold text-popover-foreground">Open With</h3>
        <button
          class="flex items-center justify-center w-6 h-6 rounded-md text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent transition-colors"
          onclick={onclose}
          aria-label="Close"
        >
          <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
          </svg>
        </button>
      </div>

      <p class="text-xs text-muted-foreground mb-3 truncate">{path}</p>

      {#if loading}
        <div class="flex items-center justify-center py-8">
          <Loader2 class="h-5 w-5 animate-spin text-muted-foreground" />
        </div>
      {:else if error}
        <div class="py-4 text-center text-xs text-destructive">{error}</div>
      {:else if apps.length === 0}
        <div class="py-4 text-center text-xs text-muted-foreground">No applications found</div>
      {:else}
        <div class="flex max-h-[300px] flex-col gap-0.5 overflow-y-auto">
          {#each apps as app}
            <button
              class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-xs text-popover-foreground transition-colors hover:bg-accent hover:text-accent-foreground cursor-pointer border-0 bg-transparent text-left"
              onclick={() => openWithApp(app.path)}
            >
              <span class="flex h-8 w-8 items-center justify-center rounded-md bg-muted text-xs font-medium uppercase text-muted-foreground">
                {app.name.charAt(0)}
              </span>
              <div class="flex-1 min-w-0">
                <div class="font-medium truncate">{app.name}</div>
                <div class="truncate text-[0.65rem] text-muted-foreground">{app.exec}</div>
              </div>
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}
