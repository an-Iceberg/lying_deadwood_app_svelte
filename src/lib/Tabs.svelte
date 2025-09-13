<script lang="ts">
import {
  is_one_time,
  is_input,
  is_data,
  to_one_time,
  to_input,
  to_data,
} from "$lib/tab_state.svelte";
</script>

<nav class="nav nav-pills nav-fill justify-content-center w-100 gap-2 mb-4">
  <button class="nav-link border border-primary" class:active={is_one_time()} onclick={to_one_time}
    >Einmalige Daten</button
  >
  <button class="nav-link border border-primary" class:active={is_input()} onclick={to_input}
    >Neue Daten</button
  >
  <button class="nav-link border border-primary" class:active={is_data()} onclick={to_data}
    >Aufgenommene Daten</button
  >
</nav>

<style>
</style>
