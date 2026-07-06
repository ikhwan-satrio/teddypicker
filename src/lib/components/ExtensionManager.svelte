<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  type ExtEntry = {
    id: string;
    name: string;
    version: string;
    type: string;
    enabled: boolean;
  };

  let extensions = $state<ExtEntry[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  onMount(() => loadExtensions());

  async function loadExtensions() {
    try {
      isLoading = true;
      error = null;
      extensions = await invoke('app_get_extensions');
      isLoading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      isLoading = false;
    }
  }

  async function toggleExtension(id: string, enabled: boolean) {
    try {
      if (enabled) await invoke('app_enable_extension', { extensionId: id });
      else await invoke('app_disable_extension', { extensionId: id });
      await loadExtensions();
    } catch (e) {
      console.error('Failed to toggle extension:', e);
    }
  }

  async function reloadExtensions() {
    try {
      await invoke('app_reload_extensions');
      await loadExtensions();
    } catch (e) {
      console.error('Failed to reload extensions:', e);
    }
  }
</script>

<div class="p-4">
  <div class="flex items-center justify-between mb-4">
    <h2 class="text-lg font-semibold m-0">Extensions</h2>
    <button
      class="px-4 py-2 rounded-md bg-primary text-primary-foreground text-sm cursor-pointer border-0 enabled:hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
      disabled={isLoading}
      onclick={reloadExtensions}
    >
      {isLoading ? 'Loading...' : 'Reload'}
    </button>
  </div>

  {#if error}
    <div class="p-8 text-center text-destructive bg-destructive/10 rounded-md">{error}</div>
  {:else if isLoading}
    <div class="p-8 text-center text-muted-foreground">Loading extensions...</div>
  {:else if extensions.length === 0}
    <div class="p-8 text-center text-muted-foreground">No extensions installed</div>
  {:else}
    <div class="flex flex-col gap-3">
      {#each extensions as ext}
        <div class="flex items-center justify-between p-4 rounded-lg border border-border bg-card transition-opacity {ext.enabled ? '' : 'opacity-60'}">
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-medium text-foreground">{ext.name}</span>
              <span
                class="text-xs px-1.5 py-0.5 rounded font-medium {ext.type === 'theme'
                  ? 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300'
                  : 'bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300'}"
              >
                {ext.type}
              </span>
            </div>
            <div class="flex gap-3 text-xs text-muted-foreground">
              <span>v{ext.version}</span>
              <span>{ext.id}</span>
            </div>
          </div>
          <label class="relative inline-block w-11 h-6">
            <input
              type="checkbox"
              class="opacity-0 w-0 h-0 peer"
              checked={ext.enabled}
              onchange={(e) => toggleExtension(ext.id, (e.target as HTMLInputElement).checked)}
            />
            <span class="absolute cursor-pointer inset-0 bg-muted rounded-full transition-colors peer-checked:bg-primary before:absolute before:h-[18px] before:w-[18px] before:left-[3px] before:bottom-[3px] before:bg-white before:rounded-full before:transition-transform peer-checked:before:translate-x-5"></span>
          </label>
        </div>
      {/each}
    </div>
  {/if}
</div>
