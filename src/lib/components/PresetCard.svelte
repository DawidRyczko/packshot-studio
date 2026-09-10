<script lang="ts">
  import type { CanvasSizePreset, OutputFormat } from '$lib/types';

  interface PresetOptions {
    canvasSize: CanvasSizePreset;
    bgColor: string;
    paddingPercent: number;
    outputFormat: OutputFormat;
    quality: number;
  }

  let {
    title,
    badge,
    badgeClass = 'badge-primary',
    btnClass = 'btn-primary',
    dimensions,
    features,
    canvasSize,
    bgColor,
    paddingPercent,
    outputFormat,
    quality,
    onApply,
  }: {
    title: string;
    badge: string;
    badgeClass?: string;
    btnClass?: string;
    dimensions: string;
    features: string[];
    canvasSize: CanvasSizePreset;
    bgColor: string;
    paddingPercent: number;
    outputFormat: OutputFormat;
    quality: number;
    onApply: (options: PresetOptions) => void;
  } = $props();
</script>

<div class="card bg-base-200 border border-base-300 p-5 flex flex-col justify-between gap-4 shadow-sm">
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between">
      <span class="badge {badgeClass} font-bold">{badge}</span>
      <span class="text-xs font-mono text-base-content/60">{dimensions}</span>
    </div>
    <h3 class="font-bold text-base">{title}</h3>
    <ul class="text-xs text-base-content/70 flex flex-col gap-1 list-disc list-inside">
      {#each features as feature}
        <li>{feature}</li>
      {/each}
    </ul>
  </div>
  <button
    class="btn btn-sm {btnClass}"
    onclick={() =>
      onApply({
        canvasSize,
        bgColor,
        paddingPercent,
        outputFormat,
        quality,
      })}
  >
    Apply Preset
  </button>
</div>
