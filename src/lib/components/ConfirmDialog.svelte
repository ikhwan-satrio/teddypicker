<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		open = false,
		title,
		children,
		confirmLabel = 'Confirm',
		confirmClass = 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
		onconfirm,
		oncancel
	}: {
		open: boolean;
		title: string;
		children: Snippet;
		confirmLabel?: string;
		confirmClass?: string;
		onconfirm: () => void;
		oncancel: () => void;
	} = $props();
</script>

{#if open}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
		<div class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-lg">
			<h2 class="text-lg font-semibold text-foreground">{title}</h2>
			<div class="mt-2 text-sm text-muted-foreground">
				{@render children()}
			</div>
			<div class="mt-4 flex justify-end gap-2">
				<button
					class="inline-flex h-8 items-center justify-center rounded-md border border-border bg-transparent px-3 text-xs font-medium text-foreground transition-colors hover:bg-accent"
					onclick={oncancel}
				>
					Cancel
				</button>
				<button
					class="inline-flex h-8 items-center justify-center rounded-md px-3 text-xs font-medium transition-colors {confirmClass}"
					onclick={onconfirm}
				>
					{confirmLabel}
				</button>
			</div>
		</div>
	</div>
{/if}
