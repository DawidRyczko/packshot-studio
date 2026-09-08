<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  interface FileItem {
    id: string;
    name: string;
    path: string;
    previewUrl: string;
  }

  let isDragging = $state(false);
  let files = $state<FileItem[]>([]);

  const SUPPORTED_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.heic', '.webp'];

  function isImageFile(name: string): boolean {
    const lower = name.toLowerCase();
    return SUPPORTED_EXTENSIONS.some((ext) => lower.endsWith(ext));
  }

  function addPaths(paths: string[]) {
    const newItems: FileItem[] = paths
      .filter((p) => isImageFile(p))
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
  }
</script>

<main class="min-h-screen bg-base-100 text-base-content flex flex-col items-center justify-center p-6 select-none">
  <div class="w-full max-w-xl flex flex-col gap-4">
    <!-- Drag & Drop Area -->
    <div
      role="region"
      aria-label="Photo dropzone"
      class="border-2 border-dashed rounded-3xl p-12 text-center transition-all duration-200 flex flex-col items-center justify-center gap-4 cursor-pointer
        {isDragging
          ? 'border-primary bg-primary/10 scale-[1.02]'
          : 'border-base-300 hover:border-primary/60 bg-base-200/50 hover:bg-base-200'}"
    >
      <div class="w-16 h-16 rounded-2xl bg-base-100 shadow-md flex items-center justify-center text-primary pointer-events-none">
        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
          />
        </svg>
      </div>

      <div class="flex flex-col gap-1 pointer-events-none">
        <h2 class="text-lg font-bold">Drag & drop photos here</h2>
        <p class="text-sm text-base-content/60">Supports JPG, JPEG, PNG, HEIC, WEBP</p>
      </div>

      <button class="btn btn-primary btn-sm mt-2 shadow" onclick={handleBrowseFiles}>
        Browse Files
      </button>
    </div>

    <!-- Status & Files List with Preview -->
    {#if files.length > 0}
      <div class="flex flex-col gap-2 bg-base-200 p-4 rounded-2xl text-sm shadow-sm">
        <div class="flex items-center justify-between border-b border-base-300 pb-2">
          <span class="font-semibold text-base-content">
            {files.length} {files.length === 1 ? 'photo' : 'photos'} ready
          </span>
          <button class="btn btn-ghost btn-xs text-error" onclick={clearFiles}>
            Clear all
          </button>
        </div>

        <ul class="flex flex-col gap-2 max-h-60 overflow-y-auto pr-1">
          {#each files as item (item.id)}
            <li class="flex items-center justify-between gap-3 bg-base-100 p-2 rounded-xl border border-base-300">
              <div class="flex items-center gap-3 min-w-0 flex-1">
                <img
                  src={item.previewUrl}
                  alt={item.name}
                  class="w-10 h-10 rounded-lg object-cover bg-base-300 shrink-0"
                />
                <span class="truncate text-xs font-mono">{item.name}</span>
              </div>
              <button
                class="btn btn-ghost btn-circle btn-xs text-error/80 hover:text-error shrink-0"
                aria-label="Remove photo"
                onclick={() => removeFile(item.id)}
              >
                ✕
              </button>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
</main>
