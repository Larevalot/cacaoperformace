<script lang="ts">
  import { appStore, formatTemp } from '../stores/appState.svelte';

  let canvasEl: HTMLCanvasElement | null = $state(null);
  let animId: number;

  $effect(() => {
    if (!canvasEl) return;
    const ctx = canvasEl.getContext('2d');
    if (!ctx) return;

    let width = 0;
    let height = 0;
    interface Dot {
      r: number;
      c: number;
      baseX: number;
      baseY: number;
      x: number;
      y: number;
      vx: number;
      vy: number;
      size: number;
    }
    let dots: Dot[] = [];
    let mouse = { x: -1000, y: -1000 };
    let time = 0;

    const spacing = 26;

    function initDots() {
      if (!canvasEl || !canvasEl.parentElement) return;
      const parent = canvasEl.parentElement;
      width = canvasEl.width = parent.clientWidth;
      height = canvasEl.height = parent.clientHeight;
      dots = [];

      const cols = Math.ceil(width / spacing) + 1;
      const rows = Math.ceil(height / spacing) + 1;

      for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {
          const baseX = c * spacing;
          const baseY = r * spacing;
          dots.push({
            r,
            c,
            baseX,
            baseY,
            x: baseX,
            y: baseY,
            vx: 0,
            vy: 0,
            size: 2.2
          });
        }
      }
    }

    initDots();

    const parentEl = canvasEl.parentElement;
    if (!parentEl) return;

    const resizeObserver = new ResizeObserver(() => {
      initDots();
    });
    resizeObserver.observe(parentEl);

    function handleMouseMove(e: MouseEvent) {
      if (!canvasEl) return;
      const rect = canvasEl.getBoundingClientRect();
      mouse.x = e.clientX - rect.left;
      mouse.y = e.clientY - rect.top;
    }

    function handleMouseLeave() {
      mouse.x = -1000;
      mouse.y = -1000;
    }

    parentEl.addEventListener('mousemove', handleMouseMove);
    parentEl.addEventListener('mouseleave', handleMouseLeave);

    function animate() {
      if (!canvasEl || !ctx) return;
      time += 0.025;
      ctx.clearRect(0, 0, width, height);

      const maxDist = 135;

      // Update positions
      for (let i = 0; i < dots.length; i++) {
        const dot = dots[i];

        // Wave motion
        const waveX = Math.sin(dot.baseY * 0.04 + time * 1.5) * 7;
        const waveY = Math.cos(dot.baseX * 0.04 + time * 1.5) * 7;
        let targetX = dot.baseX + waveX;
        let targetY = dot.baseY + waveY;

        // Repulsion from mouse
        const dx = dot.x - mouse.x;
        const dy = dot.y - mouse.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist < maxDist && dist > 0) {
          const force = Math.pow((maxDist - dist) / maxDist, 2);
          const angle = Math.atan2(dy, dx);
          const repelDist = force * 55;
          targetX += Math.cos(angle) * repelDist;
          targetY += Math.sin(angle) * repelDist;
        }

        // Spring physics
        dot.vx = (dot.vx + (targetX - dot.x) * 0.12) * 0.8;
        dot.vy = (dot.vy + (targetY - dot.y) * 0.12) * 0.8;
        dot.x += dot.vx;
        dot.y += dot.vy;
      }

      // Draw grid lines
      const cols = Math.ceil(width / spacing) + 1;
      ctx.lineWidth = 1;

      for (let i = 0; i < dots.length; i++) {
        const dot = dots[i];

        // Connect to right neighbor
        if (dot.c + 1 < cols && i + 1 < dots.length) {
          const rightDot = dots[i + 1];
          if (rightDot.r === dot.r) {
            const dLine = Math.hypot(dot.x - rightDot.x, dot.y - rightDot.y);
            if (dLine < spacing * 1.6) {
              const alpha = (1 - dLine / (spacing * 1.6)) * 0.15;
              ctx.strokeStyle = `rgba(210, 125, 45, ${alpha})`;
              ctx.beginPath();
              ctx.moveTo(dot.x, dot.y);
              ctx.lineTo(rightDot.x, rightDot.y);
              ctx.stroke();
            }
          }
        }

        // Connect to bottom neighbor
        const bottomIdx = i + cols;
        if (bottomIdx < dots.length) {
          const bottomDot = dots[bottomIdx];
          const dLine = Math.hypot(dot.x - bottomDot.x, dot.y - bottomDot.y);
          if (dLine < spacing * 1.6) {
            const alpha = (1 - dLine / (spacing * 1.6)) * 0.15;
            ctx.strokeStyle = `rgba(210, 125, 45, ${alpha})`;
            ctx.beginPath();
            ctx.moveTo(dot.x, dot.y);
            ctx.lineTo(bottomDot.x, bottomDot.y);
            ctx.stroke();
          }
        }
      }

      // Draw dots
      for (let i = 0; i < dots.length; i++) {
        const dot = dots[i];
        const distFromBase = Math.hypot(dot.x - dot.baseX, dot.y - dot.baseY);
        const intensity = Math.min(1, distFromBase / 30);
        const alpha = 0.25 + intensity * 0.6;

        ctx.beginPath();
        ctx.arc(dot.x, dot.y, dot.size + intensity * 1.2, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(${210 + Math.floor(intensity * 45)}, ${125 + Math.floor(intensity * 60)}, 45, ${alpha})`;
        ctx.fill();
      }

      animId = requestAnimationFrame(animate);
    }

    animate();

    return () => {
      cancelAnimationFrame(animId);
      resizeObserver.disconnect();
      parentEl.removeEventListener('mousemove', handleMouseMove);
      parentEl.removeEventListener('mouseleave', handleMouseLeave);
    };
  });
</script>

<div class="metrics-dashboard-layout">
  <!-- Left Column: Tall Narrow Canvas Animation Box (100% Vertical Height) -->
  <div class="mesh-card">
    <div class="mesh-canvas-wrapper">
      <canvas bind:this={canvasEl} class="mesh-canvas"></canvas>
    </div>
  </div>

  <!-- Right Column: 4 Metric Cards Stacked Vertically -->
  <div class="metrics-vertical-stack">
    <!-- CPU Card -->
    <div class="card flex-col justify-between">
      <div class="card-header flex items-center justify-between">
        <div class="flex items-center gap-2">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="4" y="4" width="16" height="16" rx="2"/>
            <rect x="9" y="9" width="6" height="6"/>
            <line x1="9" y1="1" x2="9" y2="4"/>
            <line x1="15" y1="1" x2="15" y2="4"/>
            <line x1="9" y1="20" x2="9" y2="23"/>
            <line x1="15" y1="20" x2="15" y2="23"/>
            <line x1="20" y1="9" x2="23" y2="9"/>
            <line x1="20" y1="15" x2="23" y2="15"/>
            <line x1="1" y1="9" x2="4" y2="9"/>
            <line x1="1" y1="15" x2="4" y2="15"/>
          </svg>
          <h3>{appStore.t('metric_cpu')}</h3>
        </div>
        <span class="badge badge-primary">{formatTemp(appStore.metrics.cpu_temp_c, appStore.tempUnit)}</span>
      </div>
      
      <div class="card-body flex items-baseline justify-between my-1">
        <span class="metric-value">{appStore.metrics.cpu_usage.toFixed(1)}%</span>
        <span class="metric-sub">{appStore.metrics.process_count} procs</span>
      </div>

      <div class="progress-bar-bg">
        <div class="progress-bar-fill" style="width: {Math.min(appStore.metrics.cpu_usage, 100)}%;"></div>
      </div>
    </div>

    <!-- GPU Card -->
    <div class="card flex-col justify-between">
      <div class="card-header flex items-center justify-between">
        <div class="flex items-center gap-2">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="12 2 2 7 12 12 22 7 12 2"/>
            <polyline points="2 17 12 22 22 17"/>
            <polyline points="2 12 12 17 22 12"/>
          </svg>
          <h3>{appStore.t('metric_gpu')}</h3>
        </div>
        <span class="badge badge-primary">
          {appStore.metrics.gpu_temp_c >= 0 ? formatTemp(appStore.metrics.gpu_temp_c, appStore.tempUnit) : 'N/A'}
        </span>
      </div>
      
      <div class="card-body flex items-baseline justify-between my-1">
        <span class="metric-value">
          {appStore.metrics.gpu_usage >= 0 ? `${appStore.metrics.gpu_usage.toFixed(1)}%` : 'Sin info'}
        </span>
        <span class="metric-sub">
          {appStore.metrics.gpu_usage >= 0 ? 'Active' : 'Sin sensor'}
        </span>
      </div>

      <div class="progress-bar-bg">
        <div class="progress-bar-fill" style="width: {appStore.metrics.gpu_usage >= 0 ? Math.min(appStore.metrics.gpu_usage, 100) : 0}%;"></div>
      </div>
    </div>

    <!-- RAM Card -->
    <div class="card flex-col justify-between">
      <div class="card-header flex items-center justify-between">
        <div class="flex items-center gap-2">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 19v-3"/>
            <path d="M10 19v-3"/>
            <path d="M14 19v-3"/>
            <path d="M18 19v-3"/>
            <path d="M6 8V5"/>
            <path d="M10 8V5"/>
            <path d="M14 8V5"/>
            <path d="M18 8V5"/>
            <rect x="2" y="8" width="20" height="8" rx="2"/>
          </svg>
          <h3>{appStore.t('metric_ram')}</h3>
        </div>
        <span class="badge badge-primary">{formatTemp(appStore.metrics.ram_temp_c, appStore.tempUnit)}</span>
      </div>
      
      <div class="card-body flex items-baseline justify-between my-1">
        <span class="metric-value">{appStore.metrics.ram_usage_pct.toFixed(1)}%</span>
        <span class="metric-sub">{(appStore.metrics.ram_used_mb / 1024).toFixed(1)} / {(appStore.metrics.ram_total_mb / 1024).toFixed(1)} GB</span>
      </div>

      <div class="progress-bar-bg">
        <div class="progress-bar-fill" style="width: {Math.min(appStore.metrics.ram_usage_pct, 100)}%;"></div>
      </div>
    </div>

    <!-- DISK Card -->
    <div class="card flex-col justify-between">
      <div class="card-header flex items-center justify-between">
        <div class="flex items-center gap-2">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="22" y1="12" x2="2" y2="12"/>
            <path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>
            <line x1="6" y1="16" x2="6.01" y2="16"/>
            <line x1="10" y1="16" x2="10.01" y2="16"/>
          </svg>
          <h3>{appStore.t('metric_disk')}</h3>
        </div>
        <span class="badge badge-primary">{formatTemp(appStore.metrics.disk_temp_c, appStore.tempUnit)}</span>
      </div>
      
      <div class="card-body flex items-baseline justify-between my-1">
        <span class="metric-value">{appStore.metrics.disk_usage_pct.toFixed(1)}%</span>
        <span class="metric-sub">{appStore.metrics.disk_used_gb.toFixed(0)} / {appStore.metrics.disk_total_gb.toFixed(0)} GB</span>
      </div>

      <div class="progress-bar-bg">
        <div class="progress-bar-fill" style="width: {Math.min(appStore.metrics.disk_usage_pct, 100)}%;"></div>
      </div>
    </div>
  </div>
</div>

<style>
  .metrics-dashboard-layout {
    display: grid;
    grid-template-columns: 140px 1fr;
    gap: 1.15rem;
    padding: 1.25rem;
    height: 100%;
    box-sizing: border-box;
    overflow: hidden;
  }

  @media (max-width: 768px) {
    .metrics-dashboard-layout {
      grid-template-columns: 1fr;
      overflow-y: auto;
    }
  }

  /* Left Column Canvas Animation Card */
  .mesh-card {
    height: 100%;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background-color: var(--color-bg-card);
    box-shadow: var(--shadow-sm);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .mesh-canvas-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    border-radius: var(--radius-lg);
    background-color: var(--color-bg-primary);
    overflow: hidden;
  }

  .mesh-canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  /* Right Column 4 Stacked Metric Cards */
  .metrics-vertical-stack {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
    height: 100%;
  }

  .card {
    flex: 1;
    background-color: var(--color-bg-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: 0.9rem 1.25rem;
    box-shadow: var(--shadow-sm);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .card-header h3 {
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--color-text-secondary);
  }

  .metric-value {
    font-size: 1.55rem;
    font-weight: 800;
    color: var(--color-brand-primary);
    letter-spacing: -0.02em;
  }

  .metric-sub {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    font-weight: 600;
  }

  .progress-bar-bg {
    width: 100%;
    height: 5px;
    background-color: var(--color-bg-tertiary);
    border-radius: var(--radius-pill);
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    background-color: var(--color-brand-primary);
    border-radius: var(--radius-pill);
  }

  .my-1 {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
  }
</style>
