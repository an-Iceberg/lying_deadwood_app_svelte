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
let part_id_error = $state("Kein Wert vorhanden");
let species = $state(-1);
let species_error = $state("");

let d1_max = $state(0.0);
let d1_max_error = $state("");
let d2_max = $state(0.0);
let d2_max_error = $state("");
let azimax = $state(0);
let azimax_error = $state("");
let distmax = $state(0.0);
let distmax_error = $state("");

let d1_min = $state(0.0);
let d1_min_error = $state("");
let d2_min = $state(0.0);
let d2_min_error = $state("");
let azimin = $state(0);
let azimin_error = $state("");
let distmin = $state(0.0);
let distmin_error = $state("");

let length = $state(0);
let length_error = $state("");
let min_vals_error = $state("");
let decay = $state(0);
let ref_tree_nr = $state(0);
let ref_tree_nr_error = $state("");
let former_tree_nr = $state(0);
let former_tree_nr_error = $state("");

async function greet(event: Event) {
  event.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", {name});
}
</script>

<main class="container-fluid">
  <h1 class="h1">Welcome to Tauri + Svelte</h1>

  <!-- Todo: use CSS:grid for this and split into 3 columns: label, input, error message -->
  <div class="vertical inputs">
    <div class="container text-start">
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

      <label class="form-label" for="piece_id">Stück ID:</label>
      <input
        class="form-control"
        type="number"
        name="piece_id"
        id="piece_id"
        min="1"
        max="999999"
        step="1"
        required
      />
      <div class="alert alert-warning form-text mb-4">{piece_id_error}</div>

      <label for="part_id" class="form-label">Teilstück ID:</label>
      <input type="number" class="form-control" id="part_id" min="1" max="99" step="1" required />
      <p class="alert alert-warning form-text mb-4">{part_id_error}</p>
    </div>

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
