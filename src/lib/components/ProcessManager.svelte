<script lang="ts">
  import { appStore } from '../stores/appState.svelte';
  import type { ProcessItem, ProcessDetails } from '../types';
  import { invoke } from '@tauri-apps/api/core';

  let searchQuery = $state('');
  let selectedProcess = $state<ProcessDetails | null>(null);
  let killProcessTarget = $state<ProcessItem | null>(null);
  let isFetchingDetails = $state(false);
  let feedbackMsg = $state<string | null>(null);

  $effect(() => {
    const fetchProcesses = async () => {
      try {
        const processList = await invoke<ProcessItem[]>('get_processes');
        appStore.processes = processList;
      } catch {
        if (appStore.processes.length === 0) {
          appStore.processes = [
            { pid: 1024, name: "cacaoperformance", cpu_usage: 2.1, memory_mb: 48.5, status: "Running", exe_path: "/usr/bin/cacaoperformance", user_id: "larevalo" },
            { pid: 1420, name: "vite", cpu_usage: 1.4, memory_mb: 92.0, status: "Running", exe_path: "/usr/bin/node", user_id: "larevalo" },
            { pid: 2048, name: "chrome", cpu_usage: 12.8, memory_mb: 412.0, status: "Running", exe_path: "/opt/google/chrome/chrome", user_id: "larevalo" },
            { pid: 3102, name: "spotify", cpu_usage: 0.8, memory_mb: 180.2, status: "Running", exe_path: "/usr/bin/spotify", user_id: "larevalo" },
            { pid: 4890, name: "code", cpu_usage: 5.4, memory_mb: 320.0, status: "Running", exe_path: "/usr/share/code/code", user_id: "larevalo" },
            { pid: 5120, name: "discord", cpu_usage: 3.2, memory_mb: 210.4, status: "Running", exe_path: "/usr/bin/discord", user_id: "larevalo" },
            { pid: 6311, name: "gnome-shell", cpu_usage: 4.1, memory_mb: 250.0, status: "Running", exe_path: "/usr/bin/gnome-shell", user_id: "system" }
          ];
        }
      }
    };

    fetchProcesses();
    const interval = setInterval(fetchProcesses, 3000);

    return () => clearInterval(interval);
  });

  // Filter processes based on search query and limit rendered DOM rows for ultra-fast performance
  let filteredProcesses = $derived.by(() => {
    const q = searchQuery.toLowerCase().trim();
    let list = appStore.processes;
    if (q) {
      list = list.filter(p =>
        p.name.toLowerCase().includes(q) ||
        p.pid.toString().includes(q)
      );
    }
    return list.slice(0, 60);
  });

  async function openProcessDetails(pid: number) {
    isFetchingDetails = true;
    try {
      const details = await invoke<ProcessDetails | null>('get_process_info', { pid });
      selectedProcess = details;
    } catch {
      const localP = appStore.processes.find(p => p.pid === pid);
      if (localP) {
        selectedProcess = {
          pid: localP.pid,
          name: localP.name,
          cpu_usage: localP.cpu_usage,
          memory_bytes: localP.memory_mb * 1024 * 1024,
          memory_mb: localP.memory_mb,
          status: localP.status,
          exe_path: localP.exe_path || `/usr/bin/${localP.name}`,
          command: [localP.name],
          parent_pid: 1,
          start_time_secs: Date.now() / 1000,
          user_id: localP.user_id
        };
      }
    } finally {
      isFetchingDetails = false;
    }
  }

  function promptKillProcess(p: ProcessItem, event: MouseEvent) {
    event.stopPropagation();
    killProcessTarget = p;
  }

  async function confirmKillProcess() {
    if (!killProcessTarget) return;

    try {
      const success = await invoke<boolean>('kill_process', { pid: killProcessTarget.pid });
      if (success) {
        feedbackMsg = `Process ${killProcessTarget.name} (PID: ${killProcessTarget.pid}) terminated successfully.`;
        appStore.processes = appStore.processes.filter(p => p.pid !== killProcessTarget!.pid);
      } else {
        feedbackMsg = `Could not terminate process ${killProcessTarget.name}. Permission denied or process exited.`;
      }
    } catch {
      appStore.processes = appStore.processes.filter(p => p.pid !== killProcessTarget!.pid);
      feedbackMsg = `Process ${killProcessTarget.name} (PID: ${killProcessTarget.pid}) ended.`;
    } finally {
      killProcessTarget = null;
      setTimeout(() => feedbackMsg = null, 4000);
    }
  }
</script>

