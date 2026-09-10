<script lang="ts">
  import { goto } from '$app/navigation';
  import { resizerState } from '$lib/state/resizer.svelte';
  import PresetCard from '$lib/components/PresetCard.svelte';
  import type { CanvasSizePreset, OutputFormat } from '$lib/types';

  interface PresetItem {
    badge: string;
    badgeClass: string;
    btnClass: string;
    dimensions: string;
    title: string;
    features: string[];
    canvasSize: CanvasSizePreset;
    bgColor: string;
    paddingPercent: number;
    outputFormat: OutputFormat;
    quality: number;
  }

  const presets: PresetItem[] = [
    {
      badge: 'Amazon',
      badgeClass: 'badge-primary',
      btnClass: 'btn-primary',
      dimensions: '2000×2000 px',
      title: 'Standard Main Packshot',
      features: [
        'Pure White (#FFFFFF) canvas',
        '5% safe inner margin',
        'High-res JPG (90% quality)',
        'Meets zoom requirements',
      ],
      canvasSize: '2000',
      bgColor: '#FFFFFF',
      paddingPercent: 5,
      outputFormat: 'jpg',
      quality: 90,
    },
    {
      badge: 'Allegro / eBay',
      badgeClass: 'badge-secondary',
      btnClass: 'btn-secondary',
      dimensions: '1500×1500 px',
      title: 'Marketplace Balance',
      features: [
        'Pure White (#FFFFFF) canvas',
        '5% safe inner margin',
        'JPG format (85% quality)',
        'Fast loading, crisp details',
      ],
      canvasSize: '1500',
      bgColor: '#FFFFFF',
      paddingPercent: 5,
      outputFormat: 'jpg',
      quality: 85,
    },
    {
      badge: 'Shopify & Web',
      badgeClass: 'badge-accent',
      btnClass: 'btn-accent',
      dimensions: '1000×1000 px',
      title: 'Transparent PNG',
      features: [
        'Transparent background',
        'Lossless PNG format',
        '5% safe inner margin',
        'Adaptive to any store theme',
      ],
      canvasSize: '1000',
      bgColor: 'transparent',
      paddingPercent: 5,
      outputFormat: 'png',
      quality: 90,
    },
  ];

  function applyPreset(options: {
    canvasSize: CanvasSizePreset;
    bgColor: string;
    paddingPercent: number;
    outputFormat: OutputFormat;
    quality: number;
  }) {
    resizerState.canvasSize = options.canvasSize;
    resizerState.bgColor = options.bgColor;
    resizerState.paddingPercent = options.paddingPercent;
    resizerState.outputFormat = options.outputFormat;
    resizerState.quality = options.quality;
    goto('/');
  }
</script>

<div class="flex flex-col gap-6 max-w-4xl mx-auto w-full">
  <div class="flex flex-col gap-1">
    <h2 class="text-xl font-bold">eCommerce Presets & Recommendations</h2>
    <p class="text-sm text-base-content/70">
      Optimized square canvas and margin presets for top online marketplaces.
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    {#each presets as preset (preset.badge)}
      <PresetCard
        title={preset.title}
        badge={preset.badge}
        badgeClass={preset.badgeClass}
        btnClass={preset.btnClass}
        dimensions={preset.dimensions}
        features={preset.features}
        canvasSize={preset.canvasSize}
        bgColor={preset.bgColor}
        paddingPercent={preset.paddingPercent}
        outputFormat={preset.outputFormat}
        quality={preset.quality}
        onApply={applyPreset}
      />
    {/each}
  </div>
</div>
