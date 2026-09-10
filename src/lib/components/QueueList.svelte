<script lang="ts">
  import { resizerState } from '$lib/state/resizer.svelte';
</script>

{#if resizerState.hasFiles}
  <div class="card bg-base-200/80 shadow-sm p-4 border border-base-300/50">
    <div class="flex items-center justify-between mb-2">
      <span class="text-xs font-semibold uppercase tracking-wider text-base-content/70">
        Queue ({resizerState.files.length}
        {resizerState.files.length === 1 ? 'photo' : 'photos'})
      </span>
    </div>

    <ul class="flex flex-col gap-1.5 max-h-72 overflow-y-auto pr-1">
      {#each resizerState.files as item (item.id)}
        <li
          class="flex items-center justify-between gap-3 bg-base-100 p-2 rounded-xl border border-base-300"
        >
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
            onclick={() => resizerState.removeFile(item.id)}
            disabled={resizerState.isProcessing}
          >
            ✕
          </button>
        </li>
      {/each}
    </ul>
  </div>
{/if}
