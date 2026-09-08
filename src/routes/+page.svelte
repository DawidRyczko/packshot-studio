<script lang="ts">
  interface ImageItem {
    id: string;
    file: File;
    name: string;
    size: number;
    previewUrl: string;
    originalWidth?: number;
    originalHeight?: number;
  }

  let isDragging = $state(false);
  let images = $state<ImageItem[]>([]);
  let targetWidth = $state<number>(1920);
  let targetHeight = $state<number>(1080);
  let maintainAspectRatio = $state(true);
  let quality = $state(85);
  let outputFormat = $state<'jpeg' | 'png' | 'webp'>('jpeg');

  function addFiles(fileList: FileList | File[]) {
    const newItems: ImageItem[] = [];
    Array.from(fileList).forEach((file) => {
      if (!file.type.startsWith('image/')) return;
      const previewUrl = URL.createObjectURL(file);
      const item: ImageItem = {
        id: crypto.randomUUID(),
        file,
        name: file.name,
        size: file.size,
        previewUrl
      };

      // Read image dimensions
      const img = new Image();
      img.onload = () => {
        item.originalWidth = img.width;
        item.originalHeight = img.height;
      };
      img.src = previewUrl;

      newItems.push(item);
    });

    images = [...images, ...newItems];
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    if (e.dataTransfer?.files) {
      addFiles(e.dataTransfer.files);
    }
  }

  function handleFileSelect(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files) {
      addFiles(target.files);
      target.value = '';
    }
  }

  function removeImage(id: string) {
    const item = images.find((img) => img.id === id);
    if (item) {
      URL.revokeObjectURL(item.previewUrl);
    }
    images = images.filter((img) => img.id !== id);
  }

  function clearAll() {
    images.forEach((img) => URL.revokeObjectURL(img.previewUrl));
    images = [];
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

<div class="min-h-screen bg-base-100 text-base-content p-6 flex flex-col gap-6 max-w-6xl mx-auto">
  <!-- Header -->
  <header class="flex items-center justify-between border-b border-base-200 pb-4">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-xl bg-primary text-primary-content flex items-center justify-center font-bold text-lg shadow">
        PR
      </div>
      <div>
        <h1 class="text-xl font-bold">Photo Resizer</h1>
        <p class="text-xs text-base-content/60">Batch resize & convert photos with high performance</p>
      </div>
    </div>

    {#if images.length > 0}
      <button class="btn btn-ghost btn-sm text-error" onclick={clearAll}>
        Clear all ({images.length})
      </button>
    {/if}
  </header>

  <!-- Main Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
    <!-- Left / Center: Upload and Image List -->
    <div class="lg:col-span-2 flex flex-col gap-4">
      <!-- Dropzone -->
      <div
        role="region"
        aria-label="Photo dropzone"
        class="border-2 border-dashed rounded-2xl p-8 text-center transition-all flex flex-col items-center justify-center gap-3
          {isDragging
            ? 'border-primary bg-primary/10 scale-[1.01]'
            : 'border-base-300 hover:border-primary/50 bg-base-200/50'}"
        ondragover={(e) => {
          e.preventDefault();
          isDragging = true;
        }}
        ondragleave={() => (isDragging = false)}
        ondrop={handleDrop}
      >
        <div class="w-14 h-14 rounded-full bg-base-100 shadow flex items-center justify-center text-primary">
          <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </div>
        <div>
          <p class="font-semibold text-base">Drag & drop photos here</p>
          <p class="text-xs text-base-content/60 mt-0.5">Supports JPG, PNG, WEBP, AVIF</p>
        </div>

        <label class="btn btn-primary btn-sm mt-1 cursor-pointer">
          Browse Files
          <input type="file" accept="image/*" multiple class="hidden" onchange={handleFileSelect} />
        </label>
      </div>

      <!-- Image Queue / List -->
      {#if images.length > 0}
        <div class="card bg-base-200 shadow-sm p-4">
          <h2 class="text-sm font-semibold text-base-content/80 mb-3 flex items-center justify-between">
            <span>Selected Photos</span>
            <span class="badge badge-neutral badge-sm">{images.length}</span>
          </h2>
          <div class="flex flex-col gap-2 max-h-96 overflow-y-auto pr-1">
            {#each images as item (item.id)}
              <div class="flex items-center gap-3 bg-base-100 p-2.5 rounded-xl border border-base-300">
                <img src={item.previewUrl} alt={item.name} class="w-12 h-12 rounded-lg object-cover bg-base-200" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-medium truncate">{item.name}</p>
                  <p class="text-xs text-base-content/60">
                    {formatBytes(item.size)}
                    {#if item.originalWidth && item.originalHeight}
                      • {item.originalWidth}×{item.originalHeight}px
                    {/if}
                  </p>
                </div>
                <button
                  class="btn btn-ghost btn-circle btn-xs text-error"
                  aria-label="Remove image"
                  onclick={() => removeImage(item.id)}
                >
                  ✕
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- Right: Resize & Output Settings -->
    <div class="card bg-base-200 shadow-sm p-5 flex flex-col gap-4">
      <h2 class="text-base font-bold">Resize Settings</h2>

      <!-- Dimensions -->
      <div class="grid grid-cols-2 gap-3">
        <div class="form-control">
          <label class="label py-1" for="width-input">
            <span class="label-text text-xs">Width (px)</span>
          </label>
          <input
            id="width-input"
            type="number"
            class="input input-bordered input-sm w-full"
            bind:value={targetWidth}
          />
        </div>
        <div class="form-control">
          <label class="label py-1" for="height-input">
            <span class="label-text text-xs">Height (px)</span>
          </label>
          <input
            id="height-input"
            type="number"
            class="input input-bordered input-sm w-full"
            bind:value={targetHeight}
          />
        </div>
      </div>

      <div class="form-control">
        <label class="label cursor-pointer justify-start gap-2 py-1">
          <input type="checkbox" class="checkbox checkbox-sm checkbox-primary" bind:checked={maintainAspectRatio} />
          <span class="label-text text-xs">Maintain aspect ratio</span>
        </label>
      </div>

      <!-- Format -->
      <div class="form-control">
        <label class="label py-1" for="format-select">
          <span class="label-text text-xs font-medium">Output Format</span>
        </label>
        <select id="format-select" class="select select-bordered select-sm w-full" bind:value={outputFormat}>
          <option value="jpeg">JPEG (.jpg)</option>
          <option value="png">PNG (.png)</option>
          <option value="webp">WEBP (.webp)</option>
        </select>
      </div>

      <!-- Quality Slider -->
      <div class="form-control">
        <div class="flex justify-between items-center py-1">
          <label class="label-text text-xs font-medium" for="quality-range">Quality</label>
          <span class="text-xs font-mono font-semibold">{quality}%</span>
        </div>
        <input
          id="quality-range"
          type="range"
          min="10"
          max="100"
          class="range range-primary range-xs"
          bind:value={quality}
        />
      </div>

      <!-- Action Button -->
      <div class="mt-2 pt-2 border-t border-base-300">
        <button
          class="btn btn-primary w-full shadow"
          disabled={images.length === 0}
        >
          {images.length > 1 ? `Resize ${images.length} Photos` : 'Resize Photo'}
        </button>
      </div>
    </div>
  </div>
</div>
