<script lang="ts">
  import type { WidgetConfig, SystemMetrics } from '../types';
  import { formatTemp } from '../stores/appState.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let { config, metrics, isWindow = false }: { config: WidgetConfig; metrics: SystemMetrics; isWindow?: boolean } = $props();

  let timeStr = $state(new Date().toLocaleTimeString());
  let dateStr = $state(new Date().toLocaleDateString(undefined, { month: 'short', day: 'numeric' }));

  $effect(() => {
    const timer = setInterval(() => {
      const now = new Date();
      timeStr = now.toLocaleTimeString();
      dateStr = now.toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
    }, 1000);
    return () => clearInterval(timer);
  });

  async function handleMouseDown(e: MouseEvent) {
    const targetTag = (e.target as HTMLElement)?.tagName?.toLowerCase();
    if (targetTag === 'input' || targetTag === 'button' || targetTag === 'select' || targetTag === 'option') {
      return;
    }

    if (e.button === 0) {
      try {
        await invoke('drag_window');
      } catch {
        try {
          const appWindow = getCurrentWindow();
          await appWindow.startDragging();
        } catch {
          // Dev web preview fallback
        }
      }
    }
  }

  async function handleWidgetClose(e: MouseEvent) {
    e.stopPropagation();
    try {
      await invoke('window_close');
    } catch {
      try {
        const appWindow = getCurrentWindow();
        await appWindow.close();
      } catch {
        // Dev preview fallback
      }
    }
  }

  // Calculate dimensions based on config
  let dimensions = $derived.by(() => {
    switch (config.widget_size) {
      case 'small': return { w: 200, h: 120 };
      case 'medium': return { w: 280, h: 180 };
      case 'large': return { w: 360, h: 260 };
      case 'custom': return { w: config.custom_width || 280, h: config.custom_height || 180 };
      default: return { w: 280, h: 180 };
    }
  });

  // Calculate border radius based on shape
  let borderRadius = $derived.by(() => {
    switch (config.shape) {
      case 'square': return '6px';
      case 'rounded': return '20px';
      case 'capsule': return '40px';
      default: return '20px';
    }
  });

  // Convert hex color + opacity into RGBA
  function hexToRgba(hex: string, alpha: number) {
    if (!hex) return `rgba(61, 35, 20, ${alpha})`;
    let c = hex.replace('#', '');
    if (c.length === 3) {
      c = c.split('').map(char => char + char).join('');
    }
    const num = parseInt(c, 16) || 0;
    const r = (num >> 16) & 255;
    const g = (num >> 8) & 255;
    const b = num & 255;
    return `rgba(${r}, ${g}, ${b}, ${alpha})`;
  }

  let computedBg = $derived(hexToRgba(config.bg_color || '#3D2314', config.widget_opacity ?? 0.9));
  let computedBorder = $derived(
    config.has_border
      ? `1.5px solid ${hexToRgba(config.border_color || '#8B5A2B', config.border_opacity ?? 0.8)}`
      : 'none'
  );
  let textColor = $derived(config.text_color || '#FFFFFF');
  let fontFamily = $derived(config.font_family || 'Montserrat');
</script>

<div
  class="widget-box {isWindow ? 'tauri-drag-region' : ''}"
  data-tauri-drag-region
  onmousedown={handleMouseDown}
  role="region"
  aria-label="Desktop Widget"
  style="
    width: {isWindow ? '100%' : dimensions.w + 'px'};
    height: {isWindow ? '100%' : dimensions.h + 'px'};
    background-color: {computedBg};
    border-radius: {borderRadius};
    border: {computedBorder};
    color: {textColor};
    font-family: '{fontFamily}', sans-serif;
    font-size: {config.font_size || 13}px;
    font-weight: {config.font_weight || '500'};
  "
