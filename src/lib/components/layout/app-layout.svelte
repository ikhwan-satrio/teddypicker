<script lang="ts">
  import AppTitlebar from './app-titlebar.svelte';
  import type { Snippet } from 'svelte';

  let {
    sidebar,
    children,
  }: {
    sidebar?: Snippet;
    children?: Snippet;
  } = $props();
</script>

<div class="flex h-screen w-screen flex-col overflow-hidden bg-background text-foreground">
  <AppTitlebar />

  <div class="flex flex-1 overflow-hidden">
    <div
      class="flex w-[220px] min-w-[180px] max-w-[300px] shrink-0 flex-col overflow-y-auto border-r border-border bg-sidebar text-sidebar-foreground"
    >
      {#if sidebar}
        {@render sidebar()}
      {/if}
    </div>

    <main class="flex min-h-0 flex-1 flex-col overflow-hidden">
      {#if children}
        {@render children()}
      {/if}
    </main>
  </div>
</div>
