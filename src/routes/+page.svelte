<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { convertFileSrc, invoke, Channel } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  interface FileItem {
    id: string;
    name: string;
    path: string;
    previewUrl: string;
  }

  interface ProcessResult {
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

  interface ProcessProgress {
    completed: number;
    total: number;
    current_file: string;
  }

  let isDragging = $state(false);
  let files = $state<FileItem[]>([]);
  let isProcessing = $state(false);
  let results = $state<ProcessResult[] | null>(null);

  // Live progress state
  let progressCurrent = $state(0);
  let progressTotal = $state(0);
  let currentProcessingFile = $state('');

  // Settings
  let canvasSize = $state<'2000' | '1500' | '1000' | 'custom'>('2000');
  let customWidth = $state(2000);
  let customHeight = $state(2000);
  let bgColor = $state('#FFFFFF');
  let paddingPercent = $state(5); // 5% safe margin
  let outputFormat = $state<'jpg' | 'png' | 'webp'>('jpg');
  let quality = $state(90);

  let outputDirectory = $state<string>('');
  let isCustomOutputDir = $state(false);

  const targetWidth = $derived(canvasSize === 'custom' ? customWidth : parseInt(canvasSize, 10));
  const targetHeight = $derived(canvasSize === 'custom' ? customHeight : parseInt(canvasSize, 10));

  const SUPPORTED_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.heic', '.webp'];

  function isImageFile(name: string): boolean {
    const lower = name.toLowerCase();
    return SUPPORTED_EXTENSIONS.some((ext) => lower.endsWith(ext));
  }

  function getParentDirectory(filePath: string): string {
    const lastSlash = Math.max(filePath.lastIndexOf('/'), filePath.lastIndexOf('\\'));
    return lastSlash > 0 ? filePath.substring(0, lastSlash) : '';
  }

  // Automatically keep default output directory in sync with the first photo's location
  $effect(() => {
    if (!isCustomOutputDir) {
      if (files.length > 0) {
        const parent = getParentDirectory(files[0].path);
        if (parent) {
          outputDirectory = `${parent}/resized`;
        }
      } else {
        outputDirectory = '';
      }
    }
  });

  function addPaths(paths: string[]) {
    const newItems: FileItem[] = paths
      .filter((p) => isImageFile(p))
      .filter((p) => !files.some((f) => f.path === p))
      .map((p) => {
        const name = p.split(/[/\\]/).pop() || p;
        return {
          id: crypto.randomUUID(),
          name,
          path: p,
          previewUrl: convertFileSrc(p)
        };
      });

    files = [...files, ...newItems];
  }

  async function handleBrowseFiles() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: 'Images (JPG, PNG, HEIC, WEBP)',
          extensions: ['jpg', 'jpeg', 'png', 'heic', 'webp']
        }
      ]
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      addPaths(paths);
    }
  }

  async function handleSelectOutputDir() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Destination Folder'
    });

    if (selected && typeof selected === 'string') {
      outputDirectory = selected;
      isCustomOutputDir = true;
    }
  }

  function resetToDefaultOutputDir() {
    isCustomOutputDir = false;
    if (files.length > 0) {
      const parent = getParentDirectory(files[0].path);
      if (parent) {
        outputDirectory = `${parent}/resized`;
      }
    }
  }

  async function handleOpenOutputFolder() {
    if (!outputDirectory) return;
    try {
      await invoke('open_folder', { path: outputDirectory });
    } catch (err) {
      console.error('Failed to open output directory:', err);
      alert('Could not open folder: ' + String(err));
    }
  }

  // Native Tauri drag & drop listener
  onMount(() => {
    let unlisten: (() => void) | undefined;

    async function setupTauriDragDrop() {
      const webview = getCurrentWebview();
      unlisten = await webview.onDragDropEvent((event) => {
        if (event.payload.type === 'enter' || event.payload.type === 'over') {
          isDragging = true;
        } else if (event.payload.type === 'leave') {
          isDragging = false;
        } else if (event.payload.type === 'drop') {
          isDragging = false;
          if (event.payload.paths && event.payload.paths.length > 0) {
            addPaths(event.payload.paths);
          }
        }
      });
    }

    setupTauriDragDrop();

    return () => {
      if (unlisten) unlisten();
    };
  });

  function removeFile(id: string) {
    files = files.filter((item) => item.id !== id);
  }

  function clearFiles() {
    files = [];
    results = null;
    isCustomOutputDir = false;
    outputDirectory = '';
    progressCurrent = 0;
    progressTotal = 0;
    currentProcessingFile = '';
  }

  async function startProcessing() {
    if (files.length === 0 || !outputDirectory) return;
    isProcessing = true;
    results = null;
    progressCurrent = 0;
    progressTotal = files.length;
    currentProcessingFile = '';

    const onProgress = new Channel<ProcessProgress>();
    onProgress.onmessage = (message) => {
      progressCurrent = message.completed;
      progressTotal = message.total;
      currentProcessingFile = message.current_file;
    };

    try {
      const paths = files.map((f) => f.path);
      const res = await invoke<ProcessResult[]>('resize_images', {
        paths,
        options: {
          canvas_width: targetWidth,
          canvas_height: targetHeight,
          bg_color_hex: bgColor,
          padding_percent: paddingPercent,
          output_format: outputFormat,
          quality: quality,
          output_dir: outputDirectory,
          file_prefix: null,
          file_suffix: null
        },
        onProgress
      });

      results = res;
    } catch (err) {
      console.error('Error during batch resizing:', err);
      alert('Error resizing photos: ' + String(err));
    } finally {
      isProcessing = false;
    }
  }
