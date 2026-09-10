export interface FileItem {
  id: string;
  name: string;
  path: string;
  previewUrl: string;
}

export interface ProcessResult {
  input_path: string;
  output_path: string;
  success: boolean;
  error?: string;
  original_width: number;
  original_height: number;
  output_width: number;
  output_height: number;
  output_size_bytes: number;
}

export interface ProcessProgress {
  completed: number;
  total: number;
  current_file: string;
}

export type CanvasSizePreset = '2000' | '1500' | '1000' | 'custom';
export type OutputFormat = 'jpg' | 'png' | 'webp';

export interface ResizeBackendOptions {
  canvas_width: number;
  canvas_height: number;
  bg_color_hex: string;
  padding_percent: number;
  output_format: OutputFormat;
  quality: number;
  output_dir: string;
  file_prefix: string | null;
  file_suffix: string | null;
}
