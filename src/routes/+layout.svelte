<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { resizerState } from '$lib/state/resizer.svelte';
  import Navbar from '$lib/components/Navbar.svelte';

  let { children } = $props();

  onMount(() => {
    let unlisten: (() => void) | undefined;

    async function setupTauriDragDrop() {
      try {
        const webview = getCurrentWebview();
        unlisten = await webview.onDragDropEvent((event) => {
          if (event.payload.type === 'enter' || event.payload.type === 'over') {
            resizerState.isDragging = true;
          } else if (event.payload.type === 'leave') {
            resizerState.isDragging = false;
          } else if (event.payload.type === 'drop') {
            resizerState.isDragging = false;
            if (event.payload.paths && event.payload.paths.length > 0) {
              resizerState.addPaths(event.payload.paths);
            }
          }
        });
      } catch (err) {
        console.warn('Tauri Drag & Drop initialization:', err);
      }
    }

    setupTauriDragDrop();

    return () => {
      if (unlisten) unlisten();
    };
  });
</script>

<main class="min-h-screen bg-base-100 text-base-content p-6 flex flex-col max-w-6xl mx-auto select-none">
  <Navbar />
  {@render children()}
</main>