</script>

<main class="min-h-screen bg-base-100 text-base-content p-6 flex flex-col max-w-6xl mx-auto select-none">
  <!-- Top Bar -->
  <header class="flex items-center justify-between border-b border-base-200 pb-4 mb-6">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-xl bg-primary text-primary-content flex items-center justify-center font-black text-lg shadow-sm">
        PS
      </div>
      <div>
        <h1 class="text-xl font-extrabold tracking-tight">Packshot Studio</h1>
        <p class="text-xs text-base-content/60">Batch eCommerce Packshot Generator & Canvas Padding</p>
      </div>
    </div>

    {#if files.length > 0}
      <button class="btn btn-ghost btn-sm text-error" onclick={clearFiles} disabled={isProcessing}>
        Clear all ({files.length})
      </button>
    {/if}
  </header>

  <!-- Two Column Layout -->
  <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
    <!-- Left Column: Dropzone & File List -->
    <div class="lg:col-span-7 flex flex-col gap-4">
      <!-- Drag & Drop Area -->
      <div
        role="region"
        aria-label="Photo dropzone"
        class="border-2 border-dashed rounded-3xl p-8 text-center transition-all duration-200 flex flex-col items-center justify-center gap-3 cursor-pointer
          {isDragging
            ? 'border-primary bg-primary/10 scale-[1.01]'
            : 'border-base-300 hover:border-primary/60 bg-base-200/50 hover:bg-base-200'}"
      >
        <div class="w-14 h-14 rounded-2xl bg-base-100 shadow-sm flex items-center justify-center text-primary pointer-events-none">
          <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
            />
          </svg>
        </div>

        <div class="flex flex-col gap-0.5 pointer-events-none">
          <h2 class="text-base font-bold">Drag & drop product photos here</h2>
          <p class="text-xs text-base-content/60">Supports JPG, JPEG, PNG, HEIC, WEBP</p>
        </div>

        <button class="btn btn-primary btn-sm mt-1 shadow" onclick={handleBrowseFiles} disabled={isProcessing}>
          Browse Files
        </button>
      </div>

      <!-- File Queue List -->
      {#if files.length > 0}
        <div class="card bg-base-200/80 shadow-sm p-4 border border-base-300/50">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-semibold uppercase tracking-wider text-base-content/70">
              Queue ({files.length} {files.length === 1 ? 'photo' : 'photos'})
            </span>
          </div>

          <ul class="flex flex-col gap-1.5 max-h-72 overflow-y-auto pr-1">
            {#each files as item (item.id)}
              <li class="flex items-center justify-between gap-3 bg-base-100 p-2 rounded-xl border border-base-300">
                <div class="flex items-center gap-3 min-w-0 flex-1">
                  <img
                    src={item.previewUrl}
                    alt={item.name}
                    class="w-9 h-9 rounded-lg object-cover bg-base-300 shrink-0"
                  />
                  <span class="truncate text-xs font-mono">{item.name}</span>
                </div>
                <button
                  class="btn btn-ghost btn-circle btn-xs text-error/80 hover:text-error shrink-0"
                  aria-label="Remove photo"
                  onclick={() => removeFile(item.id)}
                  disabled={isProcessing}
                >
                  ✕
                </button>
              </li>
            {/each}
          </ul>
        </div>
      {/if}

      <!-- Results Summary -->
      {#if results}
        {@const successful = results.filter(r => r.success)}
        {@const failed = results.filter(r => !r.success)}

        <div class="card {failed.length > 0 ? 'bg-warning/10 border-warning/30' : 'bg-success/10 border-success/30'} border p-4 rounded-2xl flex flex-col gap-3">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <span class="{failed.length > 0 ? 'text-warning' : 'text-success'} text-base font-bold">
                {failed.length === 0 ? '✓' : '⚠'} Created {successful.length} of {results.length} packshots
              </span>
            </div>
            {#if outputDirectory && successful.length > 0}
              <button
                class="btn btn-xs btn-outline btn-success"
                onclick={handleOpenOutputFolder}
              >
                Open Output Folder
              </button>
            {/if}
          </div>

          {#if failed.length > 0}
            <div class="flex flex-col gap-1 text-xs text-error bg-base-100/60 p-2 rounded-lg">
              <span class="font-bold">Errors occurred on {failed.length} items:</span>
              {#each failed as fail}
                <div class="font-mono text-[11px] truncate">
                  • {fail.input_path.split(/[/\\]/).pop()}: {fail.error}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Right Column: eCommerce Canvas & Output Settings -->
    <div class="lg:col-span-5 flex flex-col gap-4">
      <div class="card bg-base-200/80 border border-base-300/50 p-5 flex flex-col gap-4 shadow-sm">
        <h2 class="text-sm font-bold uppercase tracking-wider text-base-content/70">
          Canvas & Fit Settings
        </h2>

        <!-- Canvas Preset Buttons -->
        <div class="flex flex-col gap-1.5">
          <span class="text-xs font-semibold">Square Canvas Size</span>
          <div class="grid grid-cols-4 gap-1.5">
            <button
              class="btn btn-xs {canvasSize === '2000' ? 'btn-primary' : 'btn-outline'}"
              onclick={() => (canvasSize = '2000')}
              disabled={isProcessing}
            >
              2000×2000
            </button>
            <button
              class="btn btn-xs {canvasSize === '1500' ? 'btn-primary' : 'btn-outline'}"
              onclick={() => (canvasSize = '1500')}
              disabled={isProcessing}
            >
              1500×1500
            </button>
            <button
              class="btn btn-xs {canvasSize === '1000' ? 'btn-primary' : 'btn-outline'}"
              onclick={() => (canvasSize = '1000')}
              disabled={isProcessing}
            >
              1000×1000
            </button>
            <button
              class="btn btn-xs {canvasSize === 'custom' ? 'btn-primary' : 'btn-outline'}"
              onclick={() => (canvasSize = 'custom')}
              disabled={isProcessing}
            >
              Custom
            </button>
          </div>
        </div>

        {#if canvasSize === 'custom'}
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-base-content/70" for="custom-w">Width (px)</label>
              <input
                id="custom-w"
                type="number"
                class="input input-sm input-bordered w-full"
                bind:value={customWidth}
                disabled={isProcessing}
              />
            </div>
            <div>
              <label class="text-xs text-base-content/70" for="custom-h">Height (px)</label>
              <input
                id="custom-h"
                type="number"
                class="input input-sm input-bordered w-full"
                bind:value={customHeight}
                disabled={isProcessing}
              />
            </div>
          </div>
        {/if}

        <!-- Background Color -->
        <div class="flex flex-col gap-1.5">
          <div class="flex justify-between items-center">
            <span class="text-xs font-semibold">Background Color</span>
            <span class="text-xs font-mono">{bgColor}</span>
          </div>
          <div class="flex items-center gap-2">
            <button
              class="btn btn-xs {bgColor === '#FFFFFF' ? 'btn-neutral' : 'btn-outline'}"
              onclick={() => (bgColor = '#FFFFFF')}
              disabled={isProcessing}
            >
              White
            </button>
            <button
              class="btn btn-xs {bgColor === '#F3F4F6' ? 'btn-neutral' : 'btn-outline'}"
              onclick={() => (bgColor = '#F3F4F6')}
              disabled={isProcessing}
            >
              Light Gray
            </button>
            <button
              class="btn btn-xs {bgColor === 'transparent' ? 'btn-neutral' : 'btn-outline'}"
              onclick={() => {
                bgColor = 'transparent';
                if (outputFormat === 'jpg') outputFormat = 'png';
              }}
              disabled={isProcessing}
            >
              Transparent
            </button>
            <input
              type="color"
              class="w-7 h-7 rounded cursor-pointer border border-base-300"
              bind:value={bgColor}
              disabled={isProcessing}
            />
          </div>
        </div>

        <!-- Margin / Padding Slider -->
        <div class="flex flex-col gap-1">
          <div class="flex justify-between items-center">
            <span class="text-xs font-semibold">Inner Safe Margin</span>
            <span class="text-xs font-mono font-bold">{paddingPercent}%</span>
          </div>
          <input
            type="range"
            min="0"
            max="25"
            class="range range-primary range-xs"
            bind:value={paddingPercent}
            disabled={isProcessing}
          />
          <span class="text-[11px] text-base-content/50">Space around product so it doesn't touch canvas edges.</span>
        </div>

        <div class="divider my-0"></div>

        <!-- Output Format & Quality -->
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1">
            <label class="text-xs font-semibold" for="format-select">Format</label>
            <select
              id="format-select"
              class="select select-sm select-bordered w-full"
              bind:value={outputFormat}
              disabled={isProcessing}
            >
              <option value="jpg">JPG (Best for Marketplace)</option>
              <option value="png">PNG (Lossless / Transparent)</option>
              <option value="webp">WEBP (Modern Web)</option>
            </select>
          </div>

          <div class="flex flex-col gap-1">
            <div class="flex justify-between items-center">
              <label class="text-xs font-semibold" for="quality-range">Quality</label>
              <span class="text-xs font-mono">{quality}%</span>
            </div>
            <input
              id="quality-range"
              type="range"
              min="50"
              max="100"
              class="range range-primary range-xs"
              bind:value={quality}
              disabled={outputFormat === 'png' || isProcessing}
            />
          </div>
        </div>

        <!-- Destination Folder -->
        <div class="flex flex-col gap-1">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold">Destination Folder</span>
            {#if isCustomOutputDir}
              <button
                class="btn btn-ghost btn-xs text-primary text-[11px] p-0 h-auto min-h-0"
                onclick={resetToDefaultOutputDir}
              >
                Reset to default
              </button>
            {/if}
          </div>
          <div class="flex gap-2">
            <input
              type="text"
              class="input input-sm input-bordered flex-1 text-xs font-mono truncate"
              placeholder={files.length > 0 ? 'Automatic (source folder / resized)' : 'Add photos to set path...'}
              bind:value={outputDirectory}
              readonly
            />
            <button
              class="btn btn-sm btn-outline"
              onclick={handleSelectOutputDir}
              disabled={isProcessing}
            >
              Browse
            </button>
          </div>
        </div>

        <!-- Action Button & Live Counter -->
        <div class="flex flex-col gap-2 mt-2">
          <button
            class="btn btn-primary w-full shadow"
            disabled={files.length === 0 || !outputDirectory || isProcessing}
            onclick={startProcessing}
          >
            {#if isProcessing}
              <span class="loading loading-spinner loading-xs"></span>
              Generating Packshots...
            {:else}
              Generate {files.length} Packshot{files.length === 1 ? '' : 's'} ({targetWidth}×{targetHeight})
            {/if}
          </button>

          <!-- Live Progress Counter & Bar -->
          {#if isProcessing || (progressTotal > 0 && progressCurrent > 0)}
            <div class="flex flex-col gap-1 bg-base-100 p-2.5 rounded-xl border border-base-300">
              <div class="flex justify-between items-center text-xs">
                <span class="font-semibold text-base-content/80">
                  {#if isProcessing}
                    Processing packshots...
                  {:else}
                    Done!
                  {/if}
                </span>
                <span class="font-mono font-bold text-primary">
                  {progressCurrent} / {progressTotal}
                </span>
              </div>

              <progress
                class="progress progress-primary w-full h-2"
                value={progressCurrent}
                max={progressTotal || 1}
              ></progress>

              {#if currentProcessingFile && isProcessing}
                <span class="text-[11px] font-mono text-base-content/50 truncate">
                  {currentProcessingFile}
                </span>
              {/if}
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
</main>
