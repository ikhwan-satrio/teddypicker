<script lang="ts">
  import { toggleMode, mode } from 'mode-watcher';
  import { settings } from '$lib/stores/settings.svelte';
  import { Moon, Sun, Image, EyeOff } from 'lucide-svelte';

  let {
    open,
    onclose,
  }: {
    open: boolean;
    onclose: () => void;
  } = $props();

  $effect(() => {
    if (!open) return;

    function handleKeydown(e: KeyboardEvent) {
      if (e.key === 'Escape') onclose();
    }

    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (target.closest('.settings-popup')) return;
      onclose();
    };

    // Delay adding click listener so the current click that opened the popup
    // doesn't immediately trigger the outside-detect
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
    aria-label="Settings"
  >
    <div
      class="settings-popup min-w-[300px] rounded-xl border border-border bg-popover p-4 shadow-xl"
    >
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-semibold text-popover-foreground">Settings</h3>
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

      <div class="flex flex-col gap-3">
        <button
          class="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-xs text-popover-foreground transition-colors hover:bg-accent hover:text-accent-foreground cursor-pointer border-0 bg-transparent text-left"
          onclick={() => toggleMode()}
        >
          {#if mode.current === 'dark'}
            <Moon class="h-4 w-4 shrink-0" />
            <span class="flex-1">Dark Mode</span>
          {:else}
            <Sun class="h-4 w-4 shrink-0" />
            <span class="flex-1">Light Mode</span>
          {/if}
          <span class="text-[0.65rem] text-muted-foreground">{mode.current}</span>
        </button>

        <div class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-xs text-popover-foreground">
          <Image class="h-4 w-4 shrink-0" />
          <span class="flex-1">Show Thumbnails</span>
          <label class="relative inline-block w-9 h-5 cursor-pointer">
            <input
              type="checkbox"
              class="sr-only peer"
              checked={settings.thumbnailEnabled}
              onchange={(e) => settings.setThumbnailEnabled((e.target as HTMLInputElement).checked)}
            />
            <span class="absolute inset-0 rounded-full bg-muted transition-colors peer-checked:bg-primary after:absolute after:top-[2px] after:left-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-transform peer-checked:after:translate-x-4"></span>
          </label>
        </div>

        <div class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-xs text-popover-foreground">
          <EyeOff class="h-4 w-4 shrink-0" />
          <span class="flex-1">Show Hidden Files</span>
          <label class="relative inline-block w-9 h-5 cursor-pointer">
            <input
              type="checkbox"
              class="sr-only peer"
              checked={settings.showHiddenFiles}
              onchange={(e) => settings.setShowHiddenFiles((e.target as HTMLInputElement).checked)}
            />
            <span class="absolute inset-0 rounded-full bg-muted transition-colors peer-checked:bg-primary after:absolute after:top-[2px] after:left-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-transform peer-checked:after:translate-x-4"></span>
          </label>
        </div>
      </div>
    </div>
  </div>
{/if}
