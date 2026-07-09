<script lang="ts">
	import { navigation } from '$lib/stores/navigation.svelte';
	import { selection } from '$lib/stores/selection.svelte';
	import { handleItemClick, clearSelectionIfActive } from '$lib/composables/selection-utils';
	import { FolderOpen, RotateCw, Trash2, Undo2 } from 'lucide-svelte';
	import ConfirmDialog from './ConfirmDialog.svelte';

	let {
		viewMode,
		onTrashContextMenu
	}: {
		viewMode: 'grid' | 'list';
		onTrashContextMenu: (e: MouseEvent, entryId: string | null) => void;
	} = $props();

	let confirmRestoreAll = $state(false);
	let confirmClearAll = $state(false);

	function formatTime(ts: number): string {
		if (!ts) return '';
		const d = new Date(ts * 1000);
		return (
			d.toLocaleDateString() +
			' ' +
			d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
		);
	}

	function restoreAll() {
		const ids = navigation.trashEntries.map((e) => e.id);
		for (const id of ids) {
			navigation.restoreTrashItem(id);
		}
		selection.clear();
		confirmRestoreAll = false;
	}

	function clearAll() {
		navigation.emptyTrash();
		confirmClearAll = false;
	}
</script>

<ConfirmDialog
	open={confirmRestoreAll}
	title="Restore All"
	confirmLabel="Restore All"
	confirmClass="bg-primary text-primary-foreground hover:bg-primary/90"
	onconfirm={restoreAll}
	oncancel={() => (confirmRestoreAll = false)}
>
	Restore all {navigation.trashEntries.length} item(s) to their original locations?
</ConfirmDialog>

<ConfirmDialog
	open={confirmClearAll}
	title="Empty Trash"
	confirmLabel="Empty Trash"
	onconfirm={clearAll}
	oncancel={() => (confirmClearAll = false)}
>
	Permanently delete all {navigation.trashEntries.length} item(s)? This action cannot be undone.
</ConfirmDialog>

{#if navigation.trashEntries.length === 0}
	<div class="flex flex-1 items-center justify-center" role="none">
		<p class="text-sm text-muted-foreground">Trash is empty</p>
	</div>
{:else}
	<div class="flex shrink-0 items-center justify-end gap-2 border-b border-border px-4 py-2">
		{#if selection.paths.size > 0}
			<button
				class="inline-flex h-7 items-center gap-1.5 rounded-md border border-border bg-transparent px-2.5 text-xs font-medium text-foreground transition-colors hover:bg-accent"
				onclick={() => {
					const ids = Array.from(selection.paths);
					for (const id of ids) {
						navigation.restoreTrashItem(id);
					}
					selection.clear();
				}}
			>
				<Undo2 class="h-3.5 w-3.5" />
				Restore Selected
			</button>
		{/if}
		<button
			class="inline-flex h-7 items-center gap-1.5 rounded-md border border-border bg-transparent px-2.5 text-xs font-medium text-foreground transition-colors hover:bg-accent"
			onclick={() => (confirmRestoreAll = true)}
		>
			<Undo2 class="h-3.5 w-3.5" />
			Restore All
		</button>
		<button
			class="inline-flex h-7 items-center gap-1.5 rounded-md border border-destructive bg-transparent px-2.5 text-xs font-medium text-destructive transition-colors hover:bg-destructive/10"
			onclick={() => (confirmClearAll = true)}
		>
			<Trash2 class="h-3.5 w-3.5" />
			Empty Trash
		</button>
	</div>

	{#if viewMode === 'grid'}
		<div
			class="flex min-h-0 flex-1 flex-wrap content-start gap-3 overflow-y-auto p-4"
			role="none"
			onclick={clearSelectionIfActive}
			oncontextmenu={(e) => onTrashContextMenu(e, null)}
		>
			{#each navigation.trashEntries as entry (entry.id)}
				<button
					class="flex w-[160px] cursor-pointer flex-col items-center gap-2 rounded-lg border border-border bg-card p-3 text-center shadow-xs transition-all hover:bg-accent hover:text-accent-foreground active:scale-95 {selection.paths.has(
						entry.id
					)
						? 'bg-accent text-accent-foreground border-ring'
						: ''}"
					onclick={(e) => {
						e.stopPropagation();
						handleItemClick(entry.id, e);
					}}
					oncontextmenu={(e) => {
						e.stopPropagation();
						onTrashContextMenu(e, entry.id);
					}}
				>
					<FolderOpen class="h-10 w-10 shrink-0 text-muted-foreground" />
					<span class="line-clamp-2 w-full break-all text-xs leading-tight text-muted-foreground">
						{entry.name}
					</span>
					<span
						class="truncate text-[0.6rem] text-muted-foreground/60"
						title={entry.original_path}
					>
						{entry.original_path}
					</span>
					{#if entry.time_deleted}
						<span class="text-[0.55rem] text-muted-foreground/50"
							>{formatTime(entry.time_deleted)}</span
						>
					{/if}
				</button>
			{/each}
		</div>
	{:else}
		<div
			class="flex min-h-0 flex-1 flex-col overflow-y-auto"
			role="none"
			onclick={clearSelectionIfActive}
			oncontextmenu={(e) => onTrashContextMenu(e, null)}
		>
			<div
				class="flex items-center border-b border-border px-4 py-1.5 text-[0.7rem] font-medium uppercase tracking-wider text-muted-foreground select-none"
			>
				<span class="flex-1">Name</span>
				<span class="w-40 shrink-0 text-right">Deleted</span>
				<span class="w-6 shrink-0"></span>
			</div>
			{#each navigation.trashEntries as entry (entry.id)}
				<div
					role="button"
					tabindex="-1"
					class="flex w-full cursor-pointer items-center gap-3 border-b border-border bg-transparent px-4 py-2 text-left text-sm transition-all hover:bg-accent {selection.paths.has(
						entry.id
					)
						? 'bg-accent'
						: ''}"
					onclick={(e) => {
						e.stopPropagation();
						handleItemClick(entry.id, e);
					}}
					onkeydown={(e) => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.preventDefault();
							handleItemClick(entry.id, e);
						}
					}}
					oncontextmenu={(e) => {
						e.stopPropagation();
						onTrashContextMenu(e, entry.id);
					}}
				>
					<FolderOpen class="h-5 w-5 shrink-0 text-muted-foreground" />
					<div class="flex-1 min-w-0">
						<div class="truncate">{entry.name}</div>
						<div class="truncate text-[0.65rem] text-muted-foreground/60">
							{entry.original_path}
						</div>
					</div>
					<span class="w-40 shrink-0 text-right text-xs text-muted-foreground">
						{entry.time_deleted ? formatTime(entry.time_deleted) : ''}
					</span>
					<div class="flex w-6 shrink-0 items-center gap-1">
						<button
							class="flex items-center justify-center w-5 h-5 rounded text-muted-foreground hover:bg-accent hover:text-foreground cursor-pointer border-0 bg-transparent"
							onclick={(e) => {
								e.stopPropagation();
								navigation.restoreTrashItem(entry.id);
							}}
							title="Restore"
						>
							<RotateCw class="h-3 w-3" />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
{/if}
