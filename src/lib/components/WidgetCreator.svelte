<script lang="ts">
  import { appStore } from '../stores/appState.svelte';
  import WidgetPreview from './WidgetPreview.svelte';
  import CustomSelect from './CustomSelect.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let statusMsg = $state<string | null>(null);

  const fontOptions = [
    { label: 'Montserrat', value: 'Montserrat' },
    { label: 'Inter', value: 'Inter' },
    { label: 'Roboto', value: 'Roboto' },
    { label: 'Outfit', value: 'Outfit' },
    { label: 'Fira Code (Mono)', value: 'Fira Code' },
    { label: 'Poppins', value: 'Poppins' },
    { label: 'Lato', value: 'Lato' },
    { label: 'Open Sans', value: 'Open Sans' }
  ];

  const positionOptions = [
    { label: 'Centro (Center)', value: 'center' },
    { label: 'Arriba (Top)', value: 'top' },
    { label: 'Abajo (Bottom)', value: 'bottom' },
    { label: 'Izquierda (Left)', value: 'left' },
    { label: 'Derecha (Right)', value: 'right' },
    { label: 'Arriba Izquierda (Top Left)', value: 'top left' },
    { label: 'Arriba Derecha (Top Right)', value: 'top right' },
    { label: 'Abajo Izquierda (Bottom Left)', value: 'bottom left' },
    { label: 'Abajo Derecha (Bottom Right)', value: 'bottom right' }
  ];

  const imgSizeOptions = [
    { label: 'Cubrir (Cover)', value: 'cover' },
    { label: 'Contener (Contain)', value: 'contain' },
    { label: 'Original (Auto)', value: 'auto' }
  ];

  let presetOptions = $derived(
    appStore.savedWidgets.map(w => ({ value: w.id, label: w.title }))
  );

  let widgetSizeOptions = $derived([
    { value: 'small', label: appStore.t('size_small') },
    { value: 'medium', label: appStore.t('size_medium') },
    { value: 'large', label: appStore.t('size_large') },
    { value: 'custom', label: appStore.t('size_custom') }
  ]);

  let shapeOptions = $derived([
    { value: 'square', label: appStore.t('shape_square') },
    { value: 'rounded', label: appStore.t('shape_rounded') },
    { value: 'capsule', label: appStore.t('shape_capsule') }
  ]);

  let fontWeightOptions = $derived([
    { value: '300', label: appStore.t('font_light') },
    { value: '400', label: appStore.t('font_regular') },
    { value: '500', label: appStore.t('font_medium') },
    { value: '600', label: appStore.t('font_semibold') },
    { value: '700', label: appStore.t('font_bold') }
  ]);

  async function handleLaunchWidget() {
    appStore.saveCurrentWidget();
    try {
      const windowLabel = await invoke<string>('spawn_widget_window', { config: appStore.activeWidgetConfig });
      statusMsg = appStore.t('widget_launched_msg') + ` (${windowLabel})`;
    } catch (err) {
      console.error('Launch widget error:', err);
      statusMsg = `Error: ${err}`;
    } finally {
      setTimeout(() => statusMsg = null, 4000);
    }
  }

  function handleSavePreset() {
    appStore.saveCurrentWidget();
    statusMsg = appStore.t('save_widget') + ' ✓';
    setTimeout(() => statusMsg = null, 3000);
  }

  function handleImageUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        if (event.target?.result) {
          appStore.activeWidgetConfig.bg_image = event.target.result as string;
        }
      };
      reader.readAsDataURL(file);
    }
  }

  function clearImage() {
    appStore.activeWidgetConfig.bg_image = '';
  }
</script>

