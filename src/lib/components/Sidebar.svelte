<script lang="ts">
  import { navigation } from '$lib/stores/navigation.svelte';
  import { dragStore } from '$lib/stores/drag.svelte';
  import SettingsPopup from './SettingsPopup.svelte';
  import {
    Home,
    Monitor,
    FolderDown,
    FileText,
    Music,
    Image,
    Video,
    Pin,
    PinOff,
    Trash2,
    RotateCw,
    ChevronDown,
    ChevronRight,
    Puzzle,
    FolderOpen,
    Settings,
  } from 'lucide-svelte';

  let isXdgOpen = $state(true);
  let isPinnedOpen = $state(true);
  let isExtensionsOpen = $state(false);
  let dropTarget = $state(false);
  let settingsOpen = $state(false);

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer) e.dataTransfer.dropEffect = 'move';
    if (dragStore.isDragging) {
      dropTarget = true;
    }
  }

  function handleDragLeave() {
    dropTarget = false;
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    dropTarget = false;
    if (!dragStore.dragData) return;
    const { name, path } = dragStore.dragData;
    await navigation.addPinned(name, path);
    dragStore.endDrag();
  }

  function getXdgIcon(key: string) {
    switch (key) {
      case 'home': return Home;
      case 'desktop': return Monitor;
      case 'downloads': return FolderDown;
      case 'documents': return FileText;
      case 'music': return Music;
      case 'pictures': return Image;
      case 'videos': return Video;
      default: return FolderOpen;
    }
  }

  function getXdgLabel(key: string): string {
    return key.charAt(0).toUpperCase() + key.slice(1);
  }

  const xdgEntries = $derived(
    navigation.xdgDirs
      ? Object.entries(navigation.xdgDirs).filter(([key, val]) => key !== 'home' && val !== null)
      : [],
  );
</script>

<SettingsPopup
  open={settingsOpen}
  onclose={() => { settingsOpen = false; }}
/>

