<script lang="ts">
  import { resizerState } from '$lib/state/resizer.svelte';
</script>

<div
  role="region"
  aria-label="Photo dropzone"
  class="border-2 border-dashed rounded-3xl p-8 text-center transition-all duration-200 flex flex-col items-center justify-center gap-3 cursor-pointer
    {resizerState.isDragging
    ? 'border-primary bg-primary/10 scale-[1.01]'
    : 'border-base-300 hover:border-primary/60 bg-base-200/50 hover:bg-base-200'}"
>
  <div
    class="w-14 h-14 rounded-2xl bg-base-100 shadow-sm flex items-center justify-center text-primary pointer-events-none"
  >
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

  <button
    class="btn btn-primary btn-sm mt-1 shadow"
    onclick={() => resizerState.browseFiles()}
    disabled={resizerState.isProcessing}
  >
    Browse Files
  </button>
</div>
