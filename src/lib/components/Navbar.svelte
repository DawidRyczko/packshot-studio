<script lang="ts">
  import { page } from '$app/state';
  import { resizerState } from '$lib/state/resizer.svelte';
</script>

<header class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 border-b border-base-200 pb-4 mb-6">
  <div class="flex items-center gap-3">
    <img
      src="/favicon.png"
      alt="Packshot Studio logo"
      class="w-10 h-10 rounded-xl shadow-sm object-cover"
    />
    <div>
      <h1 class="text-xl font-extrabold tracking-tight">Packshot Studio</h1>
      <p class="text-xs text-base-content/60">
        Batch eCommerce Packshot Generator & Canvas Padding
      </p>
    </div>
  </div>

  <div class="flex items-center gap-2 w-full sm:w-auto justify-between sm:justify-end">
    <nav class="flex items-center gap-1 bg-base-200 p-1 rounded-xl">
      <a
        href="/"
        class="btn btn-xs rounded-lg {page.url.pathname === '/' ? 'btn-neutral shadow-xs' : 'btn-ghost'}"
      >
        Studio
      </a>
      <a
        href="/presets"
        class="btn btn-xs rounded-lg {page.url.pathname === '/presets' ? 'btn-neutral shadow-xs' : 'btn-ghost'}"
      >
        Presets & Guide
      </a>
      <a
        href="/about"
        class="btn btn-xs rounded-lg {page.url.pathname === '/about' ? 'btn-neutral shadow-xs' : 'btn-ghost'}"
      >
        About
      </a>
    </nav>

    {#if resizerState.hasFiles && page.url.pathname === '/'}
      <button
        class="btn btn-ghost btn-sm text-error"
        onclick={() => resizerState.clearFiles()}
        disabled={resizerState.isProcessing}
      >
        Clear all ({resizerState.files.length})
      </button>
    {/if}
  </div>
</header>
