import type { Language, Theme, TempUnit, WidgetConfig, SystemMetrics, ProcessItem } from '../types';
import { translate } from '../i18n';

export const defaultWidgetConfig: WidgetConfig = {
  id: 'cacao_widget_default',
  title: 'Chocolate PC Monitor',
  show_cpu: true,
  show_gpu: true,
  show_ram: true,
  show_disk: true,
  show_time: true,
  show_date: true,
  show_process_count: true,
  temp_unit: 'C',
  bg_color: '#3D2314',
  text_color: '#FFFFFF',
  bg_image: '',
  bg_image_size: 'cover',
  bg_image_position: 'center',
  bg_image_opacity: 1.0,
  widget_size: 'medium',
  custom_width: 300,
  custom_height: 220,
  shape: 'rounded',
  font_size: 13,
  font_family: 'Montserrat',
  font_weight: '500',
  widget_opacity: 0.92,
  has_border: true,
  border_color: '#8B5A2B',
  border_opacity: 0.85
};

class AppStore {
  theme = $state<Theme>((localStorage.getItem('cacao_theme') as Theme) || 'dark');
  lang = $state<Language>((localStorage.getItem('cacao_lang') as Language) || 'es');
  tempUnit = $state<TempUnit>((localStorage.getItem('cacao_temp_unit') as TempUnit) || 'C');
  activeTab = $state<'dashboard' | 'processes' | 'widget_studio'>('dashboard');

  metrics = $state<SystemMetrics>({
    cpu_usage: 18.4,
    cpu_temp_c: 44.5,
    gpu_usage: -1.0,
    gpu_temp_c: -1.0,
    ram_usage_pct: 42.8,
    ram_used_mb: 7012,
    ram_total_mb: 16384,
    ram_temp_c: 36.0,
    disk_usage_pct: 58.2,
    disk_used_gb: 298.0,
    disk_total_gb: 512.0,
    disk_temp_c: 39.1,
    process_count: 184
  });

  processes = $state<ProcessItem[]>([]);
  activeWidgetConfig = $state<WidgetConfig>({ ...defaultWidgetConfig });
  savedWidgets = $state<WidgetConfig[]>([]);

  constructor() {
    this.loadSavedWidgets();
    // Apply theme attribute on boot
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', this.theme);
    }
  }

  setTheme(newTheme: Theme) {
    this.theme = newTheme;
    localStorage.setItem('cacao_theme', newTheme);
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', newTheme);
    }
  }

  setLang(newLang: Language) {
    this.lang = newLang;
    localStorage.setItem('cacao_lang', newLang);
  }

  setTempUnit(unit: TempUnit) {
    this.tempUnit = unit;
    localStorage.setItem('cacao_temp_unit', unit);
    this.activeWidgetConfig.temp_unit = unit;
  }

  t(key: string, params?: Record<string, string | number>) {
    return translate(key, this.lang, params);
  }

  loadSavedWidgets() {
    try {
      const stored = localStorage.getItem('cacao_saved_widgets');
      if (stored) {
        this.savedWidgets = JSON.parse(stored);
      } else {
        this.savedWidgets = [{ ...defaultWidgetConfig }];
      }
    } catch {
      this.savedWidgets = [{ ...defaultWidgetConfig }];
    }
  }

  saveCurrentWidget() {
    const idx = this.savedWidgets.findIndex(w => w.id === this.activeWidgetConfig.id);
    if (idx >= 0) {
      this.savedWidgets[idx] = JSON.parse(JSON.stringify(this.activeWidgetConfig));
    } else {
      this.savedWidgets.push(JSON.parse(JSON.stringify(this.activeWidgetConfig)));
    }
    localStorage.setItem('cacao_saved_widgets', JSON.stringify(this.savedWidgets));
  }

  createNewWidget() {
    const newId = 'widget_' + Date.now().toString(36);
    this.activeWidgetConfig = {
      ...defaultWidgetConfig,
      id: newId,
      title: `Widget ${this.savedWidgets.length + 1}`
    };
  }

  deleteWidget(id: string) {
    this.savedWidgets = this.savedWidgets.filter(w => w.id !== id);
    localStorage.setItem('cacao_saved_widgets', JSON.stringify(this.savedWidgets));
    if (this.activeWidgetConfig.id === id && this.savedWidgets.length > 0) {
      this.activeWidgetConfig = { ...this.savedWidgets[0] };
    }
  }
}

export const appStore = new AppStore();

// Temperature Converter Utility
export function formatTemp(tempC: number, unit: TempUnit): string {
  if (unit === 'F') {
    const tempF = (tempC * 9/5) + 32;
    return `${tempF.toFixed(1)}°F`;
  }
  return `${tempC.toFixed(1)}°C`;
}
