<script lang="ts">
  import { appStore, defaultWidgetConfig } from '../stores/appState.svelte';
  import WidgetPreview from './WidgetPreview.svelte';
  import type { WidgetConfig } from '../types';

  let widgetConfig = $state<WidgetConfig>({ ...defaultWidgetConfig });

  $effect(() => {
    // Enable widget-mode transparent background
    document.documentElement.classList.add('widget-mode');
    document.body.classList.add('widget-mode');

    // Parse URL params for widget ID
    const urlParams = new URLSearchParams(window.location.hash.split('?')[1] || '');
    const id = urlParams.get('id');

    if (id) {
      const saved = appStore.savedWidgets.find(w => w.id === id);
      if (saved) {
        widgetConfig = saved;
      }
    }

    return () => {
      document.documentElement.classList.remove('widget-mode');
      document.body.classList.remove('widget-mode');
    };
  });
</script>

<div class="widget-window-wrapper" data-tauri-drag-region>
  <WidgetPreview config={widgetConfig} metrics={appStore.metrics} isWindow={true} />
</div>

<style>
  :global(html.widget-mode), :global(body.widget-mode), :global(#app.widget-mode) {
    background: transparent !important;
    background-color: transparent !important;
    margin: 0 !important;
    padding: 0 !important;
    overflow: hidden !important;
    width: 100vw;
    height: 100vh;
  }

  .widget-window-wrapper {
    width: 100vw;
    height: 100vh;
    background: transparent !important;
    background-color: transparent !important;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    box-sizing: border-box;
  }
</style>
