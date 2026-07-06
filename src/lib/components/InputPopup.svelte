<script lang="ts">
  let {
    title,
    label,
    value = '',
    placeholder = '',
    open,
    onclose,
    onsubmit,
  }: {
    title: string;
    label?: string;
    value?: string;
    placeholder?: string;
    open: boolean;
    onclose: () => void;
    onsubmit: (value: string) => void;
  } = $props();

  let inputValue = $state('');
  let inputEl: HTMLInputElement | undefined = $state(undefined);

  $effect(() => {
    if (!open) return;
    inputValue = value;
    requestAnimationFrame(() => {
      inputEl?.focus();
      inputEl?.select();
    });
  });

  $effect(() => {
    if (!open) return;

    function handleKeydown(e: KeyboardEvent) {
      if (e.key === 'Escape') onclose();
      if (e.key === 'Enter' && inputValue.trim()) {
        onsubmit(inputValue.trim());
      }
    }

    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (target.closest('.input-popup')) return;
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
    aria-label={title}
  >
    <div class="input-popup min-w-[320px] max-w-[400px] rounded-xl border border-border bg-popover p-4 shadow-xl">
      <div class="flex items-center justify-between mb-3">
        <h3 class="text-sm font-semibold text-popover-foreground">{title}</h3>
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

      {#if label}
        <label for="input-popup-field" class="mb-2 block text-xs text-muted-foreground">{label}</label>
      {/if}

      <input
        id="input-popup-field"
        bind:this={inputEl}
        type="text"
        bind:value={inputValue}
        {placeholder}
        class="w-full rounded-lg border border-border bg-background px-3 py-2 text-xs text-foreground outline-none focus:border-ring focus:ring-1 focus:ring-ring"
        onkeydown={(e) => {
          if (e.key === 'Enter' && inputValue.trim()) {
            onsubmit(inputValue.trim());
          }
        }}
      />

      <div class="mt-4 flex justify-end gap-2">
        <button
          class="rounded-lg px-3 py-1.5 text-xs text-muted-foreground transition-colors hover:bg-accent hover:text-foreground cursor-pointer border border-border bg-transparent"
          onclick={onclose}
        >
          Cancel
        </button>
        <button
          class="rounded-lg px-3 py-1.5 text-xs font-medium text-primary-foreground transition-opacity hover:opacity-90 cursor-pointer border-0 bg-primary disabled:opacity-40 disabled:cursor-default"
          disabled={!inputValue.trim()}
          onclick={() => onsubmit(inputValue.trim())}
        >
          Confirm
        </button>
      </div>
    </div>
  </div>
{/if}