>
  {#if config.bg_image}
    <div
      class="widget-bg-layer"
      data-tauri-drag-region
      style="
        background-image: url('{config.bg_image}');
        background-size: {config.bg_image_size || 'cover'};
        background-position: {config.bg_image_position || 'center'};
        background-repeat: no-repeat;
        opacity: {config.bg_image_opacity ?? 1.0};
        border-radius: {borderRadius};
      "
    ></div>
  {/if}

  <!-- Widget Header / Title Bar with Close Button -->
  <div class="widget-header flex items-center justify-between" data-tauri-drag-region>
    <span class="widget-title" style="color: {textColor};" data-tauri-drag-region>
      {config.title || 'Cacao Widget'}
    </span>
    <div class="flex items-center gap-1.5" data-tauri-drag-region>
      {#if config.show_time}
        <span class="widget-time" style="color: {textColor}; opacity: 0.9;" data-tauri-drag-region>
          {timeStr}
        </span>
      {/if}
      <button
        class="widget-close-btn"
        onclick={handleWidgetClose}
        title="Close Widget"
        style="color: {textColor};"
      >
        ✕
      </button>
    </div>
  </div>

  <!-- Widget Body Scrollable Grid -->
  <div class="widget-content">
    {#if config.show_cpu}
      <div class="widget-row flex items-center justify-between">
        <span class="row-label" style="color: {textColor};">CPU:</span>
        <span class="row-value" style="color: {textColor};">{formatTemp(metrics.cpu_temp_c, config.temp_unit)} | {metrics.cpu_usage.toFixed(1)}%</span>
      </div>
    {/if}

    {#if config.show_gpu}
      <div class="widget-row flex items-center justify-between">
        <span class="row-label" style="color: {textColor};">GPU:</span>
        <span class="row-value" style="color: {textColor};">
          {metrics.gpu_temp_c >= 0 ? formatTemp(metrics.gpu_temp_c, config.temp_unit) : 'N/A'} | {metrics.gpu_usage >= 0 ? `${metrics.gpu_usage.toFixed(1)}%` : 'Sin info'}
        </span>
      </div>
    {/if}

    {#if config.show_ram}
      <div class="widget-row flex items-center justify-between">
        <span class="row-label" style="color: {textColor};">RAM:</span>
        <span class="row-value" style="color: {textColor};">{formatTemp(metrics.ram_temp_c, config.temp_unit)} | {metrics.ram_usage_pct.toFixed(1)}%</span>
      </div>
    {/if}

    {#if config.show_disk}
      <div class="widget-row flex items-center justify-between">
        <span class="row-label" style="color: {textColor};">Disk:</span>
        <span class="row-value" style="color: {textColor};">{formatTemp(metrics.disk_temp_c, config.temp_unit)} | {metrics.disk_usage_pct.toFixed(1)}%</span>
      </div>
    {/if}

    {#if config.show_date}
      <div class="widget-row flex items-center justify-between">
        <span class="row-label" style="color: {textColor};">Date:</span>
        <span class="row-value" style="color: {textColor};">{dateStr}</span>
      </div>
    {/if}

    {#if config.show_process_count}
      <div class="widget-row flex items-center justify-between">
        <span class="row-label" style="color: {textColor};">Procs:</span>
        <span class="row-value" style="color: {textColor};">{metrics.process_count} running</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .widget-box {
    position: relative;
    padding: 0.5rem 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    box-sizing: border-box;
    overflow: hidden;
    user-select: none;
    cursor: grab;
  }

  .widget-bg-layer {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 0;
  }

  .widget-box:active {
    cursor: grabbing;
  }

  .tauri-drag-region {
    -webkit-app-region: drag;
  }

  .widget-header {
    position: relative;
    z-index: 1;
    border-bottom: 1px solid rgba(255, 255, 255, 0.15);
    padding-bottom: 0.25rem;
    flex-shrink: 0;
  }

  .widget-title {
    font-size: 0.9em;
    font-weight: 700;
    letter-spacing: -0.01em;
  }

  .widget-time {
    font-size: 0.85em;
    font-weight: 700;
  }

  .widget-close-btn {
    background: transparent;
    border: none;
    font-size: 0.85em;
    font-weight: 800;
    cursor: pointer;
    opacity: 0.75;
    padding: 0 5px;
    line-height: 1;
    border-radius: 4px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    -webkit-app-region: no-drag;
  }

  .widget-close-btn:hover {
    opacity: 1;
    background-color: var(--color-danger);
    color: #FFFFFF !important;
  }

  .widget-content {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
    justify-content: flex-start;
    overflow-y: auto;
    overflow-x: hidden;
    -webkit-app-region: no-drag;
    padding-right: 2px;
  }

  /* Custom Scrollbar for Widget Content */
  .widget-content::-webkit-scrollbar {
    width: 5px;
  }

  .widget-content::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
  }

  .widget-content::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.35);
    border-radius: 4px;
  }

  .widget-content::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.65);
  }

  .widget-row {
    background-color: rgba(255, 255, 255, 0.12);
    backdrop-filter: blur(4px);
    padding: 0.2rem 0.45rem;
    border-radius: 6px;
    font-size: 0.95em;
    flex-shrink: 0;
  }

  .row-label {
    opacity: 0.85;
    font-weight: 600;
  }

  .row-value {
    font-weight: 700;
  }
</style>
