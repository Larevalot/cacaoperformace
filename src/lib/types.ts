export type Language = 'en' | 'es' | 'it' | 'ja' | 'zh';
export type Theme = 'dark' | 'light';
export type TempUnit = 'C' | 'F';

export interface SystemMetrics {
  cpu_usage: number;
  cpu_temp_c: number;
  gpu_usage: number;
  gpu_temp_c: number;
  ram_usage_pct: number;
  ram_used_mb: number;
  ram_total_mb: number;
  ram_temp_c: number;
  disk_usage_pct: number;
  disk_used_gb: number;
  disk_total_gb: number;
  disk_temp_c: number;
  process_count: number;
}

export interface ProcessItem {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_mb: number;
  status: string;
  exe_path: string;
  user_id: string;
}

export interface ProcessDetails {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_bytes: number;
  memory_mb: number;
  status: string;
  exe_path: string;
  command: string[];
  parent_pid: number | null;
  start_time_secs: number;
  user_id: string;
}

export interface WidgetConfig {
  id: string;
  title: string;
  
  // Parameters
  show_cpu: boolean;
  show_gpu: boolean;
  show_ram: boolean;
  show_disk: boolean;
  show_time: boolean;
  show_date: boolean;
  show_process_count: boolean;
  temp_unit: TempUnit;

  // Styling
  bg_color: string;
  text_color: string;
  bg_image: string;
  bg_image_size: 'cover' | 'contain' | 'auto';
  bg_image_position: string;
  bg_image_opacity: number;
  widget_size: 'small' | 'medium' | 'large' | 'custom';
  custom_width: number;
  custom_height: number;
  shape: 'square' | 'rounded' | 'capsule';
  font_size: number; // px
  font_family: string;
  font_weight: '300' | '400' | '500' | '600' | '700' | '800';
  widget_opacity: number; // 0.1 to 1.0
  has_border: boolean;
  border_color: string;
  border_opacity: number; // 0.0 to 1.0
}
