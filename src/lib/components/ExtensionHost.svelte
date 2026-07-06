<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { buildSrcdoc } from '$lib/extensions';

  let { pluginId, onMessage } = $props();

  let iframe: HTMLIFrameElement | undefined = $state(undefined);
  let pluginCode = $state<string>('');
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  const allowedActions = [
    'readFile',
    'writeFile',
    'listDir',
    'getSelectedFiles',
    'showToast',
    'previewFile'
  ];

  let srcdoc = $derived(buildSrcdoc(pluginCode, pluginId, allowedActions));

  onMount(() => loadPlugin());
  onDestroy(() => { if (iframe) iframe.src = 'about:blank'; });

  async function loadPlugin() {
    try {
      isLoading = true;
      error = null;
      pluginCode = await invoke<string>('app_get_plugin_js', { pluginId });
      isLoading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
      isLoading = false;
    }
  }

  function handleMessage(event: MessageEvent) {
    if (event.data.type === 'tauri-invoke') {
      handleTauriInvoke(event);
    } else if (event.data.type === 'show-toast') {
      onMessage?.({ type: 'toast', message: event.data.message });
    } else if (event.data.type === 'plugin-error') {
      console.error('Plugin error:', event.data.error);
    }
  }

  async function handleTauriInvoke(event: MessageEvent) {
    const { command, args, requestId } = event.data;
    try {
      const result = await invoke(command, args);
      iframe?.contentWindow?.postMessage({ requestId, result }, '*');
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      iframe?.contentWindow?.postMessage({ requestId, error: errorMessage }, '*');
    }
  }
</script>

<div class="extension-host">
  {#if isLoading}
    <div class="loading">Loading plugin...</div>
  {:else if error}
    <div class="error">
      <p>Failed to load plugin: {error}</p>
      <button onclick={loadPlugin}>Retry</button>
    </div>
  {:else}
    <iframe
      bind:this={iframe}
      srcdoc={srcdoc}
      sandbox="allow-scripts"
      onmessage={handleMessage}
      title="Plugin sandbox"
    ></iframe>
  {/if}
</div>

<style>
  .extension-host {
    width: 100%;
    height: 100%;
    position: relative;
  }

  .extension-host iframe {
    width: 100%;
    height: 100%;
    border: none;
  }

  .loading,
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 1rem;
    text-align: center;
  }

  .error {
    color: #ef4444;
  }

  .error button {
    margin-top: 0.5rem;
    padding: 0.5rem 1rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
  }

  .error button:hover {
    background: #2563eb;
  }
</style>
