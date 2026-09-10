<script lang="ts">
  import { resizerState } from '$lib/state/resizer.svelte';

  const results = $derived(resizerState.results);
  const successful = $derived(results ? results.filter((r) => r.success) : []);
  const failed = $derived(results ? results.filter((r) => !r.success) : []);
</script>

{#if results}
  <div
    class="card {failed.length > 0
      ? 'bg-warning/10 border-warning/30'
      : 'bg-success/10 border-success/30'} border p-4 rounded-2xl flex flex-col gap-3"
  >
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="{failed.length > 0 ? 'text-warning' : 'text-success'} text-base font-bold">
          {failed.length === 0 ? '✓' : '⚠'} Created {successful.length} of {results.length} packshots
        </span>
      </div>
      {#if resizerState.outputDirectory && successful.length > 0}
        <button
          class="btn btn-xs btn-outline btn-success"
          onclick={() => resizerState.openOutputFolder()}
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
