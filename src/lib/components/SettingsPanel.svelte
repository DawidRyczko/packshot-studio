<script lang="ts">
  import { resizerState } from '$lib/state/resizer.svelte';
</script>

<div class="card bg-base-200/80 border border-base-300/50 p-5 flex flex-col gap-4 shadow-sm">
  <h2 class="text-sm font-bold uppercase tracking-wider text-base-content/70">
    Canvas & Fit Settings
  </h2>

  <!-- Canvas Preset Buttons -->
  <div class="flex flex-col gap-1.5">
    <span class="text-xs font-semibold">Square Canvas Size</span>
    <div class="grid grid-cols-4 gap-1.5">
      <button
        class="btn btn-xs {resizerState.canvasSize === '2000' ? 'btn-primary' : 'btn-outline'}"
        onclick={() => (resizerState.canvasSize = '2000')}
        disabled={resizerState.isProcessing}
      >
        2000×2000
      </button>
      <button
        class="btn btn-xs {resizerState.canvasSize === '1500' ? 'btn-primary' : 'btn-outline'}"
        onclick={() => (resizerState.canvasSize = '1500')}
        disabled={resizerState.isProcessing}
      >
        1500×1500
      </button>
      <button
        class="btn btn-xs {resizerState.canvasSize === '1000' ? 'btn-primary' : 'btn-outline'}"
        onclick={() => (resizerState.canvasSize = '1000')}
        disabled={resizerState.isProcessing}
      >
        1000×1000
      </button>
      <button
        class="btn btn-xs {resizerState.canvasSize === 'custom' ? 'btn-primary' : 'btn-outline'}"
        onclick={() => (resizerState.canvasSize = 'custom')}
        disabled={resizerState.isProcessing}
      >
        Custom
      </button>
    </div>
  </div>

  {#if resizerState.canvasSize === 'custom'}
    <div class="grid grid-cols-2 gap-2">
      <div>
        <label class="text-xs text-base-content/70" for="custom-w">Width (px)</label>
        <input
          id="custom-w"
          type="number"
          class="input input-sm input-bordered w-full"
          bind:value={resizerState.customWidth}
          disabled={resizerState.isProcessing}
        />
      </div>
      <div>
        <label class="text-xs text-base-content/70" for="custom-h">Height (px)</label>
        <input
          id="custom-h"
          type="number"
          class="input input-sm input-bordered w-full"
          bind:value={resizerState.customHeight}
          disabled={resizerState.isProcessing}
        />
      </div>
    </div>
  {/if}

  <!-- Background Color -->
  <div class="flex flex-col gap-1.5">
    <div class="flex justify-between items-center">
      <span class="text-xs font-semibold">Background Color</span>
      <span class="text-xs font-mono">{resizerState.bgColor}</span>
    </div>
    <div class="flex items-center gap-2">
      <button
        class="btn btn-xs {resizerState.bgColor === '#FFFFFF' ? 'btn-neutral' : 'btn-outline'}"
        onclick={() => (resizerState.bgColor = '#FFFFFF')}
        disabled={resizerState.isProcessing}
      >
        White
      </button>
      <button
        class="btn btn-xs {resizerState.bgColor === '#F3F4F6' ? 'btn-neutral' : 'btn-outline'}"
        onclick={() => (resizerState.bgColor = '#F3F4F6')}
        disabled={resizerState.isProcessing}
      >
        Light Gray
      </button>
      <button
        class="btn btn-xs {resizerState.bgColor === 'transparent' ? 'btn-neutral' : 'btn-outline'}"
        onclick={() => {
          resizerState.bgColor = 'transparent';
          if (resizerState.outputFormat === 'jpg') resizerState.outputFormat = 'png';
        }}
        disabled={resizerState.isProcessing}
      >
        Transparent
      </button>
      <input
        type="color"
        aria-label="Custom background color"
        class="w-7 h-7 rounded cursor-pointer border border-base-300"
        bind:value={resizerState.bgColor}
        disabled={resizerState.isProcessing}
      />
    </div>
  </div>

  <!-- Margin / Padding Slider -->
  <div class="flex flex-col gap-1">
    <div class="flex justify-between items-center">
      <span class="text-xs font-semibold">Inner Safe Margin</span>
      <span class="text-xs font-mono font-bold">{resizerState.paddingPercent}%</span>
    </div>
    <input
      type="range"
      min="0"
      max="25"
      class="range range-primary range-xs"
      bind:value={resizerState.paddingPercent}
      disabled={resizerState.isProcessing}
    />
    <span class="text-[11px] text-base-content/50">
      Space around product so it doesn't touch canvas edges.
    </span>
  </div>

  <div class="divider my-0"></div>

  <!-- Output Format & Quality -->
  <div class="grid grid-cols-2 gap-3">
    <div class="flex flex-col gap-1">
      <label class="text-xs font-semibold" for="format-select">Format</label>
      <select
        id="format-select"
        class="select select-sm select-bordered w-full"
        bind:value={resizerState.outputFormat}
        disabled={resizerState.isProcessing}
      >
        <option value="jpg">JPG (Best for Marketplace)</option>
        <option value="png">PNG (Lossless / Transparent)</option>
        <option value="webp">WEBP (Modern Web)</option>
      </select>
    </div>

    <div class="flex flex-col gap-1">
      <div class="flex justify-between items-center">
        <label class="text-xs font-semibold" for="quality-range">Quality</label>
        <span class="text-xs font-mono">{resizerState.quality}%</span>
      </div>
      <input
        id="quality-range"
        type="range"
        min="50"
        max="100"
        class="range range-primary range-xs"
        bind:value={resizerState.quality}
        disabled={resizerState.outputFormat === 'png' || resizerState.isProcessing}
      />
    </div>
  </div>

  <!-- Destination Folder -->
  <div class="flex flex-col gap-1">
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold">Destination Folder</span>
      {#if resizerState.isCustomOutputDir}
        <button
          class="btn btn-ghost btn-xs text-primary text-[11px] p-0 h-auto min-h-0"
          onclick={() => resizerState.resetOutputDir()}
        >
          Reset to default
        </button>
      {/if}
    </div>
    <div class="flex gap-2">
      <input
        type="text"
        class="input input-sm input-bordered flex-1 text-xs font-mono truncate"
        placeholder={resizerState.hasFiles
          ? 'Automatic (source folder / resized)'
          : 'Add photos to set path...'}
        bind:value={resizerState.outputDirectory}
        readonly
      />
      <button
        class="btn btn-sm btn-outline"
        onclick={() => resizerState.selectOutputDir()}
        disabled={resizerState.isProcessing}
      >
        Browse
      </button>
    </div>
  </div>
</div>
