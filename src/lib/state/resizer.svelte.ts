import type { CanvasSizePreset, FileItem, OutputFormat, ProcessResult } from '$lib/types';
import {
  createFileInputItem,
  executeBatchResize,
  getParentDirectory,
  isImageFile,
  openOutputDirectory,
  pickImageFiles,
  pickOutputDirectory,
} from '$lib/services/resizer';

export class ResizerState {
  files = $state<FileItem[]>([]);
  isDragging = $state<boolean>(false);
  isProcessing = $state<boolean>(false);
  results = $state<ProcessResult[] | null>(null);

  // Live progress
  progressCurrent = $state<number>(0);
  progressTotal = $state<number>(0);
  currentProcessingFile = $state<string>('');

  // Canvas and export options
  canvasSize = $state<CanvasSizePreset>('2000');
  customWidth = $state<number>(2000);
  customHeight = $state<number>(2000);
  bgColor = $state<string>('#FFFFFF');
  paddingPercent = $state<number>(5);
  outputFormat = $state<OutputFormat>('jpg');
  quality = $state<number>(90);

  outputDirectory = $state<string>('');
  isCustomOutputDir = $state<boolean>(false);

  targetWidth = $derived(this.canvasSize === 'custom' ? this.customWidth : parseInt(this.canvasSize, 10));
  targetHeight = $derived(this.canvasSize === 'custom' ? this.customHeight : parseInt(this.canvasSize, 10));

  hasFiles = $derived(this.files.length > 0);

  syncDefaultOutputDir() {
    if (!this.isCustomOutputDir) {
      if (this.hasFiles) {
        const parent = getParentDirectory(this.files[0].path);
        if (parent) {
          this.outputDirectory = `${parent}/resized`;
        }
      } else {
        this.outputDirectory = '';
      }
    }
  }

  addPaths(paths: string[]) {
    const newItems: FileItem[] = paths
      .filter((p) => isImageFile(p))
      .filter((p) => !this.files.some((f) => f.path === p))
      .map((p) => createFileInputItem(p));

    this.files = [...this.files, ...newItems];
    this.syncDefaultOutputDir();
  }

  async browseFiles() {
    const paths = await pickImageFiles();
    if (paths.length > 0) {
      this.addPaths(paths);
    }
  }

  async selectOutputDir() {
    const dir = await pickOutputDirectory();
    if (dir) {
      this.outputDirectory = dir;
      this.isCustomOutputDir = true;
    }
  }

  resetOutputDir() {
    this.isCustomOutputDir = false;
    this.syncDefaultOutputDir();
  }

  async openOutputFolder() {
    if (!this.outputDirectory) return;
    try {
      await openOutputDirectory(this.outputDirectory);
    } catch (err) {
      console.error('Failed to open output directory:', err);
      alert('Could not open folder: ' + String(err));
    }
  }

  removeFile(id: string) {
    this.files = this.files.filter((item) => item.id !== id);
    if (!this.hasFiles) {
      this.results = null;
    }
    this.syncDefaultOutputDir();
  }

  clearFiles() {
    this.files = [];
    this.results = null;
    this.isCustomOutputDir = false;
    this.outputDirectory = '';
    this.progressCurrent = 0;
    this.progressTotal = 0;
    this.currentProcessingFile = '';
  }

  async startProcessing() {
    if (!this.hasFiles || !this.outputDirectory) return;

    this.isProcessing = true;
    this.results = null;
    this.progressCurrent = 0;
    this.progressTotal = this.files.length;
    this.currentProcessingFile = '';

    try {
      const paths = this.files.map((f) => f.path);
      this.results = await executeBatchResize(
        paths,
        {
          canvas_width: this.targetWidth,
          canvas_height: this.targetHeight,
          bg_color_hex: this.bgColor,
          padding_percent: this.paddingPercent,
          output_format: this.outputFormat,
          quality: this.quality,
          output_dir: this.outputDirectory,
          file_prefix: null,
          file_suffix: null,
        },
        (progress) => {
          this.progressCurrent = progress.completed;
          this.progressTotal = progress.total;
          this.currentProcessingFile = progress.current_file;
        }
      );
    } catch (err) {
      console.error('Error during batch resizing:', err);
      alert('Error resizing photos: ' + String(err));
    } finally {
      this.isProcessing = false;
    }
  }
}

export const resizerState = new ResizerState();
