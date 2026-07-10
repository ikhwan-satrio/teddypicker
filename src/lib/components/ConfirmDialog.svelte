<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		open = false,
		title,
		children,
		confirmLabel = 'Confirm',
		confirmClass = 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
		requireInput = false,
		inputLabel = 'Type yes to confirm',
		onconfirm,
		oncancel
	}: {
		open: boolean;
		title: string;
		children: Snippet;
		confirmLabel?: string;
		confirmClass?: string;
		requireInput?: boolean;
		inputLabel?: string;
		onconfirm: () => void;
		oncancel: () => void;
	} = $props();

	let inputValue = $state('');

	$effect(() => {
		if (!open) {
			inputValue = '';
		}
	});

	let canConfirm = $derived(!requireInput || inputValue === 'yes');
</script>

{#if open}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
		<div class="w-full max-w-sm rounded-lg border border-border bg-background p-6 shadow-lg">
			<h2 class="text-lg font-semibold text-foreground">{title}</h2>
			<div class="mt-2 text-sm text-muted-foreground break-words">
				{@render children()}
			</div>
			{#if requireInput}
				<div class="mt-4">
					<label for="confirm-input" class="mb-1 block text-xs text-muted-foreground">
						{inputLabel}
					</label>
					<input
						id="confirm-input"
						type="text"
						class="flex h-8 w-full rounded-md border border-border bg-transparent px-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
						placeholder="yes"
						bind:value={inputValue}
					/>
				</div>
			{/if}
			<div class="mt-4 flex justify-end gap-2">
				<button
					class="inline-flex h-8 items-center justify-center rounded-md border border-border bg-transparent px-3 text-xs font-medium text-foreground transition-colors hover:bg-accent"
					onclick={oncancel}
				>
					Cancel
				</button>
				<button
					class="inline-flex h-8 items-center justify-center rounded-md px-3 text-xs font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-40 {confirmClass}"
					disabled={!canConfirm}
					onclick={onconfirm}
				>
					{confirmLabel}
				</button>
			</div>
		</div>
	</div>
{/if}
