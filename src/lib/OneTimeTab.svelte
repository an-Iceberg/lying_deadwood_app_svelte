<script lang="ts">
import {is_one_time} from "$lib/tab_state.svelte";
import {today} from "$lib/constants.svelte";
import {set_area_group} from "$lib/input_state.svelte";

// Ugly workaround for avoiding type errors
let area_group = $state();
let subarea = $state();
let start_date = $state(today);
</script>

<!-- Todo: if file exists, these need to be unchangeable -->
<div id="one_time" class="container text-start" class:d-none={!is_one_time()}>
  <label class="form-label" for="area_group">Flächengruppe:</label>
  <input
    class="form-control mb-4"
    bind:value={area_group}
    type="number"
    name="area_group"
    id="area_group"
    min="1"
    max="999999"
    step="1"
    required
  />

  <label class="form-label" for="subarea">Subarea:</label>
  <input
    class="form-control mb-4"
    bind:value={subarea}
    type="number"
    name="subarea"
    id="subarea"
    min="1"
    max="999999"
    step="1"
    required
  />

  <label class="form-label" for="start_date">Aufnahmedatum:</label>
  <input
    class="form-control mb-4"
    bind:value={start_date}
    type="date"
    name="start_date"
    id="start_date"
    min={today}
    required
  />

  <!-- Todo: team -->
</div>

<style>
</style>
