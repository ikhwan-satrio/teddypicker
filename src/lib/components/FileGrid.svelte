<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  import { navigation } from '$lib/stores/navigation.svelte';
  import { settings } from '$lib/stores/settings.svelte';
  import { dragStore } from '$lib/stores/drag.svelte';
  import { fileClipboard } from '$lib/stores/clipboard.svelte';
  import { getFileIcon, getFileColor } from '$lib/file-icons';
  import ContextMenu from './ContextMenu.svelte';
  import type { ContextMenuItem } from './ContextMenu.svelte';
  import OpenWithPopup from './OpenWithPopup.svelte';
  import InputPopup from './InputPopup.svelte';
  import {
    Loader2, FolderOpen, Pencil, Trash2, Copy, Scissors, Pin, PinOff, RotateCw,
    Image, FileVideo, FileAudio, FolderPlus, FilePlus, ClipboardPaste, CheckCheck,
  } from 'lucide-svelte';
  import type { DirEntryInfo, TrashEntry } from '$lib/types';

  let viewMode = $state<'grid' | 'list'>('grid');
  let selectedPaths = $state<Set<string>>(new Set());

  let ctxEntry = $state<DirEntryInfo | null>(null);
  let ctxIsEmpty = $state(false);
  let ctxX = $state(0);
  let ctxY = $state(0);
  let ctxOpen = $state(false);
  let contextMenuItems = $state<ContextMenuItem[]>([]);

  let openWithPath = $state('');
  let openWithOpen = $state(false);

  let renamePopup = $state({ open: false, path: '', name: '' });
  let createFolderPopup = $state({ open: false });
  let createFilePopup = $state({ open: false });

  let thumbnailUrls = $state<Map<string, string>>(new Map());

  const mediaExts = new Set([
    'png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp', 'ico', 'tiff', 'tif', 'avif',
    'mp4', 'avi', 'mov', 'mkv', 'webm', 'm4v', '3gp',
    'mp3', 'flac', 'ogg', 'aac', 'wav', 'm4a',
  ]);

  function isMedia(entry: DirEntryInfo) {
    return !entry.is_dir && entry.extension && mediaExts.has(entry.extension.toLowerCase());
  }

  function getMediaIcon(ext: string) {
    if (['mp4', 'avi', 'mov', 'mkv', 'webm', 'm4v', '3gp'].includes(ext)) return FileVideo;
    if (['mp3', 'flac', 'ogg', 'aac', 'wav', 'm4a'].includes(ext)) return FileAudio;
    return Image;
  }

  let pending = new Set<string>();

  async function loadThumbnail(path: string) {
    if (pending.has(path) || thumbnailUrls.has(path)) return;
    pending.add(path);
    try {
      const cachePath = await invoke<string>('app_get_thumbnail', { path });
      const bytes = await readFile(cachePath);
      const blob = new Blob([bytes], { type: 'image/jpeg' });
      const url = URL.createObjectURL(blob);
      const next = new Map(thumbnailUrls);
      next.set(path, url);
      thumbnailUrls = next;
    } catch {
      const next = new Map(thumbnailUrls);
      next.set(path, '');
      thumbnailUrls = next;
    } finally {
      pending.delete(path);
    }
  }

  function getThumbnail(entry: DirEntryInfo): string | undefined {
    const val = thumbnailUrls.get(entry.path);
    return val === '' ? undefined : val;
  }

  $effect(() => {
    navigation.entries;
    return () => {
      for (const url of thumbnailUrls.values()) {
        if (url && url.startsWith('blob:')) URL.revokeObjectURL(url);
      }
      thumbnailUrls = new Map();
      pending = new Set();
    };
  });

  $effect(() => {
    if (!settings.thumbnailEnabled) return;
    const entries = navigation.visibleEntries;
    const cancelled = { value: false };
    const mediaPaths: string[] = [];
    for (const e of entries) {
      if (isMedia(e) && !thumbnailUrls.has(e.path) && !pending.has(e.path)) {
        mediaPaths.push(e.path);
      }
    }

    async function loadAll() {
      const MAX = 8;
      async function worker(start: number) {
        for (let i = start; i < mediaPaths.length; i += MAX) {
          if (cancelled.value) return;
          await loadThumbnail(mediaPaths[i]);
        }
      }
      const workers: Promise<void>[] = [];
      for (let i = 0; i < Math.min(MAX, mediaPaths.length); i++) {
        workers.push(worker(i));
      }
      await Promise.all(workers);
    }

    if (mediaPaths.length > 0) {
      loadAll();
    }

    return () => { cancelled.value = true; };
  });

  function buildFileMenuItems(entry: DirEntryInfo): ContextMenuItem[] {
    const items: ContextMenuItem[] = [];

    if (entry.is_dir) {
      items.push({ label: 'Open', icon: FolderOpen, action: () => navigation.navigateTo(entry.path) });
    } else {
      items.push({
        label: 'Open With',
        icon: FolderOpen,
        action: () => {
          openWithPath = entry.path;
          openWithOpen = true;
        },
      });
    }

    if (entry.is_dir) {
      const pinned = navigation.isPinned(entry.path);
      items.push({
        label: pinned ? 'Unpin' : 'Pin to Sidebar',
        icon: pinned ? PinOff : Pin,
        action: async () => {
          if (pinned) {
            await navigation.removePinned(entry.path);
          } else {
            await navigation.addPinned(entry.name, entry.path);
          }
        },
      });
    }

    items.push({
      label: 'Copy',
      icon: Copy,
      action: () => fileClipboard.copy([entry.path]),
    });

    items.push({
      label: 'Cut',
      icon: Scissors,
      action: () => fileClipboard.cut([entry.path]),
    });

    items.push({
      label: 'Rename',
      icon: Pencil,
      action: () => {
        renamePopup = { open: true, path: entry.path, name: entry.name };
      },
    });

    items.push({
      label: 'Move to Trash',
      icon: Trash2,
      action: async () => {
        await invoke('app_delete_file', { path: entry.path });
        await navigation.refresh();
      },
    });

    items.push({ separator: true, label: '', action: () => {} });

    items.push({
      label: 'Copy Path',
      icon: Copy,
      action: () => writeText(entry.path),
    });

    return items;
  }

  function buildEmptyMenuItems(): ContextMenuItem[] {
    const items: ContextMenuItem[] = [];

    items.push({
      label: 'Create Folder',
      icon: FolderPlus,
      action: () => { createFolderPopup = { open: true }; },
    });

    items.push({
      label: 'Create File',
      icon: FilePlus,
      action: () => { createFilePopup = { open: true }; },
    });

    if (fileClipboard.hasItems) {
      items.push({ separator: true, label: '', action: () => {} });

      items.push({
        label: fileClipboard.operation === 'copy' ? 'Paste' : 'Paste (Move)',
        icon: ClipboardPaste,
        action: async () => {
          try {
            if (fileClipboard.operation === 'copy') {
              await invoke('app_copy_files', { sources: fileClipboard.items, dest: navigation.currentPath });
            } else {
              await invoke('app_move_files', { sources: fileClipboard.items, dest: navigation.currentPath });
            }
            fileClipboard.clear();
            await navigation.refresh();
          } catch (e) {
            alert(String(e));
          }
        },
      });
    }

    items.push({ separator: true, label: '', action: () => {} });

    items.push({
      label: 'Select All',
      icon: CheckCheck,
      action: () => {
        selectedPaths = new Set(navigation.visibleEntries.map((e) => e.path));
      },
    });

    return items;
  }

  function openContextMenu(e: MouseEvent, entry: DirEntryInfo | null) {
    e.preventDefault();
    if (ctxOpen) {
      ctxOpen = false;
      return;
    }
    ctxEntry = entry;
    ctxIsEmpty = entry === null;
    ctxX = e.clientX;
    ctxY = e.clientY;
    ctxOpen = true;

    if (entry) {
      contextMenuItems = buildFileMenuItems(entry);
    } else {
      contextMenuItems = buildEmptyMenuItems();
    }
  }

  function closeContextMenu() {
    ctxOpen = false;
    ctxEntry = null;
    ctxIsEmpty = false;
  }

  function handleDragStart(e: DragEvent, entry: DirEntryInfo) {
    if (!e.dataTransfer) return;
    const fileUri = `file://${entry.path}`;
    e.dataTransfer.setData('text/uri-list', fileUri);
    e.dataTransfer.setData('text/plain', entry.path);
    if (entry.is_dir) {
      dragStore.startDrag(entry.name, entry.path);
      e.dataTransfer.effectAllowed = 'copyLink';
    } else {
      e.dataTransfer.effectAllowed = 'copy';
    }
  }

  function handleDragEnd() {
    dragStore.endDrag();
  }

  function formatTime(ts: number): string {
    if (!ts) return '';
    const d = new Date(ts * 1000);
    return d.toLocaleDateString() + ' ' + d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }

  function handleWindowKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      if (ctxOpen) return; // context menu handles its own escape
      if (fileClipboard.hasItems || selectedPaths.size > 0) {
        selectedPaths = new Set();
        fileClipboard.clear();
      }
    }
  }

  function handleClick(entry: DirEntryInfo, e?: MouseEvent) {
    if (e?.ctrlKey || e?.metaKey) {
      const next = new Set(selectedPaths);
      if (next.has(entry.path)) next.delete(entry.path);
      else next.add(entry.path);
      selectedPaths = next;
    } else {
      selectedPaths = new Set([entry.path]);
    }
  }

  function handleDblClick(entry: DirEntryInfo) {
    if (entry.is_dir) {
      navigation.openEntry(entry);
    } else {
      invoke('app_open_default', { path: entry.path });
    }
  }