<div class="studio-container">
  <!-- Left Column: Settings Panel -->
  <div class="studio-settings flex-col gap-6">
    <div class="header-banner flex items-center justify-between">
      <div>
        <h2>{appStore.t('widget_studio_title')}</h2>
        <p class="subtitle">{appStore.activeWidgetConfig.title}</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-secondary btn-sm" onclick={() => appStore.createNewWidget()}>
          + {appStore.t('create_new_widget')}
        </button>
        <button class="btn btn-primary btn-sm" onclick={handleSavePreset}>
          {appStore.t('save_widget')}
        </button>
      </div>
    </div>

    {#if statusMsg}
      <div class="status-alert">
        {statusMsg}
      </div>
    {/if}

    <!-- Saved Presets Selector -->
    <div class="setting-group">
      <label class="group-title" for="preset-select">{appStore.t('widget_presets')}</label>
      <div class="flex items-center gap-2">
        <CustomSelect
          id="preset-select"
          options={presetOptions}
          bind:value={appStore.activeWidgetConfig.id}
          onchange={(val) => {
            const found = appStore.savedWidgets.find(w => w.id === val);
            if (found) appStore.activeWidgetConfig = JSON.parse(JSON.stringify(found));
          }}
        />
        {#if appStore.savedWidgets.length > 1}
          <button
            class="btn btn-danger btn-sm"
            onclick={() => appStore.deleteWidget(appStore.activeWidgetConfig.id)}
            title="Delete preset"
          >
            ✕
          </button>
        {/if}
      </div>
    </div>

    <!-- Title Config -->
    <div class="setting-group">
      <label class="group-title" for="widget-title-input">{appStore.t('widget_title_label')}</label>
      <input
        id="widget-title-input"
        type="text"
        class="input"
        bind:value={appStore.activeWidgetConfig.title}
      />
    </div>

    <!-- SECTION 1: PARAMETERS -->
    <div class="setting-group">
      <span class="group-title">{appStore.t('params_section')}</span>
      <div class="checkbox-grid">
        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_cpu} />
          <span>{appStore.t('param_cpu')}</span>
        </label>

        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_gpu} />
          <span>{appStore.t('param_gpu')}</span>
        </label>

        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_ram} />
          <span>{appStore.t('param_ram')}</span>
        </label>

        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_disk} />
          <span>{appStore.t('param_disk')}</span>
        </label>

        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_time} />
          <span>{appStore.t('param_time')}</span>
        </label>

        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_date} />
          <span>{appStore.t('param_date')}</span>
        </label>

        <label class="checkbox-item">
          <input type="checkbox" bind:checked={appStore.activeWidgetConfig.show_process_count} />
          <span>{appStore.t('param_processes')}</span>
        </label>
      </div>
    </div>

    <!-- SECTION 2: STYLING -->
    <div class="setting-group">
      <span class="group-title">{appStore.t('style_section')}</span>
      
      <div class="controls-grid">
        <!-- Background Color -->
        <div class="ctrl-field">
          <label class="field-label" for="bg-color-picker">{appStore.t('bg_color')}</label>
          <div class="flex items-center gap-2">
            <input
              id="bg-color-picker"
              type="color"
              class="color-picker"
              bind:value={appStore.activeWidgetConfig.bg_color}
            />
            <input
              type="text"
              class="input font-mono"
              bind:value={appStore.activeWidgetConfig.bg_color}
            />
          </div>
        </div>

        <!-- Text Color -->
        <div class="ctrl-field">
          <label class="field-label" for="text-color-picker">Color del Texto</label>
          <div class="flex items-center gap-2">
            <input
              id="text-color-picker"
              type="color"
              class="color-picker"
              bind:value={appStore.activeWidgetConfig.text_color}
            />
            <input
              type="text"
              class="input font-mono"
              bind:value={appStore.activeWidgetConfig.text_color}
            />
          </div>
        </div>

        <!-- Background Image Config -->
        <div class="ctrl-field">
          <label class="field-label" for="bg-img-upload">Imagen de Fondo del Widget</label>
          <div class="flex-col gap-2">
            <div class="flex items-center gap-2">
              <label class="btn btn-secondary btn-sm file-label">
                Subir Imagen...
                <input id="bg-img-upload" type="file" accept="image/*" onchange={handleImageUpload} class="hidden-input" />
              </label>
              {#if appStore.activeWidgetConfig.bg_image}
                <button class="btn btn-danger btn-sm" onclick={clearImage}>
                  Quitar Imagen
                </button>
              {/if}
            </div>

            <input
              type="text"
              class="input font-mono"
              placeholder="O pega la URL de la imagen aquí..."
              bind:value={appStore.activeWidgetConfig.bg_image}
            />
          </div>
        </div>

        {#if appStore.activeWidgetConfig.bg_image}
          <!-- Image Size -->
          <div class="ctrl-field">
            <label class="field-label" for="bg-img-size-select">Tamaño de la Imagen</label>
            <CustomSelect
              id="bg-img-size-select"
              options={imgSizeOptions}
              bind:value={appStore.activeWidgetConfig.bg_image_size}
            />
          </div>

          <!-- Image Position -->
          <div class="ctrl-field">
            <label class="field-label" for="bg-img-pos-select">Posición de la Imagen</label>
            <CustomSelect
              id="bg-img-pos-select"
              options={positionOptions}
              bind:value={appStore.activeWidgetConfig.bg_image_position}
            />
          </div>

          <!-- Image Opacity -->
          <div class="ctrl-field">
            <label class="field-label" for="bg-img-opacity-range">Opacidad de la Imagen: {Math.round(appStore.activeWidgetConfig.bg_image_opacity * 100)}%</label>
            <input
              id="bg-img-opacity-range"
              type="range"
              min="0.0"
              max="1.0"
              step="0.05"
              class="range-slider"
              bind:value={appStore.activeWidgetConfig.bg_image_opacity}
            />
          </div>
        {/if}

        <!-- Widget Size -->
        <div class="ctrl-field">
          <label class="field-label" for="widget-size-select">{appStore.t('widget_size')}</label>
          <CustomSelect
            id="widget-size-select"
            options={widgetSizeOptions}
            bind:value={appStore.activeWidgetConfig.widget_size}
          />
        </div>

        {#if appStore.activeWidgetConfig.widget_size === 'custom'}
          <div class="ctrl-field">
            <label class="field-label" for="custom-w-input">{appStore.t('custom_width')}</label>
            <input id="custom-w-input" type="number" class="input" bind:value={appStore.activeWidgetConfig.custom_width} min="80" max="800" />
          </div>

          <div class="ctrl-field">
            <label class="field-label" for="custom-h-input">{appStore.t('custom_height')}</label>
            <input id="custom-h-input" type="number" class="input" bind:value={appStore.activeWidgetConfig.custom_height} min="30" max="800" />
          </div>
        {/if}

        <!-- Widget Shape -->
        <div class="ctrl-field">
          <label class="field-label" for="shape-select">{appStore.t('widget_shape')}</label>
          <CustomSelect
            id="shape-select"
            options={shapeOptions}
            bind:value={appStore.activeWidgetConfig.shape}
          />
        </div>

        <!-- Typography / Font Family -->
        <div class="ctrl-field">
          <label class="field-label" for="font-family-select">Fuente / Tipografía</label>
          <CustomSelect
            id="font-family-select"
            options={fontOptions}
            bind:value={appStore.activeWidgetConfig.font_family}
          />
        </div>

        <!-- Text Weight -->
        <div class="ctrl-field">
          <label class="field-label" for="font-weight-select">Grosor de la Fuente</label>
          <CustomSelect
            id="font-weight-select"
            options={fontWeightOptions}
            bind:value={appStore.activeWidgetConfig.font_weight}
          />
        </div>

        <!-- Text Size -->
        <div class="ctrl-field">
          <label class="field-label" for="text-size-range">{appStore.t('text_size')}: {appStore.activeWidgetConfig.font_size}px</label>
          <input
            id="text-size-range"
            type="range"
            min="10"
            max="22"
            class="range-slider"
            bind:value={appStore.activeWidgetConfig.font_size}
          />
        </div>

        <!-- Widget Opacity -->
        <div class="ctrl-field">
          <label class="field-label" for="widget-opacity-range">{appStore.t('widget_opacity')}: {Math.round(appStore.activeWidgetConfig.widget_opacity * 100)}%</label>
          <input
            id="widget-opacity-range"
            type="range"
            min="0.1"
            max="1.0"
            step="0.05"
            class="range-slider"
            bind:value={appStore.activeWidgetConfig.widget_opacity}
          />
        </div>

        <!-- Enable Border -->
        <div class="ctrl-field">
          <label class="checkbox-item" style="margin-top: 1rem;">
            <input type="checkbox" bind:checked={appStore.activeWidgetConfig.has_border} />
            <span>{appStore.t('border_enable')}</span>
          </label>
        </div>

        {#if appStore.activeWidgetConfig.has_border}
          <!-- Border Color -->
          <div class="ctrl-field">
            <label class="field-label" for="border-color-picker">{appStore.t('border_color')}</label>
            <div class="flex items-center gap-2">
              <input
                id="border-color-picker"
                type="color"
                class="color-picker"
                bind:value={appStore.activeWidgetConfig.border_color}
              />
              <input
                type="text"
                class="input font-mono"
                bind:value={appStore.activeWidgetConfig.border_color}
              />
            </div>
          </div>

          <!-- Border Opacity -->
          <div class="ctrl-field">
            <label class="field-label" for="border-opacity-range">{appStore.t('border_opacity')}: {Math.round(appStore.activeWidgetConfig.border_opacity * 100)}%</label>
            <input
              id="border-opacity-range"
              type="range"
              min="0.1"
              max="1.0"
              step="0.05"
              class="range-slider"
              bind:value={appStore.activeWidgetConfig.border_opacity}
            />
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Right Column: Live Preview & Floating Window Launcher -->
  <div class="studio-preview flex-col items-center justify-center gap-6">
    <div class="preview-card flex-col items-center gap-4">
      <div class="preview-header flex items-center justify-between w-full">
        <span class="preview-title">{appStore.t('preview')}</span>
        <span class="badge badge-primary">{appStore.activeWidgetConfig.font_family} {appStore.activeWidgetConfig.font_weight}</span>
      </div>

      <!-- Live Canvas Preview -->
      <div class="preview-stage flex items-center justify-center">
        <WidgetPreview config={appStore.activeWidgetConfig} metrics={appStore.metrics} />
      </div>
    </div>

    <!-- Launch Desktop Widget Button -->
    <button class="btn btn-primary launch-btn flex items-center gap-2" onclick={handleLaunchWidget}>
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
        <polyline points="15 3 21 3 21 9"/>
        <line x1="10" y1="14" x2="21" y2="3"/>
      </svg>
      <span>{appStore.t('launch_desktop_widget')}</span>
    </button>
  </div>
</div>

<style>
  .studio-container {
    display: grid;
    grid-template-columns: 460px 1fr;
    height: 100%;
    padding: 1.25rem;
    gap: 1.25rem;
    box-sizing: border-box;
    overflow: hidden;
    background-color: var(--color-bg-primary);
  }

  .studio-settings {
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    padding: 1.5rem;
    overflow-y: auto;
    overflow-x: hidden;
    max-height: calc(100vh - 48px - 2.5rem);
    clip-path: inset(0 round var(--radius-lg));
  }

  .header-banner h2 {
    font-size: 1.1rem;
    font-weight: 800;
    color: var(--color-brand-primary);
  }

  .subtitle {
    font-size: 0.775rem;
    color: var(--color-text-muted);
    font-weight: 600;
  }

  .status-alert {
    padding: 0.65rem 1rem;
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-brand-accent);
    color: var(--color-brand-primary);
    font-size: 0.825rem;
    font-weight: 700;
    border-radius: var(--radius-md);
  }

  .setting-group {
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .group-title {
    font-size: 0.8rem;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--color-brand-primary);
  }

  .checkbox-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.825rem;
    font-weight: 600;
    color: var(--color-text-primary);
    cursor: pointer;
  }

  .controls-grid {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  .ctrl-field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .field-label {
    font-size: 0.775rem;
    font-weight: 700;
    color: var(--color-text-secondary);
  }

  .color-picker {
    width: 36px;
    height: 36px;
    border: none;
    border-radius: var(--radius-sm);
    cursor: pointer;
    background: transparent;
  }

  .file-label {
    position: relative;
    overflow: hidden;
    cursor: pointer;
  }

  .hidden-input {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
  }

  .range-slider {
    accent-color: var(--color-brand-primary);
    cursor: pointer;
  }

  .font-mono { font-family: monospace; }

  .studio-settings::-webkit-scrollbar {
    width: 6px;
  }

  .studio-settings::-webkit-scrollbar-track {
    background: transparent;
    margin-top: 16px;
    margin-bottom: 16px;
  }

  .studio-settings::-webkit-scrollbar-thumb {
    background: var(--color-brand-accent);
    border-radius: 9999px;
  }

  .studio-preview {
    background: transparent;
    padding: 1rem;
    overflow-y: auto;
    max-height: calc(100vh - 48px - 2.5rem);
  }

  .preview-card {
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: 1.5rem;
    box-shadow: var(--shadow-md);
    width: 100%;
    max-width: 520px;
  }

  .preview-title {
    font-size: 0.9rem;
    font-weight: 800;
    color: var(--color-brand-primary);
  }

  .preview-stage {
    min-height: 280px;
    width: 100%;
    background-image: radial-gradient(var(--color-border) 1px, transparent 1px);
    background-size: 16px 16px;
    border-radius: var(--radius-md);
    padding: 1.5rem;
  }

  .launch-btn {
    padding: 0.85rem 1.85rem;
    font-size: 0.95rem;
    font-weight: 800;
    border-radius: var(--radius-pill);
    box-shadow: var(--shadow-md);
  }

  .launch-btn:hover {
    background-color: #FAF5F0;
    color: #1A0F08 !important;
  }
</style>
