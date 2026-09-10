<script lang="ts">
  import Dropzone from '$lib/components/Dropzone.svelte';
  import QueueList from '$lib/components/QueueList.svelte';
  import ResultsSummary from '$lib/components/ResultsSummary.svelte';
  import SettingsPanel from '$lib/components/SettingsPanel.svelte';
  import ProgressCard from '$lib/components/ProgressCard.svelte';
</script>

<div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
  <!-- Left Column: Dropzone, Queue & Results -->
  <div class="lg:col-span-7 flex flex-col gap-4">
    <Dropzone />
    <QueueList />
    <ResultsSummary />
  </div>

  <!-- Right Column: Canvas Settings & Action -->
  <div class="lg:col-span-5 flex flex-col gap-4">
    <SettingsPanel />
    <ProgressCard />
  </div>
</div>
