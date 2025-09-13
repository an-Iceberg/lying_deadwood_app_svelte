<script lang="ts">
import {invoke} from "@tauri-apps/api/core";
import Header from "$lib/Header.svelte";
import Tabs from "$lib/Tabs.svelte";
import {
  is_one_time,
  is_input,
  is_data,
  to_one_time,
  to_input,
  to_data,
} from "$lib/tab_state.svelte";
import {
  today,
  MAX_DIAMETER,
  MAX_DISTANCE,
  MAX_LENGTH,
  MIN_DIAMETER_1,
  MIN_DIAMETER_2,
  MIN_LENGTH_1,
  MIN_LENGTH_2,
} from "$lib/constants.svelte";

// https://svelte.dev/playground/052c877eb34c45ee8f773a8bf8475347?version=5.38.6#H4sIAAAAAAAAA5VVUW_TMBD-K0cAJRVN2iF48ZpKlZgmBGLS2Atq--AkbmuWOMF2ulRZ_ztnO9lSmDZ4qOSc7777vvNnt_UELZhHvM-iqjXsac4zqnkpvLG34TlTHlm2nj5UJscEMN5VLKoqUnuWaxNLqGJPxdNSaCY0wngzlUpe6flKrHTONOhS0xxiOJtOz_sYNywUBpetCa00EqoZgenYfaZlXkoC_uX14odvQscxPJ_5aXH95fvXxc3Fv5fcXCy-usy1JfaGgKqu9I5Jw8zRDjuqUUGroIF4Dk1k4eD-HqajSLKsTlkQ0DEkI7NN4R2uLJ75bWqRmjH3E2cBFxlrRj01M4yCNu5UXmw6ivBoNJNB0IyB234cXsUxONDn2Wi-gcABL23-ulMyf2DwQAuTn0iMHxI7xKOdntM6mzyeu5gpfciZtUBkkUJjEMoFk12HjKsqpwcCm5w1HRyOaCtCrlmhCKRoJya7nZ-1QvqHsLOZqwqVprKncsczvSPGZG_PB6ywv_pVU8naYdpZ9FGyoqvcMb7d6dNYQeWWi1DaHYg-PO4kpcyYDCXNeI0so_em6rShFdwO0wmIUrAOoax1joM4iXWoSal1WaCKqgFVomFA8XzP5BP4YWmN2reh6e1WlrXIws7dWlKhKhQu9Em1LV6aix6LukiYXBMS3rHkluuQCzyfUFWoPKmRiRjDSyUo5rQEWnCU-gxaVYwimbRXDMMR4610gaN1UO8a0b5mNN317wRV0FoLjsHKw6vNraJZxveQ5lSp2P_DZ_7c9RmmOCv4YPvEK-_vsbUOf-XBpK-3uGDlrzw3ANwuuMDPKa6G7X1I8MYQyzV2lI9QCmI34zawl_LxKRgduzazCbLEVTsxss0sXhL2f7LM63h1vfh2efGgbKjLd7L8k44w9NmpsP6ZPAJeY5rkLDOYvQj8O9Cs0R7REuWv8Yvy_A7LPbKhuWLH3zmV9pKNBgAA

let area_group = $state();
let subarea = $state(0);
let start_date = $state("");

let piece_id = $state(0);
let piece_id_error: string | null = $state("Kein Wert vorhanden");

let part_id = $state(1);
let part_id_error = $state("");

let species: number | null = $state(null);
let species_error = $state("Keine valide Spezies");

let d1_max: number | null = $state(null);
let d1_max_error: string | null = $state("Kein Wert vorhanden");
let d1_max_not_measurable = $state(false);

let d2_max: number | null = $state(0.0);
let d2_max_error: string | null = $state("Kein Wert vorhanden");
let d2_max_not_measurable = $state(false);

let azimax = $state(0);
let azimax_error = $state("Kein Wert vorhanden");

let distmax = $state(0.0);
let distmax_error = $state("Kein Wert vorhanden");