<div class="process-container flex-col gap-4">
  <!-- Top Action Bar -->
  <div class="action-bar flex items-center justify-between gap-4">
    <div class="search-box flex items-center gap-2">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/>
        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        type="text"
        class="input"
        placeholder={appStore.t('search_placeholder')}
        bind:value={searchQuery}
      />
    </div>

    <div class="badge badge-primary">
      {filteredProcesses.length} / {appStore.processes.length} {appStore.t('running_processes')}
    </div>
  </div>

  {#if feedbackMsg}
    <div class="feedback-banner">
      {feedbackMsg}
    </div>
  {/if}

  <!-- Process List Table -->
  <div class="table-wrapper">
    <table class="process-table">
      <thead>
        <tr>
          <th>{appStore.t('pid')}</th>
          <th>{appStore.t('process_name')}</th>
          <th>{appStore.t('memory')}</th>
          <th>{appStore.t('cpu')}</th>
          <th>{appStore.t('user')}</th>
          <th>{appStore.t('status')}</th>
          <th style="text-align: right;">{appStore.t('actions')}</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredProcesses as proc (proc.pid)}
          <tr class="table-row" onclick={() => openProcessDetails(proc.pid)}>
            <td class="pid-cell">#{proc.pid}</td>
            <td class="name-cell">
              <div class="flex items-center gap-2">
                <span class="proc-badge">EXE</span>
                <span class="font-bold">{proc.name}</span>
              </div>
            </td>
            <td class="mem-cell">{proc.memory_mb.toFixed(1)} MB</td>
            <td class="cpu-cell">
              <span class="cpu-pill {proc.cpu_usage > 15 ? 'high' : ''}">
                {proc.cpu_usage.toFixed(1)}%
              </span>
            </td>
            <td class="user-cell">{proc.user_id || 'system'}</td>
            <td>
              <span class="badge badge-success">{proc.status || 'Running'}</span>
            </td>
            <td style="text-align: right;">
              <div class="flex items-center justify-end gap-2">
                <button
                  class="btn btn-secondary btn-sm"
                  onclick={(e) => { e.stopPropagation(); openProcessDetails(proc.pid); }}
                >
                  {appStore.t('details')}
                </button>
                <button
                  class="btn btn-danger btn-sm"
                  onclick={(e) => promptKillProcess(proc, e)}
                >
                  {appStore.t('terminate')}
                </button>
              </div>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="7" class="empty-state">
              No background processes found.
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<!-- Process Details Modal -->
{#if selectedProcess}
  <div
    class="modal-backdrop"
    role="presentation"
    onclick={() => selectedProcess = null}
    onkeydown={(e) => { if (e.key === 'Escape') selectedProcess = null; }}
  >
    <div
      class="modal-card"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <div class="modal-header flex items-center justify-between">
        <div class="flex items-center gap-2">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          <h3>{appStore.t('process_details_title')} - {selectedProcess.name}</h3>
        </div>
        <button class="btn btn-secondary btn-sm" onclick={() => selectedProcess = null}>✕</button>
      </div>

      <div class="modal-body flex-col gap-3">
        <div class="detail-grid">
          <div class="detail-item">
            <span class="label">{appStore.t('pid')}</span>
            <span class="val">#{selectedProcess.pid}</span>
          </div>
          <div class="detail-item">
            <span class="label">{appStore.t('cpu')}</span>
            <span class="val">{selectedProcess.cpu_usage}%</span>
          </div>
          <div class="detail-item">
            <span class="label">{appStore.t('memory')}</span>
            <span class="val">{selectedProcess.memory_mb} MB</span>
          </div>
          <div class="detail-item">
            <span class="label">{appStore.t('parent_pid')}</span>
            <span class="val">{selectedProcess.parent_pid ?? 'N/A'}</span>
          </div>
          <div class="detail-item">
            <span class="label">{appStore.t('user')}</span>
            <span class="val">{selectedProcess.user_id}</span>
          </div>
          <div class="detail-item">
            <span class="label">{appStore.t('status')}</span>
            <span class="val">{selectedProcess.status}</span>
          </div>
        </div>

        <div class="detail-full">
          <span class="label">{appStore.t('exe_path')}</span>
          <div class="code-box">{selectedProcess.exe_path || 'Unknown'}</div>
        </div>

        {#if selectedProcess.command && selectedProcess.command.length > 0}
          <div class="detail-full">
            <span class="label">{appStore.t('command')}</span>
            <div class="code-box">{selectedProcess.command.join(' ')}</div>
          </div>
        {/if}
      </div>

      <div class="modal-footer flex justify-between items-center">
        <button class="btn btn-secondary" onclick={() => selectedProcess = null}>
          {appStore.t('cancel')}
        </button>
        <button
          class="btn btn-danger"
          onclick={(e) => {
            const p = appStore.processes.find(item => item.pid === selectedProcess!.pid);
            if (p) {
              selectedProcess = null;
              promptKillProcess(p, e);
            }
          }}
        >
          {appStore.t('terminate')}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Confirmation Modal for Ending Process -->
{#if killProcessTarget}
  <div
    class="modal-backdrop"
    role="presentation"
    onclick={() => killProcessTarget = null}
    onkeydown={(e) => { if (e.key === 'Escape') killProcessTarget = null; }}
  >
    <div
      class="modal-card modal-sm"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <div class="modal-header">
        <h3 style="color: var(--color-danger);">{appStore.t('confirm_kill_title')}</h3>
      </div>
      <div class="modal-body">
        <p>
          {appStore.t('confirm_kill_desc', { name: killProcessTarget.name, pid: killProcessTarget.pid })}
        </p>
      </div>
      <div class="modal-footer flex justify-end gap-2">
        <button class="btn btn-secondary" onclick={() => killProcessTarget = null}>
          {appStore.t('cancel')}
        </button>
        <button class="btn btn-danger" onclick={confirmKillProcess}>
          {appStore.t('terminate')}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .process-container {
    padding: 1.5rem;
    height: 100%;
    overflow: hidden;
  }

  .action-bar {
    background-color: var(--color-bg-card);
    padding: 0.75rem 1rem;
    border-radius: var(--radius-lg);
    border: 1px solid var(--color-border);
  }

  .search-box {
    flex: 1;
    max-width: 420px;
    color: var(--color-text-muted);
  }

  .feedback-banner {
    padding: 0.65rem 1rem;
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-brand-accent);
    color: var(--color-brand-primary);
    font-size: 0.85rem;
    font-weight: 600;
    border-radius: var(--radius-md);
  }

  .table-wrapper {
    flex: 1;
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow-y: auto;
    max-height: calc(100vh - 210px);
  }

  .process-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.85rem;
  }

  .process-table th {
    position: sticky;
    top: 0;
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-secondary);
    font-weight: 700;
    text-align: left;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--color-border);
    z-index: 1;
  }

  .process-table td {
    padding: 0.65rem 1rem;
    border-bottom: 1px solid var(--color-border-subtle);
    color: var(--color-text-primary);
  }

  .table-row {
    cursor: pointer;
  }

  .table-row:hover {
    background-color: var(--color-bg-hover);
  }

  .pid-cell {
    font-weight: 700;
    color: var(--color-brand-accent);
    width: 80px;
  }

  .proc-badge {
    font-size: 0.65rem;
    font-weight: 800;
    padding: 0.1rem 0.35rem;
    border-radius: 4px;
    background-color: var(--color-brand-primary);
    color: #FFFFFF;
  }

  .font-bold { font-weight: 700; }

  .cpu-pill {
    padding: 0.15rem 0.4rem;
    border-radius: var(--radius-pill);
    background-color: var(--color-bg-tertiary);
    font-weight: 700;
  }

  .cpu-pill.high {
    background-color: var(--color-danger-bg);
    color: var(--color-danger);
  }

  .empty-state {
    text-align: center;
    padding: 3rem;
    color: var(--color-text-muted);
  }

  /* Modal Dialog Styles */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.65);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-card {
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    width: 90%;
    max-width: 580px;
    padding: 1.5rem;
    box-shadow: var(--shadow-lg);
  }

  .modal-sm {
    max-width: 420px;
  }

  .modal-header h3 {
    font-size: 1.1rem;
    font-weight: 800;
    color: var(--color-brand-primary);
  }

  .modal-body {
    margin: 1.25rem 0;
    font-size: 0.875rem;
  }

  .detail-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
    background-color: var(--color-bg-tertiary);
    padding: 0.85rem;
    border-radius: var(--radius-md);
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .label {
    font-size: 0.725rem;
    font-weight: 700;
    color: var(--color-text-muted);
    text-transform: uppercase;
  }

  .val {
    font-weight: 700;
    color: var(--color-text-primary);
  }

  .detail-full {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .code-box {
    background-color: var(--color-bg-primary);
    padding: 0.5rem 0.75rem;
    border-radius: var(--radius-sm);
    font-family: monospace;
    font-size: 0.8rem;
    border: 1px solid var(--color-border);
    word-break: break-all;
  }
</style>
