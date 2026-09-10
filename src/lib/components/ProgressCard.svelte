<script lang="ts">
  import { resizerState } from '$lib/state/resizer.svelte';
</script>

<div class="flex flex-col gap-2 mt-2">
  <button
    class="btn btn-primary w-full shadow"
    disabled={!resizerState.hasFiles || !resizerState.outputDirectory || resizerState.isProcessing}
    onclick={() => resizerState.startProcessing()}
  >
    {#if resizerState.isProcessing}
      <span class="loading loading-spinner loading-xs"></span>
      Generating Packshots...
    {:else}
      Generate {resizerState.files.length} Packshot{resizerState.files.length === 1 ? '' : 's'} ({resizerState.targetWidth}×{resizerState.targetHeight})
    {/if}
  </button>

  <!-- Live Progress Counter & Bar -->
  {#if resizerState.isProcessing || (resizerState.progressTotal > 0 && resizerState.progressCurrent > 0)}
    <div class="flex flex-col gap-1 bg-base-100 p-2.5 rounded-xl border border-base-300">
      <div class="flex justify-between items-center text-xs">
        <span class="font-semibold text-base-content/80">
          {#if resizerState.isProcessing}
            Processing packshots...
          {:else}
            Done!
          {/if}
        </span>
        <span class="font-mono font-bold text-primary">
          {resizerState.progressCurrent} / {resizerState.progressTotal}
        </span>
      </div>

      <progress
        class="progress progress-primary w-full h-2"
        value={resizerState.progressCurrent}
        max={resizerState.progressTotal || 1}
      ></progress>

      {#if resizerState.currentProcessingFile && resizerState.isProcessing}
        <span class="text-[11px] font-mono text-base-content/50 truncate">
          {resizerState.currentProcessingFile}
        </span>
      {/if}
    </div>
  {/if}
</div>
