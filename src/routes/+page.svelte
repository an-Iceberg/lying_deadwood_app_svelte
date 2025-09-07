<script lang="ts">
import {invoke} from "@tauri-apps/api/core";
// import "../../static/bootstrap.css";
// import "$static/bootstrap.css";

const tree_species = [
  [-1, "unknown"],
  [100, "some species"],
];

const decay_states = [1, 2, 3, 4, 5];

// https://svelte.dev/playground/052c877eb34c45ee8f773a8bf8475347?version=5.38.6#H4sIAAAAAAAAA5VVUW_TMBD-K0cAJRVN2iF48ZpKlZgmBGLS2Atq--AkbmuWOMF2ulRZ_ztnO9lSmDZ4qOSc7777vvNnt_UELZhHvM-iqjXsac4zqnkpvLG34TlTHlm2nj5UJscEMN5VLKoqUnuWaxNLqGJPxdNSaCY0wngzlUpe6flKrHTONOhS0xxiOJtOz_sYNywUBpetCa00EqoZgenYfaZlXkoC_uX14odvQscxPJ_5aXH95fvXxc3Fv5fcXCy-usy1JfaGgKqu9I5Jw8zRDjuqUUGroIF4Dk1k4eD-HqajSLKsTlkQ0DEkI7NN4R2uLJ75bWqRmjH3E2cBFxlrRj01M4yCNu5UXmw6ivBoNJNB0IyB234cXsUxONDn2Wi-gcABL23-ulMyf2DwQAuTn0iMHxI7xKOdntM6mzyeu5gpfciZtUBkkUJjEMoFk12HjKsqpwcCm5w1HRyOaCtCrlmhCKRoJya7nZ-1QvqHsLOZqwqVprKncsczvSPGZG_PB6ywv_pVU8naYdpZ9FGyoqvcMb7d6dNYQeWWi1DaHYg-PO4kpcyYDCXNeI0so_em6rShFdwO0wmIUrAOoax1joM4iXWoSal1WaCKqgFVomFA8XzP5BP4YWmN2reh6e1WlrXIws7dWlKhKhQu9Em1LV6aix6LukiYXBMS3rHkluuQCzyfUFWoPKmRiRjDSyUo5rQEWnCU-gxaVYwimbRXDMMR4610gaN1UO8a0b5mNN317wRV0FoLjsHKw6vNraJZxveQ5lSp2P_DZ_7c9RmmOCv4YPvEK-_vsbUOf-XBpK-3uGDlrzw3ANwuuMDPKa6G7X1I8MYQyzV2lI9QCmI34zawl_LxKRgduzazCbLEVTsxss0sXhL2f7LM63h1vfh2efGgbKjLd7L8k44w9NmpsP6ZPAJeY5rkLDOYvQj8O9Cs0R7REuWv8Yvy_A7LPbKhuWLH3zmV9pKNBgAA

let name = $state("");
let greetMsg = $state("");

let area_group = $state(0);
let subarea = $state(0);
let start_date = $state("");

let piece_id = $state(0);
let piece_id_error = $state("Kein Wert vorhanden");
let part_id = $state(0);
let part_id_error = $state("");
let species = $state(-1);
let species_error = $state("Keine valide Spezies");

let d1_max = $state(0.0);
let d1_max_error = $state("Kein Wert vorhanden");
let d2_max = $state(0.0);
let d2_max_error = $state("Kein Wert vorhanden");
let azimax = $state(0);
let azimax_error = $state("Kein Wert vorhanden");
let distmax = $state(0.0);
let distmax_error = $state("Kein Wert vorhanden");

let d1_min = $state(0.0);
let d1_min_error = $state("Kein Wert vorhanden");
let d2_min = $state(0.0);
let d2_min_error = $state("Kein Wert vorhanden");
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

function piece_id_validator() {
  console.log(`piece_id: ${piece_id}`);
}

async function greet(event: Event) {
  event.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", {name});
}
</script>

