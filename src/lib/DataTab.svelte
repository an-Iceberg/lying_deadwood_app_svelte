<script lang="ts">
import {is_data} from "$lib/tab_state.svelte";
</script>

<div id="data_tab" class="container text-start" class:d-none={!is_data()}>
  <h1 class="h1">Aufgenommene Daten</h1>
</div>

<style>
</style>
