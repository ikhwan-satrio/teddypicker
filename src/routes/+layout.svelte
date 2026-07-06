<script lang="ts">
  import "./layout.css"
  import { onMount } from 'svelte';
  import AppLayout from '$lib/components/layout/app-layout.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { navigation } from '$lib/stores/navigation.svelte';
	import { ModeWatcher } from "mode-watcher";

  let { children } = $props();

  onMount(() => {
    navigation.init();
  });
</script>

<AppLayout>
  {#snippet sidebar()}
    <Sidebar />
  {/snippet}

  {@render children()}
</AppLayout>
<ModeWatcher/>
