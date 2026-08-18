<script lang="ts">
  import { appStore } from '../stores/appState.svelte';
  import type { Language } from '../types';
  import CustomSelect from './CustomSelect.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

  let isMobileMenuOpen = $state(false);
  let updateStatus = $state<{ checking: boolean; message: string | null }>({ checking: false, message: null });

  function handleCheckUpdates() {
    updateStatus = { checking: true, message: 'Buscando actualizaciones...' };
    setTimeout(() => {
      updateStatus = { checking: false, message: '¡Cacao Performance v1.0.0 está actualizado!' };
      setTimeout(() => {
        updateStatus = { checking: false, message: null };
      }, 4000);
    }, 1500);
  }

  const languages: { code: Language; name: string; flag: string }[] = [
    { code: 'es', name: 'Español', flag: '🇪🇸' },
    { code: 'en', name: 'English', flag: '🇺🇸' },
    { code: 'it', name: 'Italiano', flag: '🇮🇹' },
    { code: 'ja', name: '日本語', flag: '🇯🇵' },
    { code: 'zh', name: '中文', flag: '🇨🇳' }
  ];

  const langOptions = languages.map(l => ({
    value: l.code,
    label: `${l.flag} ${l.name}`
  }));

  function toggleTheme() {
    appStore.setTheme(appStore.theme === 'dark' ? 'light' : 'dark');
  }

  function toggleUnit() {
    appStore.setTempUnit(appStore.tempUnit === 'C' ? 'F' : 'C');
  }

  // Window Titlebar Action Handlers with Cross-Platform Resilience
  async function handleMinimize() {
    try {
      const appWindow = getCurrentWebviewWindow();
      await appWindow.minimize();
    } catch {
      try {
        await invoke('window_minimize');
      } catch {}
    }
  }

  async function handleMaximize() {
    try {
      const appWindow = getCurrentWebviewWindow();
      await appWindow.toggleMaximize();
    } catch {
      try {
        await invoke('window_maximize');
      } catch {}
    }
  }

  async function handleClose() {
    try {
      const appWindow = getCurrentWebviewWindow();
      await appWindow.close();
    } catch {
      try {
        await invoke('window_close');
      } catch {}
    }
  }

  async function handleStartDrag(e: MouseEvent) {
    if (e.button !== 0) return;
    const target = e.target as HTMLElement;
    if (
      target.closest('button') ||
      target.closest('a') ||
      target.closest('input') ||
      target.closest('.custom-select-wrapper') ||
      target.closest('.titlebar-right') ||
      target.closest('.nav-tabs') ||
      target.closest('.mobile-drawer-overlay')
    ) {
      return;
    }
    try {
      const appWindow = getCurrentWebviewWindow();
      await appWindow.startDragging();
    } catch {
      try {
        await invoke('drag_window');
      } catch {}
    }
  }
</script>

<div
  class="custom-titlebar flex items-center justify-between"
  data-tauri-drag-region
  onmousedown={handleStartDrag}
  role="toolbar"
  tabindex="-1"