<main class="container-fluid">
  <h1 class="h1">Welcome to Tauri + Svelte</h1>

  <ul class="nav nav-pills">
    <li class="nav-item">
      <a class="nav-link active" href="#">Einmalige Daten</a>
    </li>
    <li class="nav-item">
      <a href="#" class="nav-link">Neue Daten</a>
    </li>
    <li class="nav-item">
      <a href="#" class="nav-link">Aufgenommene Daten</a>
    </li>
  </ul>

  <div class="one-time-data container text-start">
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
      min={new Date().toISOString().split("T")[0]}
      value={new Date().toISOString().split("T")[0]}
      required
    />
  </div>

  <!-- Todo: team -->

  <div class="input container text-start">
    <label class="form-label" for="piece_id">Stück ID:</label>
    <input
      class="form-control"
      class:mb-4={piece_id_error == ""}
      bind:value={piece_id}
      onchange={piece_id_validator}
      onkeypress={piece_id_validator}
      type="number"
      name="piece_id"
      id="piece_id"
      min="1"
      max="999999"
      step="1"
      required
    />
    {#if piece_id_error != ""}
      <div class="alert alert-warning form-text mb-4">{piece_id_error}</div>
    {/if}

    <label for="part_id" class="form-label">Teilstück ID:</label>
    <input
      type="number"
      class="form-control"
      class:mb-4={part_id_error == ""}
      id="part_id"
      min="1"
      max="99"
      step="1"
    />
    {#if part_id_error != ""}
      <div class="alert alert-warning form-text mb-4">{part_id_error}</div>
    {/if}

    <!-- help: https://www.w3schools.com/howto/howto_js_filter_dropdown.asp -->
    <label class="form-label" for="species">Spezies:</label>
    <div class="dropdown mb-4" id="species">
      <button class="btn btn-secondary dropdown-toggle" type="button" data-bs-toggle="species"
        >Spezies</button
      >
      <ul class="dropdown-menu">
        <li class="dropdown-item">100 something</li>
        <li class="dropdown-item">101 something</li>
        <li class="dropdown-item">102 something</li>
        <li class="dropdown-item">103 something</li>
      </ul>
    </div>

    <!-- <label for="species" class="form-label">Spezies:</label>
    <select id="species" class="dropdown-menu drowdown-toggle" required>
      <option class="dropdown-item" value="-1">-1 unknown</option>
      <option class="dropdown-item" value="100">100 species</option>
      <option class="dropdown-item" value="101">101 species</option>
      <option class="dropdown-item" value="102">102 species</option>
      <option class="dropdown-item" value="103">103 species</option>
    </select> -->

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

    <label for="d1_max" class="form-label">Durchmesser 1 max.:</label>
    <input
      class="form-control"
      type="number"
      name="d1_max"
      id="d1_max"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d1_max_error != ""}
      <div class="alert alert-warning form-text mb-4">{d1_max_error}</div>
    {/if}

    <label for="d2_max" class="form-label">Durchmesser 2 max.:</label>
    <input
      class="form-control"
      type="number"
      name="d2_max"
      id="d2_max"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d2_max_error != ""}
      <div class="alert alert-warning form-text mb-4">{d2_max_error}</div>
    {/if}

    <label for="azimax" class="form-label">Azimuth max.:</label>
    <input
      class="form-control"
      type="number"
      name="azimax"
      id="azimax"
      min="0"
      max="99"
      step="0.1"
    />
    {#if azimax_error != ""}
      <div class="alert alert-warning form-text mb-4">{azimax_error}</div>
    {/if}

    <label for="distmax" class="form-label">Distanz max.:</label>
    <input
      class="form-control"
      type="number"
      name="distmax"
      id="distmax"
      min="0"
      max="99"
      step="0.1"
    />
    {#if distmax_error != ""}
      <div class="alert alert-warning form-text mb-4">{distmax_error}</div>
    {/if}

    <label for="d1_min" class="form-label">Durchmesser 1 min.:</label>
    <input
      class="form-control"
      type="number"
      name="d1_min"
      id="d1_min"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d1_min_error != ""}
      <div class="alert alert-warning form-text mb-4">{d1_min_error}</div>
    {/if}

    <label for="d2_min" class="form-label">Durchmesser 2 min.:</label>
    <input
      class="form-control"
      type="number"
      name="d2_min"
      id="d2_min"
      min="0"
      max="99"
      step="0.1"
    />
    {#if d2_min_error != ""}
      <div class="alert alert-warning form-text mb-4">{d2_min_error}</div>
    {/if}

    <label for="azimin" class="form-label">Azimuth min.:</label>
    <input
      class="form-control"
      type="number"
      name="azimin"
      id="azimin"
      min="0"
      max="99"
      step="0.1"
    />
    {#if azimin_error != ""}
      <div class="alert alert-warning form-text mb-4">{azimin_error}</div>
    {/if}

    <label for="distmin" class="form-label">Distanz min.:</label>
    <input
      class="form-control"
      type="number"
      name="distmin"
      id="distmin"
      min="0"
      max="99"
      step="0.1"
    />
    {#if distmax_error != ""}
      <div class="alert alert-warning form-text mb-4">{distmin_error}</div>
    {/if}

    <label for="length" class="form-label">Länge</label>
    <input
      class="form-control"
      type="number"
      name="length"
      id="length"
      min="0"
      max="99"
      step="0.1"
    />
    {#if length_error != ""}
      <div class="alert alert-warning form-text">{length_error}</div>
    {/if}
    {#if min_vals_error != ""}
      <div class="alert alert-warning from-text mb-4">{min_vals_error}</div>
    {/if}

    <!-- Todo: decay -->

    <label class="form-label" for="ref_tree_nr">Referenz Baum ID:</label>
    <input
      class="form-control"
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
      type="number"
      name="length"
      id="length"
      min="1"
      max="999999"
      step="1"
    />
    {#if former_tree_nr_error != ""}
      <div class="alert alert-warning form-text mb-4">{former_tree_nr_error}</div>
    {/if}
  </div>

  <div class="data container text-start"></div>

  <!-- Todo: use CSS:grid for this and split into 3 columns: label, input, error message -->
  <div class="vertical inputs">
    <div class="container text-start"></div>

    <label>
      Year of data collection:
      <input id="year" type="number" min="2025" step="1" required />
    </label>

    <label>
      Area ID:
      <input id="area_id" type="number" min="0" step="1" required />
    </label>

    <label>
      Area ID
      <input type="number" min="0" step="1" required />
    </label>

    <label>
      Piece ID:
      <input id="piece_id" type="number" min="1" step="1" required />
    </label>

    <label>
      Part ID
      <input id="part_id" type="number" min="1" step="1" required />
    </label>

    <label>
      Species:
      <select id="species" required>
        <option value="-1">-1 unknown</option>
        <option value="100">100 species</option>
        <option value="101">101 species</option>
        <option value="102">102 species</option>
        <option value="103">103 species</option>
      </select>
    </label>

    <label>
      Diameter 1 max.:
      <input id="d1_max" type="number" min="0" step="0.1" required />
      cm
    </label>

    <label>
      Diameter 2 max.:
      <input id="d2_max" type="number" min="0" step="0.1" required />
      cm
    </label>

    <label>
      Azimuth max.:
      <input id="azimax" type="number" min="0" max="400" step="0.5" required />
      gon
    </label>

    <label>
      Distance max.:
      <input id="distmax" type="number" min="0" step="0.1" required />
      m
    </label>

    <label>
      Diameter 1 min.:
      <input id="d1_min" type="number" min="0" step="0.1" required />
      cm
    </label>

    <label>
      Diameter 2 min.:
      <input id="d2_min" type="number" min="0" step="0.1" required />
      cm
    </label>

    <label>
      Azimuth min.:
      <input id="azimin" type="number" min="0" max="400" step="0.5" required />
      gon
    </label>

    <label>
      Distance min.:
      <input id="distmin" type="number" min="0" step="0.1" required />
      m
    </label>

    <label>
      Length:
      <input id="length" type="number" min="0" step="0.1" required />
      m
    </label>

    <label>
      Decay:
      <select id="decay" required>
        <option value="1">1</option>
        <option value="2">2</option>
        <option value="3">3</option>
        <option value="4">4</option>
        <option value="5">5</option>
      </select>
    </label>

    <label>
      Reference Tree ID.:
      <input id="ref_tree_id" type="number" min="1" step="1" required />
    </label>

    <label>
      Former Tree Nr.:
      <input id="former_id" type="number" min="1" step="1" />
    </label>
  </div>
</main>

<style>
</style>
