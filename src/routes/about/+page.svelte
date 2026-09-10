<script lang="ts">
  import FeatureCard from '$lib/components/FeatureCard.svelte';

  interface FeatureItem {
    title: string;
    description: string;
  }

  const features: FeatureItem[] = [
    {
      title: '⚡ Multi-threaded Engine',
      description:
        'Powered by a native Rust backend with parallel Lanczos3 scaling, multi-threading with Rayon, and SIMD hardware acceleration.',
    },
    {
      title: '🖼️ Supported Formats',
      description:
        'Supports JPG, JPEG, PNG, HEIC, and WEBP inputs. Exports directly to JPG, PNG, or WEBP with custom compression.',
    },
    {
      title: '📐 Aspect Ratio Preservation',
      description:
        'Keeps original photo proportions intact while padding into standard square canvas formats with custom background colors and safety margins.',
    },
    {
      title: '🔒 Privacy & Offline',
      description:
        '100% offline desktop application. Your product photos are processed locally on your device and never uploaded to cloud servers.',
    },
  ];
</script>

<div class="card bg-base-200 border border-base-300 p-6 flex flex-col gap-6 max-w-3xl mx-auto shadow-sm">
  <div class="flex items-center gap-4">
    <img
      src="/favicon.png"
      alt="Packshot Studio logo"
      class="w-16 h-16 rounded-2xl shadow object-cover"
    />
    <div>
      <h2 class="text-2xl font-black tracking-tight">Packshot Studio</h2>
      <p class="text-sm text-base-content/70">
        High-Performance Native Batch Image Resizer & Canvas Padder
      </p>
    </div>
  </div>

  <div class="divider my-0"></div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
    {#each features as feature (feature.title)}
      <FeatureCard title={feature.title} description={feature.description} />
    {/each}
  </div>

  <div class="flex justify-end pt-2">
    <a href="/" class="btn btn-primary btn-sm">
      ← Back to Studio
    </a>
  </div>
</div>