>
  <!-- Left Side: Brand Logo & Title -->
  <div class="titlebar-left flex items-center gap-2" data-tauri-drag-region onmousedown={handleStartDrag} role="presentation">
    <img src="/logo-full-black.png" alt="Logo Cacao" class="titlebar-logo" data-tauri-drag-region />
    <div class="brand-text-stacked flex-col" data-tauri-drag-region>
      <span class="brand-cacao" data-tauri-drag-region>cacao</span>
      <span class="brand-performance" data-tauri-drag-region>performance</span>
    </div>
    <span class="titlebar-version-badge" data-tauri-drag-region>v1.0.0</span>
  </div>

  <!-- Desktop Navigation Tabs Centered -->
  <nav class="nav-tabs desktop-tabs flex items-center gap-1">
    <button
      class="tab-btn {appStore.activeTab === 'dashboard' ? 'active' : ''}"
      onclick={() => appStore.activeTab = 'dashboard'}
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="7" height="7"/>
        <rect x="14" y="3" width="7" height="7"/>
        <rect x="14" y="14" width="7" height="7"/>
        <rect x="3" y="14" width="7" height="7"/>
      </svg>
      {appStore.t('nav_dashboard')}
    </button>

    <button
      class="tab-btn {appStore.activeTab === 'processes' ? 'active' : ''}"
      onclick={() => appStore.activeTab = 'processes'}
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="8" y1="6" x2="21" y2="6"/>
        <line x1="8" y1="12" x2="21" y2="12"/>
        <line x1="8" y1="18" x2="21" y2="18"/>
        <line x1="3" y1="6" x2="3.01" y2="6"/>
        <line x1="3" y1="12" x2="3.01" y2="12"/>
        <line x1="3" y1="18" x2="3.01" y2="18"/>
      </svg>
      {appStore.t('nav_processes')}
    </button>

    <button
      class="tab-btn {appStore.activeTab === 'widget_studio' ? 'active' : ''}"
      onclick={() => appStore.activeTab = 'widget_studio'}
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polygon points="12 2 2 7 12 12 22 7 12 2"/>
        <polyline points="2 17 12 22 22 17"/>
        <polyline points="2 12 12 17 22 12"/>
      </svg>
      {appStore.t('nav_widget_creator')}
    </button>
  </nav>

  <!-- Right Side: Controls & Pinned Window Buttons -->
  <div class="titlebar-right flex items-center gap-1.5">
    <!-- Desktop Controls -->
    <div class="desktop-controls flex items-center gap-1.5">
      <!-- Temp Unit Switcher -->
      <button
        type="button"
        class="titlebar-action-btn"
        onclick={toggleUnit}
        title="Temperature unit (°C / °F)"
      >
        <span class="unit-text font-bold">°{appStore.tempUnit}</span>
      </button>

      <!-- Language Selector -->
      <div class="lang-select-wrapper">
        <CustomSelect
          options={langOptions}
          bind:value={appStore.lang}
          onchange={(val) => appStore.setLang(val as Language)}
        />
      </div>

      <!-- Theme Toggle -->
      <button
        type="button"
        class="titlebar-action-btn"
        onclick={toggleTheme}
        title={appStore.theme === 'light' ? 'Modo Claro' : 'Modo Oscuro'}
      >
        {#if appStore.theme === 'dark'}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="5"/>
            <line x1="12" y1="1" x2="12" y2="3"/>
            <line x1="12" y1="21" x2="12" y2="23"/>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
            <line x1="1" y1="12" x2="3" y2="12"/>
            <line x1="21" y1="12" x2="23" y2="12"/>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
          </svg>
        {:else}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
          </svg>
        {/if}
      </button>

      <!-- Check Updates Button -->
      <button
        type="button"
        class="titlebar-action-btn {updateStatus.checking ? 'animate-spin' : ''}"
        onclick={handleCheckUpdates}
        title="Buscar actualizaciones"
      >
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21.5 2v6h-6M2.5 22v-6h6"/>
          <path d="M2 11.5a10 10 0 0 1 18.8-4.3L21.5 8M2.5 16l1.2 0.8A10 10 0 0 0 22 12.5"/>
        </svg>
      </button>

      <div class="titlebar-divider"></div>
    </div>

    <!-- Mobile Hamburger Menu Button (visible on narrow screens) -->
    <button
      type="button"
      class="mobile-menu-btn"
      onclick={() => isMobileMenuOpen = !isMobileMenuOpen}
      title="Menu Navigation"
    >
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        {#if isMobileMenuOpen}
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        {:else}
          <line x1="3" y1="12" x2="21" y2="12"/>
          <line x1="3" y1="6" x2="21" y2="6"/>
          <line x1="3" y1="18" x2="21" y2="18"/>
        {/if}
      </svg>
    </button>

    <!-- Pinned Window Action Buttons (Minimize, Maximize, Close) -->
    <button type="button" class="titlebar-window-btn" onclick={handleMinimize} title="Minimizar">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <line x1="5" y1="12" x2="19" y2="12"/>
      </svg>
    </button>

    <button type="button" class="titlebar-window-btn" onclick={handleMaximize} title="Maximizar / Restaurar">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <rect x="4" y="4" width="16" height="16" rx="1"/>
      </svg>
    </button>

    <button type="button" class="titlebar-window-btn btn-close" onclick={handleClose} title="Cerrar">
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>
</div>

<!-- Floating Responsive Drawer Menu (Narrow Screens) -->
{#if isMobileMenuOpen}
  <div
    class="mobile-drawer-overlay"
    onclick={() => isMobileMenuOpen = false}
    onkeydown={(e) => { if (e.key === 'Escape') isMobileMenuOpen = false; }}
    role="presentation"
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="mobile-drawer-card"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      aria-label="Mobile Navigation Menu"
      tabindex="-1"
    >
      <div class="mobile-nav-list flex-col gap-2">
        <button
          class="mobile-nav-item {appStore.activeTab === 'dashboard' ? 'active' : ''}"
          onclick={() => { appStore.activeTab = 'dashboard'; isMobileMenuOpen = false; }}
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7"/>
            <rect x="14" y="3" width="7" height="7"/>
            <rect x="14" y="14" width="7" height="7"/>
            <rect x="3" y="14" width="7" height="7"/>
          </svg>
          <span>{appStore.t('nav_dashboard')}</span>
        </button>

        <button
          class="mobile-nav-item {appStore.activeTab === 'processes' ? 'active' : ''}"
          onclick={() => { appStore.activeTab = 'processes'; isMobileMenuOpen = false; }}
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="8" y1="6" x2="21" y2="6"/>
            <line x1="8" y1="12" x2="21" y2="12"/>
            <line x1="8" y1="18" x2="21" y2="18"/>
            <line x1="3" y1="6" x2="3.01" y2="6"/>
            <line x1="3" y1="12" x2="3.01" y2="12"/>
            <line x1="3" y1="18" x2="3.01" y2="18"/>
          </svg>
          <span>{appStore.t('nav_processes')}</span>
        </button>

        <button
          class="mobile-nav-item {appStore.activeTab === 'widget_studio' ? 'active' : ''}"
          onclick={() => { appStore.activeTab = 'widget_studio'; isMobileMenuOpen = false; }}
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="12 2 2 7 12 12 22 7 12 2"/>
            <polyline points="2 17 12 22 22 17"/>
            <polyline points="2 12 12 17 22 12"/>
          </svg>
          <span>{appStore.t('nav_widget_creator')}</span>
        </button>
      </div>

      <div class="mobile-drawer-divider"></div>

      <!-- Mobile Controls Row -->
      <div class="mobile-controls-row flex items-center justify-between gap-2">
        <button type="button" class="btn btn-secondary btn-sm" onclick={toggleUnit}>
          °{appStore.tempUnit}
        </button>

        <div class="flex-1">
          <CustomSelect
            options={langOptions}
            bind:value={appStore.lang}
            onchange={(val) => appStore.setLang(val as Language)}
          />
        </div>

        <button type="button" class="btn btn-secondary btn-sm" onclick={toggleTheme}>
          {#if appStore.theme === 'dark'}☀️{:else}🌙{/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Floating Update Toast Notification Banner -->
{#if updateStatus.message}
  <div class="update-toast-banner flex items-center gap-2">
    {#if updateStatus.checking}
      <svg class="animate-spin" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <circle cx="12" cy="12" r="10" stroke-opacity="0.25"/>
        <path d="M12 2a10 10 0 0 1 10 10" />
      </svg>
    {:else}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-success">
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
        <polyline points="22 4 12 14.01 9 11.01"/>
      </svg>
    {/if}
    <span>{updateStatus.message}</span>
  </div>
{/if}

<style>
  .custom-titlebar {
    height: 48px;
    min-height: 48px;
    max-height: 48px;
    background-color: var(--color-bg-card);
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 0.75rem;
    flex-wrap: nowrap;
    user-select: none;
    -webkit-user-select: none;
    -webkit-app-region: drag;
    app-region: drag;
    position: sticky;
    top: 0;
    z-index: 1000;
    width: 100%;
    box-sizing: border-box;
    overflow: visible;
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-shrink: 0;
    white-space: nowrap;
    -webkit-app-region: drag;
    app-region: drag;
  }

  .titlebar-logo {
    height: 26px;
    width: auto;
    object-fit: contain;
    -webkit-app-region: drag;
    app-region: drag;
  }

  :global([data-theme="dark"]) .titlebar-logo {
    filter: invert(1) brightness(1.8);
  }

  .brand-text-stacked {
    line-height: 1.0;
    -webkit-app-region: drag;
    app-region: drag;
  }

  .brand-cacao {
    font-size: 0.85rem;
    font-weight: 800;
    color: var(--color-brand-primary);
    text-transform: lowercase;
    letter-spacing: -0.02em;
  }

  .brand-performance {
    font-size: 0.68rem;
    font-weight: 700;
    color: var(--color-brand-accent);
    text-transform: lowercase;
    letter-spacing: 0.02em;
  }

  .titlebar-version-badge {
    background: var(--color-bg-tertiary);
    color: var(--color-brand-primary);
    font-size: 0.7rem;
    font-weight: 700;
    padding: 0.15rem 0.5rem;
    border-radius: var(--radius-pill);
    border: 1px solid var(--color-border);
    -webkit-app-region: drag;
    app-region: drag;
  }

  .nav-tabs {
    background-color: var(--color-bg-tertiary);
    padding: 3px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--color-border-subtle);
    display: flex;
    align-items: center;
    gap: 0.2rem;
    flex-shrink: 0;
    white-space: nowrap;
    -webkit-app-region: no-drag;
    app-region: no-drag;
  }

  .tab-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.3rem 0.65rem;
    font-size: 0.78rem;
    font-weight: 700;
    color: var(--color-text-secondary);
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    white-space: nowrap;
    -webkit-app-region: no-drag;
    app-region: no-drag;
  }

  .tab-btn.active {
    background-color: var(--color-brand-primary);
    color: #FFFFFF;
  }

  .titlebar-right {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    height: 100%;
    flex-shrink: 0;
    white-space: nowrap;
    -webkit-app-region: no-drag;
    app-region: no-drag;
    overflow: visible;
  }

  .desktop-controls {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    overflow: visible;
  }

  .mobile-menu-btn {
    display: none;
    background: transparent;
    border: none;
    color: var(--color-brand-primary);
    width: 32px;
    height: 32px;
    border-radius: var(--radius-md);
    align-items: center;
    justify-content: center;
    cursor: pointer;
    -webkit-app-region: no-drag;
  }

  .mobile-menu-btn:hover {
    background-color: var(--color-bg-hover);
  }

  .titlebar-action-btn,
  .titlebar-window-btn {
    -webkit-app-region: no-drag;
    app-region: no-drag;
    flex-shrink: 0;
  }

  .titlebar-action-btn {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    width: 32px;
    height: 32px;
    border-radius: var(--radius-pill);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }

  .titlebar-action-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .unit-text {
    font-size: 0.75rem;
  }

  .lang-select-wrapper {
    width: 110px;
    flex-shrink: 0;
    position: relative;
    z-index: 1001;
  }

  .titlebar-divider {
    width: 1px;
    height: 18px;
    background: var(--color-border);
    margin: 0 0.25rem;
    flex-shrink: 0;
  }

  .titlebar-window-btn {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    width: 34px;
    height: 30px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }

  .titlebar-window-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .titlebar-window-btn.btn-close:hover {
    background: var(--color-danger);
    color: #FFFFFF;
  }

  /* Responsive Media Queries for Narrow Screens (< 820px) */
  @media (max-width: 820px) {
    .desktop-tabs {
      display: none !important;
    }

    .desktop-controls {
      display: none !important;
    }

    .mobile-menu-btn {
      display: inline-flex !important;
    }
  }

  /* Floating Responsive Drawer Menu Styles */
  .mobile-drawer-overlay {
    position: fixed;
    top: 48px;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(4px);
    z-index: 9999;
    padding: 0.75rem;
    display: flex;
    justify-content: flex-end;
  }

  .mobile-drawer-card {
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    width: 100%;
    max-width: 300px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    height: fit-content;
    overflow: visible;
    position: relative;
  }

  .mobile-nav-item {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    padding: 0.65rem 0.85rem;
    font-size: 0.875rem;
    font-weight: 700;
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border-subtle);
    border-radius: var(--radius-md);
    cursor: pointer;
    width: 100%;
    text-align: left;
  }

  .mobile-nav-item.active {
    background-color: var(--color-brand-primary);
    color: #FFFFFF;
    border-color: var(--color-brand-accent);
  }

  /* Floating Update Toast Banner Styles */
  .update-toast-banner {
    position: fixed;
    top: 56px;
    right: 1.25rem;
    background-color: var(--color-bg-card);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-pill);
    padding: 0.5rem 1rem;
    box-shadow: var(--shadow-lg);
    font-size: 0.8rem;
    font-weight: 700;
    z-index: 10000;
    animation: toastSlideIn 0.25s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes toastSlideIn {
    from {
      opacity: 0;
      transform: translateY(-8px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
