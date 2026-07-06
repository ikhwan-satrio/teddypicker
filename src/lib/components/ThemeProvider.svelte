<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let { children } = $props();

  let activeTheme = $state<string | null>(null);
  let themeStyles = $state<string>('');

  onMount(async () => {
    await loadThemes();
  });

  async function loadThemes() {
    try {
      const themes = await invoke<Array<{id: string, name: string}>>('get_themes');
      if (themes.length > 0) {
        await setTheme(themes[0].id);
      }
    } catch (error) {
      console.error('Failed to load themes:', error);
    }
  }

  async function setTheme(themeId: string) {
    try {
      const css = await invoke<string>('get_theme_css', { themeId });
      activeTheme = themeId;
      themeStyles = css;
    } catch (error) {
      console.error('Failed to load theme:', error);
    }
  }

  async function refreshThemes() {
    await loadThemes();
  }

  export { setTheme, refreshThemes };
</script>

{@html `<style>${themeStyles}</style>`}

{@render children()}