let d1_min: number | null = $state(0.0);
let d1_min_error: string | null = $state("Kein Wert vorhanden");
let d1_min_not_measurable = $state(false);

let d2_min: number | null = $state(0.0);
let d2_min_error: string | null = $state("Kein Wert vorhanden");
let d2_min_not_measurable = $state(false);

let azimin = $state(0);
let azimin_error = $state("Kein Wert vorhanden");

let distmin = $state(0.0);
let distmin_error = $state("Kein Wert vorhanden");

let length = $state(0);
let length_error = $state("Kein Wert vorhanden");

let min_vals_error = $state("Keine Werte vorhanden");

let decay = $state(0);

let ref_tree_nr = $state(0);
let ref_tree_nr_error = $state("Kein Wert vorhanden");

let former_tree_nr = $state(0);
let former_tree_nr_error = $state("");

let comments = $state("");

let name = $state("");
let greetMsg = $state("");

async function greet(event: Event) {
  event.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", {name});
}
</script>

<main class="container-fluid">
  <!-- Todo: extract these into components -->
  <Header />

  <Tabs />

  <!-- Todo: extract tabs into components -->

  <!-- Todo: if file exists, these need to be unchangeable -->
  <div id="one_time" class="container text-start" class:d-none={!is_one_time()}>
    <label class="form-label" for="area_group">Flächengruppe:</label>
    <input
      class="form-control mb-4"
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
      type="date"
      name="start_date"
      id="start_date"
      min={today}
      value={today}
      required
    />
  </div>

  <!-- Todo: team -->

  <div id="input_tab" class="container text-start" class:d-none={!is_input()}>
    <label class="form-label" for="piece_id">Stück ID:</label>
    <input
      class="form-control"
      class:mb-4={!piece_id_error}
      bind:value={piece_id}
      onchange={() => {
        if (piece_id < 1) piece_id_error = "Must be > 1";
        else piece_id_error = null;
      }}
      type="number"
      name="piece_id"
      id="piece_id"
      min="1"
      max="999999"
      step="1"
      required
    />
    {#if piece_id_error}
      <div class="alert alert-warning form-text mb-4">{piece_id_error}</div>
    {/if}

    <label for="part_id" class="form-label">Teilstück ID:</label>
    <input
      type="number"
      class="form-control"
      class:mb-4={!part_id_error}
      bind:value={part_id}
      onchange={() => {}}
      id="part_id"
      min="1"
      max="99"
      step="1"
    />
    {#if part_id_error}
      <div class="alert alert-warning form-text mb-4">{part_id_error}</div>
    {/if}

    <!-- help: https://www.w3schools.com/howto/howto_js_filter_dropdown.asp -->

    <label class="form-label" for="species">Baumart:</label>
    <select class="form-select mb-4" id="species">
      <option value="null">null unknown</option>
      <option value="100">100 species</option>
      <option value="101">101 species</option>
      <option value="102">102 species</option>
      <option value="103">103 species</option>
    </select>

    <!-- <label for="cars">Choose a car:</label>
    <select name="cars" id="cars">
      <optgroup label="Swedish Cars">
        <option value="volvo">Volvo</option>
        <option value="saab">Saab</option>
      </optgroup>
      <optgroup label="German Cars">
        <option value="mercedes">Mercedes</option>
        <option value="audi">Audi</option>
      </optgroup>
    </select> -->

    <!-- TODO: add units to unit measurements -->
    <label for="d1_max" class="form-label">Durchmesser 1 max.:</label>
    <input
      class="ms-2"
      bind:checked={d1_max_not_measurable}
      onchange={() => {
        if (d2_max_not_measurable) d2_max_not_measurable = false;
        if (d1_max_not_measurable) d1_max = null;
      }}
      type="checkbox"
      name="d1_max_not_measurable"
      id="d1_max_not_measurable"
    />
    <label for="d1_max_not_measurable">nicht messbar</label>
    <input
      class="form-control"
      class:mb-4={!piece_id_error}
      disabled={d1_max_not_measurable}
      bind:value={d1_max}
      onchange={() => {
        if (d1_max == null) d1_max_error = "Kein Wert vorhanden";
        else if (d1_max > MAX_DIAMETER) d1_max_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
        else if (d1_max < MIN_DIAMETER_1)
          d1_max_error = `Wert muss grösser sein als ${MIN_DIAMETER_1} sein`;
        else d1_max_error = null;
      }}
      type="number"
      name="d1_max"
      id="d1_max"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d1_max_error}
      <div class="alert alert-warning form-text mb-4">{d1_max_error}</div>
    {/if}

    <label for="d2_max" class="form-label">Durchmesser 2 max.:</label>
    <input
      class="ms-2"
      bind:checked={d2_max_not_measurable}
      onchange={() => {
        if (d1_max_not_measurable) d1_max_not_measurable = false;
        if (d2_max_not_measurable) d2_max = null;
      }}
      type="checkbox"
      name="d2_max_not_measurable"
      id="d2_max_not_measurable"
    />
    <label for="d2_max_not_measurable">nicht messbar</label>
    <input
      class="form-control"
      class:mb-4={!d2_max_error}
      disabled={d2_max_not_measurable}
      bind:value={d2_max}
      onchange={() => {
        if (d2_max == null) d2_max_error = "Kein Wert vorhanden";
        else if (d2_max > MAX_DIAMETER) d2_max_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
        else if (d2_max < MIN_DIAMETER_1)
          d2_max_error = `Wert muss grösser als ${MIN_DIAMETER_1} sein`;
        else d2_max_error = null;
      }}
      type="number"
      name="d2_max"
      id="d2_max"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d2_max_error}
      <div class="alert alert-warning form-text mb-4">{d2_max_error}</div>
    {/if}

    <label for="azimax" class="form-label">Azimuth max.:</label>
    <input
      class="form-control"
      class:mb-4={!azimax_error}
      bind:value={azimax}
      onchange={() => {}}
      type="number"
      name="azimax"
      id="azimax"
      min="0"
      max="99"
      step="0.1"
    />
    {#if azimax_error}
      <div class="alert alert-warning form-text mb-4">{azimax_error}</div>
    {/if}

    <label for="distmax" class="form-label">Distanz max.:</label>
    <input
      class="form-control"
      class:mb-4={!distmax_error}
      bind:value={distmax}
      onchange={() => {}}
      type="number"
      name="distmax"
      id="distmax"
      min="0"
      max="99"
      step="0.1"
    />
    {#if distmax_error}
      <div class="alert alert-warning form-text mb-4">{distmax_error}</div>
    {/if}

    <label for="d1_min" class="form-label">Durchmesser 1 min.:</label>
    <input
      class="ms-2"
      bind:checked={d1_min_not_measurable}
      onchange={() => {
        if (d2_min_not_measurable) d2_min_not_measurable = false;
        if (d1_min_not_measurable) d1_min = null;
      }}
      type="checkbox"
      name="d1_min_not_measurable"
      id="d1_min_not_measurable"
    />
    <label for="d1_min_not_measurable">nicht messbar</label>
    <input
      class="form-control"
      class:mb-4={!d1_min_error}
      disabled={d1_min_not_measurable}
      bind:value={d1_min}
      onchange={() => {
        if (d1_min == null) d1_min_error = "Kein Wert vorhanden";
        else if (d1_min > MAX_DIAMETER) d1_min_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
        else if (d1_min < MIN_DIAMETER_1)
          d1_min_error = `Wert muss grösser als ${MIN_DIAMETER_1} sein`;
        else d1_min_error = null;
      }}
      type="number"
      name="d1_min"
      id="d1_min"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d1_min_error}
      <div class="alert alert-warning form-text mb-4">{d1_min_error}</div>
    {/if}

    <label for="d2_min" class="form-label">Durchmesser 2 min.:</label>
    <input
      class="ms-2"
      bind:checked={d2_min_not_measurable}
      onchange={() => {
        if (d1_min_not_measurable) d1_min_not_measurable = false;
        if (d2_min_not_measurable) d2_min = null;
      }}
      type="checkbox"
      name="d2_min_not_measurable"
      id="d2_min_not_measurable"
    />
    <label for="d2_min_not_measurable">nicht messbar</label>
    <input
      class="form-control"
      class:mb-4={!d2_min_error}
      disabled={d2_min_not_measurable}
      bind:value={d2_min}
      onchange={() => {
        if (d2_min == null) d2_min_error = "Kein Wert vorhanden";
        else if (d2_min > MAX_DIAMETER) d2_min_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
        else if (d2_min < MIN_DIAMETER_1)
          d2_min_error = `Wert muss grösser als ${MIN_DIAMETER_1} sein`;
        else d2_min_error = null;
      }}
      type="number"
      name="d2_min"
      id="d2_min"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d2_min_error}
      <div class="alert alert-warning form-text mb-4">{d2_min_error}</div>
    {/if}

    <label for="azimin" class="form-label">Azimuth min.:</label>
    <input
      class="form-control"
      class:mb-4={!azimin_error}
      bind:value={azimax}
      onchange={() => {}}
      type="number"
      name="azimin"
      id="azimin"
      min="0"
      max="99"
      step="0.1"
    />
    {#if azimin_error}
      <div class="alert alert-warning form-text mb-4">{azimin_error}</div>
    {/if}

    <label for="distmin" class="form-label">Distanz min.:</label>
    <input
      class="form-control"
      class:mb-4={!distmin_error}
      bind:value={distmin}
      onchange={() => {}}
      type="number"
      name="distmin"
      id="distmin"
      min="0"
      max="99"
      step="0.1"
    />
    {#if distmax_error}
      <div class="alert alert-warning form-text mb-4">{distmin_error}</div>
    {/if}

    <label for="length" class="form-label">Länge</label>
    <input
      class="form-control"
      class:mb-4={!length_error}
      bind:value={length}
      onchange={() => {}}
      type="number"
      name="length"
      id="length"
      min="0"
      max="99"
      step="0.1"
    />
    {#if length_error}
      <div class="alert alert-warning form-text">{length_error}</div>
    {/if}
    {#if min_vals_error}
      <div class="alert alert-warning from-text mb-4">{min_vals_error}</div>
    {/if}

    <!-- Todo: decay -->

    <label class="form-label" for="ref_tree_nr">Referenz Baum ID:</label>
    <input
      class="form-control"
      class:mb-4={!ref_tree_nr_error}
      bind:value={ref_tree_nr}
      onchange={() => {}}
      type="number"
      name="length"
      id="ref_tree_nr"
      min="1"
      max="999999"
      step="1"
    />
    {#if ref_tree_nr_error}
      <div class="alert alert-warning form-text mb-4">{ref_tree_nr_error}</div>
    {/if}

    <label class="form-label" for="former_tree_nr">Ehem. Baum ID:</label>
    <input
      class="form-control"
      class:mb-4={!former_tree_nr_error}
      bind:value={former_tree_nr}
      onchange={() => {}}
      type="number"
      name="length"
      id="length"
      min="1"
      max="999999"
      step="1"
    />
    {#if former_tree_nr_error}
      <div class="alert alert-warning form-text mb-4">{former_tree_nr_error}</div>
    {/if}

    <label for="comments" class="form-label">Bemerkung(en)</label>
    <textarea name="comments" id="comments" class="form-control mb-4"></textarea>

    <button class="btn btn-primary mb-4 disabled">Daten aufnehmen</button>
  </div>

  <div id="data_tab" class="container text-start" class:d-none={!is_data()}>
    <h1 class="h1">Aufgenommene Daten</h1>
  </div>
</main>

<style>
</style>
