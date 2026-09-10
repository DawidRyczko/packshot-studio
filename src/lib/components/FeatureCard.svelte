<script lang="ts">
  let {
    title,
    description,
  }: {
    title: string;
    description: string;
  } = $props();
</script>

<div class="card bg-base-100 p-4 border border-base-300">
  <h3 class="font-bold text-base mb-2">{title}</h3>
  <p class="text-xs text-base-content/70">
    {description}
  </p>
</div>
