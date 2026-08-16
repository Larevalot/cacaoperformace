<script lang="ts">
  import { appStore } from './lib/stores/appState.svelte';
  import Header from './lib/components/Header.svelte';
  import MetricsDashboard from './lib/components/MetricsDashboard.svelte';
  import ProcessManager from './lib/components/ProcessManager.svelte';
  import WidgetCreator from './lib/components/WidgetCreator.svelte';
  import WidgetView from './lib/components/WidgetView.svelte';
  import type { SystemMetrics } from './lib/types';
  import { invoke } from '@tauri-apps/api/core';

  let isWidgetWindow = $state(false);

  $effect(() => {
    // Check if running as standalone widget window
    if (window.location.hash.includes('#/widget')) {
      isWidgetWindow = true;
    }

    // Telemetry Fast Polling Loop (1000ms) for CPU, GPU, RAM, Disk & Process count
    const pollMetrics = async () => {
      try {
        const sysMetrics = await invoke<SystemMetrics>('get_system_metrics');
        appStore.metrics = sysMetrics;
      } catch {
        // Dev fallback simulation if Tauri IPC isn't active
        simulateMetrics();
      }
    };

    pollMetrics();
    const interval = setInterval(pollMetrics, 1000);

    return () => clearInterval(interval);
  });

  function simulateMetrics() {
    const cpu = 15 + Math.random() * 25;
    appStore.metrics = {
      cpu_usage: parseFloat(cpu.toFixed(1)),
      cpu_temp_c: parseFloat((42 + (cpu * 0.3)).toFixed(1)),
      gpu_usage: -1.0,
      gpu_temp_c: -1.0,
      ram_usage_pct: 44.2,
      ram_used_mb: 7240,
      ram_total_mb: 16384,
      ram_temp_c: 36.5,
      disk_usage_pct: 58.4,
      disk_used_gb: 299.1,
      disk_total_gb: 512.0,
      disk_temp_c: 38.8,
      process_count: 186
    };
  }
</script>

{#if isWidgetWindow}
  <WidgetView />
{:else}
  <div class="app-layout">
    <Header />

    <main class="main-content">
      {#if appStore.activeTab === 'dashboard'}
        <MetricsDashboard />
      {:else if appStore.activeTab === 'processes'}
        <ProcessManager />
      {:else if appStore.activeTab === 'widget_studio'}
        <WidgetCreator />
      {/if}
    </main>
  </div>
{/if}

<style>
  .app-layout {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
    background-color: var(--color-bg-primary);
    overflow: hidden;
    border-radius: 16px;
    border: 1px solid var(--color-border);
    box-shadow: var(--shadow-lg);
    box-sizing: border-box;
  }

  .main-content {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }
</style>