</script>

<OpenWithPopup
  path={openWithPath}
  open={openWithOpen}
  onclose={() => { openWithOpen = false; }}
/>

<InputPopup
  title="Rename"
  label="Enter a new name:"
  value={renamePopup.name}
  open={renamePopup.open}
  onclose={() => { renamePopup = { ...renamePopup, open: false }; }}
  onsubmit={async (newName) => {
    if (newName && newName !== renamePopup.name) {
      await invoke('app_rename_file', { oldPath: renamePopup.path, newName });
      await navigation.refresh();
    }
    renamePopup = { ...renamePopup, open: false };
  }}
/>

<InputPopup
  title="Create Folder"
  label="Enter folder name:"
  value="New Folder"
  open={createFolderPopup.open}
  onclose={() => { createFolderPopup = { open: false }; }}
  onsubmit={async (name) => {
    await invoke('app_create_dir', { parent: navigation.currentPath, name });
    await navigation.refresh();
    createFolderPopup = { open: false };
  }}
/>

<InputPopup
  title="Create File"
  label="Enter file name:"
  value="new-file.txt"
  open={createFilePopup.open}
  onclose={() => { createFilePopup = { open: false }; }}
  onsubmit={async (name) => {
    await invoke('app_create_file', { parent: navigation.currentPath, name });
    await navigation.refresh();
    createFilePopup = { open: false };
  }}
