<script lang="ts">
  let {
    options = [],
    value = $bindable(),
    placeholder = '',
    className = '',
    id = '',
    onchange
  }: {
    options: { value: string; label: string }[];
    value: string;
    placeholder?: string;
    className?: string;
    id?: string;
    onchange?: (val: string) => void;
  } = $props();

  let isOpen = $state(false);
  let selectRef = $state<HTMLDivElement | null>(null);

  let selectedOption = $derived(
    options.find(o => o.value === value) || { value, label: value || placeholder }
  );

  function toggleOpen(e: MouseEvent) {
    e.stopPropagation();
    isOpen = !isOpen;
  }

  function selectOption(val: string, e: MouseEvent) {
    e.stopPropagation();
    value = val;
    isOpen = false;
    if (onchange) onchange(val);
  }

  function handleWindowClick(e: MouseEvent) {
    if (selectRef && !selectRef.contains(e.target as Node)) {
      isOpen = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="custom-select-wrapper {className}" bind:this={selectRef} {id}>
  <!-- Trigger Button -->
  <button
    type="button"
    class="custom-select-trigger"
    onclick={toggleOpen}
    aria-expanded={isOpen}
  >
    <span class="truncate">{selectedOption?.label || placeholder}</span>
    <svg class="chevron-icon {isOpen ? 'open' : ''}" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
      <path d="m6 9 6 6 6-6"/>
    </svg>
  </button>

  <!-- Floating Custom Menu (Rounded corners, Chocolate theme) -->
  {#if isOpen}
    <ul class="custom-select-menu" role="listbox">
      {#each options as opt (opt.value)}
        <li
          class="custom-select-option {opt.value === value ? 'selected' : ''}"
          role="option"
          tabindex="0"
          aria-selected={opt.value === value}
          onclick={(e) => selectOption(opt.value, e)}
          onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') selectOption(opt.value, e as unknown as MouseEvent); }}
        >
          <span>{opt.label}</span>
          {#if opt.value === value}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .custom-select-wrapper {
    position: relative;
    width: 100%;
    user-select: none;
  }

  .custom-select-trigger {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    font-family: var(--font-family);
    font-size: 0.875rem;
    font-weight: 600;
    background-color: var(--color-bg-card);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    box-shadow: var(--shadow-sm);
    text-align: left;
    outline: none;
  }

  .custom-select-trigger:hover {
    border-color: var(--color-brand-accent);
  }

  .custom-select-trigger:focus {
    border-color: var(--color-brand-primary);
    box-shadow: 0 0 0 2px rgba(139, 90, 43, 0.2);
  }

  .chevron-icon {
    flex-shrink: 0;
    color: var(--color-brand-accent);
  }

  .chevron-icon.open {
    transform: rotate(180deg);
  }

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Custom Floating Popover with Rounded Corners */
  .custom-select-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    z-index: 9999;
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    max-height: 240px;
    overflow-y: auto;
    padding: 0.35rem;
    list-style: none;
    margin: 0;
  }

  .custom-select-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--color-text-primary);
    border-radius: var(--radius-md);
    cursor: pointer;
    margin-bottom: 2px;
  }

  .custom-select-option:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-brand-primary);
  }

  .custom-select-option.selected {
    background-color: var(--color-brand-primary);
    color: #FFFFFF;
  }

  .custom-select-option.selected:hover {
    background-color: var(--color-brand-dark);
  }
</style>
