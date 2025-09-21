<script lang="ts">
// import {DatePicker} from "date-picker-svelte"; // https://date-picker-svelte.kasper.space/docs
import {is_one_time} from "$lib/tab_state.svelte";
import {today, team as team_members} from "$lib/constants.svelte";
// import {set_area_group} from "$lib/input_state.svelte";

// Todo: if file exists, these attributes must be immutable

let area_group: number | null = $state(null);
let area_group_error_msg: string | null = $state(`Kein Wert vorhanden`);

let subarea: number | null = $state(null);
let subarea_error_msg: string | null = $state(`Kein Wert vorhanden`);

// Todo: if file exists, this needs to be overridden
let start_date = $state(new Date());

let team = $state([]);
let team_error_msg: string | null = $state(`Team muss mindestens 2 Mitglider haben`);

function check_area_group() {
  if (area_group == null) area_group_error_msg = `Kein Wert vorhanden`;
  else if (area_group < 1) area_group_error_msg = `Wert muss grösser als 0 sein`;
  else area_group_error_msg = null;
}

function check_subarea() {
  if (subarea == null) subarea_error_msg = `Kein Wert vorhanden`;
  else if (subarea < 1) subarea_error_msg = `Wert muss grösser als 0 sein`;
  else subarea_error_msg = null;
}

function check_team() {
  if (team.length < 2) team_error_msg = `Team muss mindestens 2 Mitglider haben`;
  else team_error_msg = null;
}
</script>

<!-- Todo: if file exists, these need to be unchangeable -->
<div id="one_time" class="container text-start" class:d-none={!is_one_time()}>
  <label class="form-label" for="area_group">Flächengruppe:</label>
  <input
    class="form-control mb-4"
    class:mb-4={!area_group_error_msg}
    bind:value={area_group}
    onchange={check_area_group}
    type="number"
    id="area_group"
    min="1"
    max="999999"
    step="1"
    required
  />
  {#if area_group_error_msg}
    <div class="alert alert-warning form-text mb-4">{area_group_error_msg}</div>
  {/if}

  <label class="form-label" for="subarea">Teilfläche:</label>
  <input
    class="form-control mb-4"
    class:mb-4={!subarea_error_msg}
    bind:value={subarea}
    onchange={check_subarea}
    type="number"
    id="subarea"
    min="1"
    max="999999"
    step="1"
    required
  />
  {#if subarea_error_msg}
    <div class="alert alert-warning form-text mb-4">{subarea_error_msg}</div>
  {/if}

  <!-- <DatePicker bind:value={start_date} min={start_date} /> -->

  <label class="form-label" for="start_date">Aufnahmedatum:</label>
  <input
    class="form-control mb-4"
    bind:value={start_date}
    type="date"
    id="start_date"
    min={today}
    required
  />

  <p class="form-label">Team</p>
  {#each team_members as member}
    <label class="form-check-label">
      <input type="checkbox" bind:group={team} value={member} onchange={check_team} />
      {member.name} ({member.id})
    </label><br />
  {/each}
  {#if team_error_msg}
    <div class="alert alert-warning form-text mb-4">{team_error_msg}</div>
  {/if}

  <!-- Todo: team -->
</div>

<style>
</style>