/>

<ContextMenu
  x={ctxX}
  y={ctxY}
  open={ctxOpen}
  items={contextMenuItems}
  onclose={closeContextMenu}
/>

<div class="flex h-full w-full select-none flex-col">
  <div class="flex shrink-0 items-center justify-between border-b border-border px-3 py-1">
    <span class="text-xs text-muted-foreground">
      {#if navigation.currentView === 'trash'}
        {navigation.trashEntries.length} items
      {:else}
        {navigation.visibleEntries.length} / {navigation.entries.length} items
      {/if}
    </span>
    <div class="flex items-center gap-1">
      <button
        class="inline-flex h-7 w-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-accent hover:text-foreground {viewMode === 'grid' ? 'bg-accent text-foreground' : ''}"
        onclick={() => (viewMode = 'grid')}
        title="Grid view"
      >
        <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
          <rect x="1" y="1" width="6" height="6" rx="1" />
          <rect x="9" y="1" width="6" height="6" rx="1" />
          <rect x="1" y="9" width="6" height="6" rx="1" />
          <rect x="9" y="9" width="6" height="6" rx="1" />
        </svg>
      </button>
      <button
        class="inline-flex h-7 w-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-accent hover:text-foreground {viewMode === 'list' ? 'bg-accent text-foreground' : ''}"
        onclick={() => (viewMode = 'list')}
        title="List view"
      >
        <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
          <rect x="1" y="2" width="14" height="2.5" rx="0.5" />
          <rect x="1" y="6.75" width="14" height="2.5" rx="0.5" />
          <rect x="1" y="11.5" width="14" height="2.5" rx="0.5" />
        </svg>
      </button>
    </div>
  </div>

  {#if navigation.isLoading}
    <div class="flex flex-1 items-center justify-center">
      <Loader2 class="h-6 w-6 animate-spin text-muted-foreground" />
    </div>
  {:else if navigation.error}
    <div class="flex flex-1 flex-col items-center justify-center gap-2">
      <p class="text-sm text-destructive">{navigation.error}</p>
      <button
        class="inline-flex h-8 items-center justify-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:bg-primary/90"
        onclick={() => navigation.refresh()}
      >
        Retry
      </button>
    </div>
  {:else if navigation.currentView === 'trash'}
    {#if navigation.trashEntries.length === 0}
      <div class="flex flex-1 items-center justify-center" role="none">
        <p class="text-sm text-muted-foreground">Trash is empty</p>
      </div>
    {:else if viewMode === 'grid'}
      <div class="flex min-h-0 flex-1 flex-wrap content-start gap-3 overflow-y-auto p-4" role="none">
        {#each navigation.trashEntries as entry (entry.id)}
          <div
            class="flex w-[160px] cursor-pointer flex-col items-center gap-2 rounded-lg border border-border bg-card p-3 text-center shadow-xs transition-all hover:bg-accent hover:text-accent-foreground active:scale-95 {selectedPaths.has(entry.id) ? 'bg-accent text-accent-foreground border-ring' : ''}"
            onclick={() => { selectedPaths = new Set([entry.id]); }}
            oncontextmenu={(e) => {
              e.preventDefault();
              e.stopPropagation();
              ctxEntry = null;
              ctxX = e.clientX;
              ctxY = e.clientY;
              ctxOpen = true;
              contextMenuItems = [
                { label: 'Restore', icon: RotateCw, action: () => navigation.restoreTrashItem(entry.id) },
                { label: 'Delete Permanently', icon: Trash2, action: () => navigation.purgeTrashItem(entry.id) },
              ];
            }}
          >
            <FolderOpen class="h-10 w-10 shrink-0 text-muted-foreground" />
            <span class="line-clamp-2 w-full break-all text-xs leading-tight text-muted-foreground">
              {entry.name}
            </span>
            <span class="truncate text-[0.6rem] text-muted-foreground/60" title={entry.original_path}>
              {entry.original_path}
            </span>
            {#if entry.time_deleted}
              <span class="text-[0.55rem] text-muted-foreground/50">{formatTime(entry.time_deleted)}</span>
            {/if}
          </div>
        {/each}
      </div>
    {:else}
      <div class="flex min-h-0 flex-1 flex-col overflow-y-auto" role="none">
        <div class="flex items-center border-b border-border px-4 py-1.5 text-[0.7rem] font-medium uppercase tracking-wider text-muted-foreground select-none">
          <span class="flex-1">Name</span>
          <span class="w-40 shrink-0 text-right">Deleted</span>
          <span class="w-6 shrink-0"></span>
        </div>
        {#each navigation.trashEntries as entry (entry.id)}
          <div
            class="flex w-full cursor-pointer items-center gap-3 border-b border-border bg-transparent px-4 py-2 text-left text-sm transition-all hover:bg-accent {selectedPaths.has(entry.id) ? 'bg-accent' : ''}"
            onclick={() => { selectedPaths = new Set([entry.id]); }}
            oncontextmenu={(e) => {
              e.preventDefault();
              e.stopPropagation();
              ctxEntry = null;
              ctxX = e.clientX;
              ctxY = e.clientY;
              ctxOpen = true;
              contextMenuItems = [
                { label: 'Restore', icon: RotateCw, action: () => navigation.restoreTrashItem(entry.id) },
                { label: 'Delete Permanently', icon: Trash2, action: () => navigation.purgeTrashItem(entry.id) },
              ];
            }}
          >
            <FolderOpen class="h-5 w-5 shrink-0 text-muted-foreground" />
            <div class="flex-1 min-w-0">
              <div class="truncate">{entry.name}</div>
              <div class="truncate text-[0.65rem] text-muted-foreground/60">{entry.original_path}</div>
            </div>
            <span class="w-40 shrink-0 text-right text-xs text-muted-foreground">
              {entry.time_deleted ? formatTime(entry.time_deleted) : ''}
            </span>
            <div class="flex w-6 shrink-0 items-center gap-1">
              <button
                class="flex items-center justify-center w-5 h-5 rounded text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent"
                onclick={(e) => { e.stopPropagation(); navigation.restoreTrashItem(entry.id); }}
                title="Restore"
              >
                <RotateCw class="h-3 w-3" />
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {:else if navigation.visibleEntries.length === 0}
    <div
      class="flex flex-1 items-center justify-center"
      role="none"
      oncontextmenu={(e) => openContextMenu(e, null)}
    >
      <p class="text-sm text-muted-foreground">Empty directory</p>
    </div>
  {:else if viewMode === 'grid'}
    <div
      class="flex min-h-0 flex-1 flex-wrap content-start gap-3 overflow-y-auto p-4"
      role="none"
      oncontextmenu={(e) => openContextMenu(e, null)}
    >
      {#each navigation.visibleEntries as entry (entry.path)}
        {@const Icon = getFileIcon(entry)}
        {@const thumb = getThumbnail(entry)}
        <button
          class="flex w-[120px] cursor-pointer flex-col items-center gap-2 rounded-lg border border-border bg-card p-3 text-center shadow-xs transition-all hover:bg-accent hover:text-accent-foreground active:scale-95 {selectedPaths.has(entry.path) ? 'bg-accent text-accent-foreground border-ring' : ''}"
          style="content-visibility: auto; contain-intrinsic-size: 120px 120px;"
          onclick={(e) => handleClick(entry, e)}
          ondblclick={() => handleDblClick(entry)}
          oncontextmenu={(e) => { e.stopPropagation(); openContextMenu(e, entry); }}
          draggable={true}
          ondragstart={(e) => handleDragStart(e, entry)}
          ondragend={handleDragEnd}
        >
          {#if thumb}
            <img
              src={thumb}
              alt={entry.name}
              class="h-[76px] w-[76px] shrink-0 rounded object-cover"
              loading="lazy"
            />
          {:else if isMedia(entry)}
            {@const MediaIcon = getMediaIcon(entry.extension?.toLowerCase() || '')}
            <MediaIcon class="h-10 w-10 shrink-0 text-muted-foreground" />
          {:else}
            <Icon class="h-10 w-10 shrink-0 {getFileColor(entry)}" />
          {/if}
          <span class="line-clamp-2 w-full break-all text-xs leading-tight text-muted-foreground">
            {entry.name}
          </span>
        </button>
      {/each}
    </div>
  {:else}
    <div
      class="flex min-h-0 flex-1 flex-col overflow-y-auto"
      role="none"
      oncontextmenu={(e) => openContextMenu(e, null)}
    >
      <div class="flex items-center border-b border-border px-4 py-1.5 text-[0.7rem] font-medium uppercase tracking-wider text-muted-foreground select-none">
        <span class="flex-1">Name</span>
        <span class="w-20 shrink-0 text-right">Type</span>
      </div>
      {#each navigation.visibleEntries as entry (entry.path)}
        {@const Icon = getFileIcon(entry)}
        {@const thumb = getThumbnail(entry)}
        <button
          class="flex w-full cursor-pointer items-center gap-3 border-b border-border bg-transparent px-4 py-2.5 text-left text-sm transition-all hover:bg-accent {selectedPaths.has(entry.path) ? 'bg-accent' : ''}"
          onclick={(e) => handleClick(entry, e)}
          ondblclick={() => handleDblClick(entry)}
          oncontextmenu={(e) => { e.stopPropagation(); openContextMenu(e, entry); }}
          draggable={true}
          ondragstart={(e) => handleDragStart(e, entry)}
          ondragend={handleDragEnd}
        >
          {#if thumb}
            <img src={thumb} alt={entry.name} class="h-8 w-8 shrink-0 rounded object-cover" loading="lazy" />
          {:else if isMedia(entry)}
            {@const MediaIcon = getMediaIcon(entry.extension?.toLowerCase() || '')}
            <MediaIcon class="h-5 w-5 shrink-0 text-muted-foreground" />
          {:else}
            <Icon class="h-5 w-5 shrink-0 {getFileColor(entry)}" />
          {/if}
          <span class="flex-1 truncate">{entry.name}</span>
          <span class="w-20 shrink-0 text-right text-xs text-muted-foreground">
            {entry.is_dir ? 'Folder' : entry.extension || 'File'}
          </span>
        </button>
      {/each}
    </div>
  {/if}
</div>

<svelte:window onkeydown={handleWindowKeydown} />