<div
  class="flex h-full w-full flex-col py-2 relative select-none {dropTarget ? 'bg-accent' : ''}"
  role="application"
  oncontextmenu={(e) => e.preventDefault()}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <!-- Home -->
  {#if navigation.xdgDirs?.home}
    {@const home = navigation.xdgDirs.home}
    <button
      class="flex w-full items-center gap-2 px-3 py-1.5 text-xs text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent text-left transition-colors {navigation.currentPath === home ? 'bg-accent text-foreground font-medium' : ''}"
      onclick={() => navigation.navigateTo(home)}
    >
      <Home class="h-4 w-4 shrink-0 text-amber-500" />
      <span>Home</span>
    </button>
  {/if}

  <!-- Quick Access -->
  <div class="mt-1">
    <button
      class="flex w-full items-center gap-1 px-3 py-1.5 text-[0.6875rem] font-semibold uppercase tracking-wider text-muted-foreground hover:text-foreground cursor-pointer border-0 bg-transparent transition-colors"
      onclick={() => (isXdgOpen = !isXdgOpen)}
    >
      {#if isXdgOpen}
        <ChevronDown class="h-3 w-3 shrink-0" />
      {:else}
        <ChevronRight class="h-3 w-3 shrink-0" />
      {/if}
      <span>Quick Access</span>
    </button>

    {#if isXdgOpen}
      <div class="flex flex-col gap-0.5">
        {#each xdgEntries as [key, path]}
          {@const Icon = getXdgIcon(key)}
          <button
            class="flex w-full items-center gap-2 px-3 py-1.5 text-xs text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent text-left transition-colors {navigation.currentPath === path ? 'bg-accent text-foreground font-medium' : ''}"
            onclick={() => navigation.navigateTo(path)}
          >
            <Icon class="h-4 w-4 shrink-0 text-blue-500" />
            <span>{getXdgLabel(key)}</span>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Trash -->
  <div class="mt-1">
    <div class="flex items-center group">
      <button
        class="flex flex-1 items-center gap-2 px-3 py-1.5 text-xs text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent text-left transition-colors {navigation.currentView === 'trash' ? 'bg-accent text-foreground font-medium' : ''}"
        onclick={() => navigation.navigateToTrash()}
      >
        <Trash2 class="h-4 w-4 shrink-0 text-red-500" />
        <span>Trash</span>
      </button>
      <button
        class="flex items-center justify-center w-6 h-6 mr-1 rounded opacity-0 group-hover:opacity-100 text-muted-foreground hover:bg-destructive hover:text-white cursor-pointer border-0 bg-transparent transition-all"
        onclick={() => navigation.emptyTrash()}
        title="Empty Trash"
      >
        <RotateCw class="h-3 w-3" />
      </button>
    </div>
  </div>

  <!-- Pinned -->
  <div class="mt-1">
    <button
      class="flex w-full items-center gap-1 px-3 py-1.5 text-[0.6875rem] font-semibold uppercase tracking-wider text-muted-foreground hover:text-foreground cursor-pointer border-0 bg-transparent transition-colors"
      onclick={() => (isPinnedOpen = !isPinnedOpen)}
    >
      {#if isPinnedOpen}
        <ChevronDown class="h-3 w-3 shrink-0" />
      {:else}
        <ChevronRight class="h-3 w-3 shrink-0" />
      {/if}
      <span>Pinned</span>
    </button>

    {#if isPinnedOpen}
      <div class="flex flex-col gap-0.5">
        {#if navigation.pinnedDirs.length === 0}
          <div class="flex items-center gap-2 px-3 py-1.5 text-xs text-muted-foreground/60 italic pl-7">
            <Pin class="h-3.5 w-3.5 shrink-0" />
            <span>Drag folders here to pin</span>
          </div>
        {:else}
          {#each navigation.pinnedDirs as dir (dir.path)}
            <div class="flex items-center group">
              <button
                class="flex flex-1 items-center gap-2 px-3 py-1.5 text-xs text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent text-left transition-colors min-w-0 {navigation.currentPath === dir.path ? 'bg-accent text-foreground font-medium' : ''}"
                onclick={() => navigation.navigateTo(dir.path)}
              >
                <FolderOpen class="h-4 w-4 shrink-0 text-emerald-500" />
                <span class="truncate">{dir.name}</span>
              </button>
              <button
                class="flex items-center justify-center w-6 h-6 mr-1 rounded opacity-0 group-hover:opacity-100 text-muted-foreground hover:bg-destructive hover:text-white cursor-pointer border-0 bg-transparent transition-all"
                onclick={() => navigation.removePinned(dir.path)}
                title="Unpin"
              >
                <PinOff class="h-3 w-3" />
              </button>
            </div>
          {/each}
        {/if}
      </div>
    {/if}
  </div>

  <!-- Extensions -->
  <div class="mt-1">
    <button
      class="flex w-full items-center gap-1 px-3 py-1.5 text-[0.6875rem] font-semibold uppercase tracking-wider text-muted-foreground hover:text-foreground cursor-pointer border-0 bg-transparent transition-colors"
      onclick={() => (isExtensionsOpen = !isExtensionsOpen)}
    >
      {#if isExtensionsOpen}
        <ChevronDown class="h-3 w-3 shrink-0" />
      {:else}
        <ChevronRight class="h-3 w-3 shrink-0" />
      {/if}
      <span>Extensions</span>
    </button>

    {#if isExtensionsOpen}
      <div class="flex flex-col gap-0.5">
        <div class="flex items-center gap-2 px-3 py-1.5 text-xs text-muted-foreground/60 italic pl-7">
          <Puzzle class="h-3.5 w-3.5 shrink-0" />
          <span>No extensions</span>
        </div>
      </div>
    {/if}
  </div>

  <!-- Settings -->
  <div class="mt-auto border-t border-border pt-1">
    <button
      class="flex w-full items-center gap-2 px-3 py-2 text-xs text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent transition-colors"
      onclick={() => (settingsOpen = true)}
    >
      <Settings class="h-4 w-4 shrink-0" />
      <span>Settings</span>
    </button>
  </div>

  <!-- Drop overlay -->
  {#if dropTarget}
    <div class="absolute inset-0 flex flex-col items-center justify-center gap-2 bg-primary text-primary-foreground opacity-90 text-xs font-medium pointer-events-none rounded-lg m-1">
      <Pin class="h-5 w-5" />
      <span>Drop to pin</span>
    </div>
  {/if}
</div>
