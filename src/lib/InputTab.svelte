<script lang="ts">
import {is_input} from "$lib/tab_state.svelte";
import {
  MAX_DIAMETER,
  MAX_DISTANCE,
  MAX_LENGTH,
  MIN_DIAMETER_1,
  MIN_DIAMETER_2,
  MIN_LENGTH_1,
  MIN_LENGTH_2,
  tree_species,
} from "$lib/constants.svelte";

// https://svelte.dev/playground/052c877eb34c45ee8f773a8bf8475347?version=5.38.6#H4sIAAAAAAAAA5VVUW_TMBD-K0cAJRVN2iF48ZpKlZgmBGLS2Atq--AkbmuWOMF2ulRZ_ztnO9lSmDZ4qOSc7777vvNnt_UELZhHvM-iqjXsac4zqnkpvLG34TlTHlm2nj5UJscEMN5VLKoqUnuWaxNLqGJPxdNSaCY0wngzlUpe6flKrHTONOhS0xxiOJtOz_sYNywUBpetCa00EqoZgenYfaZlXkoC_uX14odvQscxPJ_5aXH95fvXxc3Fv5fcXCy-usy1JfaGgKqu9I5Jw8zRDjuqUUGroIF4Dk1k4eD-HqajSLKsTlkQ0DEkI7NN4R2uLJ75bWqRmjH3E2cBFxlrRj01M4yCNu5UXmw6ivBoNJNB0IyB234cXsUxONDn2Wi-gcABL23-ulMyf2DwQAuTn0iMHxI7xKOdntM6mzyeu5gpfciZtUBkkUJjEMoFk12HjKsqpwcCm5w1HRyOaCtCrlmhCKRoJya7nZ-1QvqHsLOZqwqVprKncsczvSPGZG_PB6ywv_pVU8naYdpZ9FGyoqvcMb7d6dNYQeWWi1DaHYg-PO4kpcyYDCXNeI0so_em6rShFdwO0wmIUrAOoax1joM4iXWoSal1WaCKqgFVomFA8XzP5BP4YWmN2reh6e1WlrXIws7dWlKhKhQu9Em1LV6aix6LukiYXBMS3rHkluuQCzyfUFWoPKmRiRjDSyUo5rQEWnCU-gxaVYwimbRXDMMR4610gaN1UO8a0b5mNN317wRV0FoLjsHKw6vNraJZxveQ5lSp2P_DZ_7c9RmmOCv4YPvEK-_vsbUOf-XBpK-3uGDlrzw3ANwuuMDPKa6G7X1I8MYQyzV2lI9QCmI34zawl_LxKRgduzazCbLEVTsxss0sXhL2f7LM63h1vfh2efGgbKjLd7L8k44w9NmpsP6ZPAJeY5rkLDOYvQj8O9Cs0R7REuWv8Yvy_A7LPbKhuWLH3zmV9pKNBgAA

let piece_id: number | null = $state(null);
let piece_id_error: string | null = $state("Kein Wert vorhanden");

let part_id: number | null = $state(null);
let part_id_error: string | null = $state(null);

let species = $state(-1);
let species_error = $state("Keine valide Spezies");

let crown_deadwood = $state(false);
let fungi = $state(false);

let d1_max: number | null = $state(null);
let d1_max_error: string | null = $state("Kein Wert vorhanden");
let d1_max_not_measurable = $state(false);

let d2_max: number | null = $state(null);
let d2_max_error: string | null = $state("Kein Wert vorhanden");
let d2_max_not_measurable = $state(false);

let azimax: number | null = $state(null);
let azimax_error: string | null = $state("Kein Wert vorhanden");

let distmax: number | null = $state(null);
let distmax_error: string | null = $state("Kein Wert vorhanden");

let d1_min: number | null = $state(null);
let d1_min_error: string | null = $state("Kein Wert vorhanden");
let d1_min_not_measurable = $state(false);

let d2_min: number | null = $state(null);
let d2_min_error: string | null = $state("Kein Wert vorhanden");
let d2_min_not_measurable = $state(false);

let azimin: number | null = $state(null);
let azimin_error: string | null = $state("Kein Wert vorhanden");

let distmin: number | null = $state(null);
let distmin_error: string | null = $state("Kein Wert vorhanden");

let len: number | null = $state(null);
let length_error: string | null = $state("Kein Wert vorhanden");

let len_and_dia_min_error: string | null = $state("Keine Werte vorhanden");

let decay = $state();

let ref_tree_id: number | null = $state(null);
let ref_tree_id_error: string | null = $state("Kein Wert vorhanden");

let former_tree_id: number | null = $state(null);
let former_tree_id_error: string | null = $state(null);

let comments = $state("");

function check_piece_id() {
  if (piece_id == null) {
    piece_id_error = `Kein Wert vorhanden`;
    return;
  }

  if (piece_id < 1) piece_id_error = `Muss grösser als 1 sein`;
  // Todo: check if ID exists already
  else piece_id_error = null;
}

function check_part_id() {
  // Todo: implement
}

function check_d1_max() {
  if (d1_max_not_measurable) {
    d1_max_error = null;
    return;
  }

  // Check that value is not null or any of its variants
  if (d1_max == null || d1_max == undefined) {
    d1_max_error = `Kein Wert vorhanden`;
    return;
  }

  // Check that it is in range
  if (d1_max > MAX_DIAMETER) {
    d1_max_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
    return;
  } else if (d1_max < MIN_DIAMETER_1) {
    d1_max_error = `Wert muss grösser sein als ${MIN_DIAMETER_1} sein`;
    return;
  }

  // Check that it is larger than the min counterpart
  if (d1_min != null && d1_min != undefined && d1_max < d1_min) {
    d1_max_error = `Max muss grösser sein als Min`;
    return;
  }

  d1_max_error = null;
}

function check_d2_max() {
  if (d2_max_not_measurable) {
    d2_max_error = null;
    return;
  }

  // Check that value is not null or any of its variants
  if (d2_max == null || d2_max == undefined) {
    d2_max_error = `Kein Wert vorhanden`;
    return;
  }

  // Check that it is in range
  if (d2_max > MAX_DIAMETER) {
    d2_max_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
    return;
  } else if (d2_max < MIN_DIAMETER_1) {
    d2_max_error = `Wert muss grösser sein als ${MIN_DIAMETER_1} sein`;
    return;
  }

  // Check that it is larger than the min counterpart
  if (d2_min != null && d2_min != undefined && d2_max < d2_min) {
    d2_max_error = `Max muss grösser sein als Min`;
    return;
  }

  d2_max_error = null;
}

function check_azimax() {
  if (azimax == null || azimax == undefined) {
    azimax_error = `Kein Wert vorhanden`;
  } else if (azimax < 0 || azimax > 400) {
    azimax_error = `Wert muss zwischen 0 und 400 sein`;
  } else {
    azimax_error = null;
  }
}

function check_distmax() {
  if (distmax == null || distmax == undefined) {
    distmax_error = `Kein Wert vorhanden`;
  } else if (distmax < 0 || distmax > MAX_DISTANCE) {
    distmax_error = `Wert muss zwischen 0 und ${MAX_DISTANCE} sein`;
  } else {
    distmax_error = null;
  }
}

function check_d1_min() {
  if (d1_min_not_measurable) {
    d1_min_error = null;
    return;
  }

  // Check that value is not null or any of its variants
  if (d1_min == null) {
    d1_min_error = `Kein Wert vorhanden`;
    return;
  }

  // Check that it is in range
  if (d1_min > MAX_DIAMETER) {
    d1_min_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
    return;
  } else if (d1_min < MIN_DIAMETER_1) {
    d1_min_error = `Wert muss grösser sein als ${MIN_DIAMETER_1} sein`;
    return;
  }

  // Check that it is larger than the min counterpart
  if (d1_max != null && d1_max < d1_min) {
    d1_min_error = `Min muss kleiner sein als Max`;
    return;
  }

  // Check that the additional minimum requirements are met (with length)
  // Todo: this

  if (len != null && (d2_min == null || d2_min < d1_min)) {
    if (len >= MIN_LENGTH_1 && d1_min < MIN_DIAMETER_1) {
      d1_min_error = `Wert muss grösser als ${MIN_DIAMETER_1} sein`;
      return;
    } else if (MIN_LENGTH_1 > len && len >= MIN_LENGTH_2 && d1_min < MIN_DIAMETER_2) {
      d1_min_error = `Wert muss grösser als ${MIN_DIAMETER_2} sein`;
      return;
    }
  }
  // If d2_min > d1_min then d2_min does input validation

  d1_min_error = null;
}

function check_d2_min() {
  if (d2_min_not_measurable) {
    d2_min_error = null;
    return;
  }

  // Check that value is not null or any of its variants
  if (d2_min == null) {
    d2_min_error = `Kein Wert vorhanden`;
    return;
  }

  // Check that it is in range
  if (d2_min > MAX_DIAMETER) {
    d2_min_error = `Wert muss kleiner als ${MAX_DIAMETER} sein`;
    return;
  } else if (d2_min < MIN_DIAMETER_1) {
    d2_min_error = `Wert muss grösser sein als ${MIN_DIAMETER_1} sein`;
    return;
  }

  // Check that it is larger than the min counterpart
  if (d2_max != null && d2_max != undefined && d2_max < d2_min) {
    d2_min_error = `Max muss grösser sein als Min`;
    return;
  }

  // Check that the additional minimum requirements are met (with length)
  // Todo: this

  if (len != null && (d1_min == null || d1_min < d2_min)) {
    if (len >= MIN_LENGTH_1 && d2_min < MIN_DIAMETER_1) {
      d2_min_error = `Wert muss grösser als ${MIN_DIAMETER_1} sein`;
      return;
    } else if (MIN_LENGTH_1 > len && len >= MIN_LENGTH_2 && d2_min < MIN_DIAMETER_2) {
      d2_min_error = `Wert muss grösser als ${MIN_DIAMETER_2} sein`;
      return;
    }
  }
  // If d1_min > d2_min then d1_min does input validation

  d2_min_error = null;
}

function check_azimin() {
  if (azimin == null || azimin == undefined) {
    azimin_error = `Kein Wert vorhanden`;
    return;
  } else if (azimin < 0 || azimin > 400) {
    azimin_error = `Wert muss zwischen 0 und 400 sein`;
  } else {
    azimin_error = null;
  }
}

function check_distmin() {
  if (distmin == null || distmin == undefined) {
    distmin_error = `Kein Wert vorhanden`;
  } else if (distmin < 0 || distmin > MAX_DISTANCE) {
    distmin_error = `Wert muss zwischen 0 und ${MAX_DISTANCE} sein`;
  } else {
    distmin_error = null;
  }
}

function check_length() {
  // Todo: min len and dia combination check
  // Fix: inputting «3» yielsd no-value-error
  if (len == null) {
    length_error = `Kein Wert vorhanden`;
    return;
  }

  if (len < 0 || len > MAX_LENGTH) {
    length_error = `Wert muss zwischen 0 und ${MAX_LENGTH} sein`;
    return;
  }

  if (d1_min != null || d2_min != null) {
    let dia = null;

    if (d1_min == null) dia = d2_min;
    else if (d2_min == null) dia = d1_min;
    else dia = Math.max(d1_min, d2_min);

    if (dia! >= MIN_DIAMETER_2 && len < MIN_LENGTH_2) {
      // First case for thick logs
      length_error = `Wert muss grösser als ${MIN_LENGTH_2} sein`;
      return;
    } else if (MIN_DIAMETER_2 > dia! && dia! >= MIN_DIAMETER_1 && len < MIN_LENGTH_1) {
      // Second case is for long logs
      length_error = `Wert muss grösser als ${MIN_LENGTH_1} sein`;
      return;
    }
  }

  length_error = null;
}

function check_dmin_and_len() {
  check_d1_min();
  check_d2_min();
  check_length();
}

function check_ref_tree_id() {}

function check_former_tree_id() {}

function clear_all_inputs() {}

function is_all_input_valid() {
  return (
    !piece_id_error &&
    !part_id_error &&
    !d1_max_error &&
    !d2_max_error &&
    !azimax &&
    !distmax &&
    !d1_min_error &&
    !d2_min_error &&
    !azimin_error &&
    !distmin_error &&
    !length_error &&
    !len_and_dia_min_error &&
    !ref_tree_id_error &&
    !former_tree_id_error
  );
}

function clamp(value: number, min: number, max: number): number {
  if (value < min) return min;
  else if (value > max) return max;
  else return value;
}

// Note: better validation æsthetics?: https://getbootstrap.com/docs/5.3/forms/validation/
</script>

<div id="input_tab" class="container text-start" class:d-none={!is_input()}>
  <label class="form-label" for="piece_id">Stück ID:</label>
  <!-- Todo: display largest ID here somewhere -->
  <input
    class="form-control"
    class:mb-4={!piece_id_error}
    bind:value={piece_id}
    onchange={() => {
      if (piece_id == null) return;
      if (piece_id < 1) piece_id_error = "Must be > 1";
      // Todo: check if ID exists already
      else piece_id_error = null;
    }}
    type="number"
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
    onchange={/* Todo: this */ () => {}}
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
  <select class="form-select mb-4" id="species" bind:value={species}>
    {#each tree_species as species}
      <option value={species.id}>{`${species.id} ${species.name}`}</option>
    {/each}
  </select>

  <!-- Todo: Kronentotholz -->

  <input type="checkbox" id="crown_deadwood" bind:checked={crown_deadwood} />
  <label class="form-label" for="crown_deadwood">Kronentotholz</label>
  <br />

  <!-- Todo: Fungi -->

  <input type="checkbox" id="fungi" bind:checked={fungi} />
  <label class="form-label mb-0" for="fungi">Pilze</label>

  <!-- Todo: adjust spacing -->
  <hr class="my-4" />

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
  <label>
    <input
      class="ms-2"
      bind:checked={d1_max_not_measurable}
      onchange={() => {
        // d2_max is not measurable and sets d1_max to not measurable
        // One of the 2 diameters has to be measurable
        if (d2_max_not_measurable) {
          d2_max_not_measurable = false;
          d2_max = null;
          check_d2_max();
        }

        if (d1_max_not_measurable) {
          d1_max = null;
          d1_max_error = null;
        } else {
          d1_max = null;
          check_d1_max();
        }
      }}
      type="checkbox"
      id="d1_max_not_measurable"
    />
    nicht messbar
  </label>
  <div class="input-group" class:mb-4={!d1_max_error}>
    <span class="input-group-text font-monospace">cm</span>
    <input
      id="d1_max"
      class="form-control"
      disabled={d1_max_not_measurable}
      bind:value={d1_max}
      onchange={check_d1_max}
      type="number"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if d1_max_error}
    <!-- Todo: add icons: https://getbootstrap.com/docs/5.3/components/alerts/#icons -->
    <div class="alert alert-warning form-text mb-4">{d1_max_error}</div>
  {/if}

  <label for="d2_max" class="form-label">Durchmesser 2 max.:</label>
  <label for="d2_max_not_measurable">
    <input
      class="ms-2"
      bind:checked={d2_max_not_measurable}
      onchange={() => {
        if (d1_max_not_measurable) {
          d1_max_not_measurable = false;
          d1_max = null;
          check_d1_max();
        }

        if (d2_max_not_measurable) {
          d2_max = null;
          d2_max_error = null;
        } else {
          d2_max = null;
          check_d2_max();
        }
      }}
      type="checkbox"
      name="d2_max_not_measurable"
      id="d2_max_not_measurable"
    />
    nicht messbar</label
  >
  <div class="input-group" class:mb-4={!d2_max_error}>
    <!-- ToFix: this glitches out when no error message is displayed (also d1_min && d2_min) -->
    <span class="input-group-text font-monospace">cm</span>
    <input
      id="d2_max"
      class="form-control"
      disabled={d2_max_not_measurable}
      bind:value={d2_max}
      onchange={check_d2_max}
      type="number"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if d2_max_error}
    <div class="alert alert-warning form-text mb-4">{d2_max_error}</div>
  {/if}

  <label for="azimax" class="form-label">Azimuth max.:</label>
  <div class="input-group" class:mb-4={!azimax_error}>
    <span class="input-group-text font-monospace">gon</span>
    <input
      id="azimax"
      class="form-control"
      bind:value={azimax}
      onchange={check_azimax}
      type="number"
      name="azimax"
      min="0"
      max="400"
      step="1"
    />
  </div>
  {#if azimax_error}
    <div class="alert alert-warning form-text mb-4">{azimax_error}</div>
  {/if}

  <label for="distmax" class="form-label">Distanz max.:</label>
  <div class="input-group" class:mb-4={!distmax_error}>
    <span class="input-group-text font-monospace">m</span>
    <input
      id="distmax"
      class="form-control"
      bind:value={distmax}
      onchange={check_distmax}
      type="number"
      name="distmax"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if distmax_error}
    <div class="alert alert-warning form-text mb-4">{distmax_error}</div>
  {/if}

  <button
    class="btn btn-primary mb-4"
    onclick={() => {
      // Swapping the values of 2 variables
      [d1_min, d1_max] = [d1_max, d1_min];
      [d2_min, d2_max] = [d2_max, d2_min];
      // ToFix: azimuth values are not swapped correctly: both are hidden then both are shown
      [azimin, azimax] = [azimax, azimin];
      [distmin, distmax] = [distmax, distmin];
      [d1_min_not_measurable, d1_max_not_measurable] = [
        d1_max_not_measurable,
        d1_min_not_measurable,
      ];
      [d2_min_not_measurable, d2_max_not_measurable] = [
        d2_max_not_measurable,
        d2_min_not_measurable,
      ];

      check_d1_max();
      check_d2_max();
      check_d1_min();
      check_d2_min();
      check_distmax();
      check_distmin();
      check_azimax();
      check_azimin();
      check_dmin_and_len();
    }}>↕ Min und Max vertauschen ↕</button
  >
  <br />

  <label for="d1_min" class="form-label">Durchmesser 1 min.:</label>
  <label for="d1_min_not_measurable">
    <input
      id="d1_min_not_measurable"
      class="ms-2"
      bind:checked={d1_min_not_measurable}
      onchange={() => {
        if (d2_min_not_measurable) {
          d2_min_not_measurable = false;
          d2_min = null;
          check_d2_min();
        }

        if (d1_min_not_measurable) {
          d1_min = null;
          d1_min_error = null;
        } else {
          d1_min = null;
          check_d1_min();
        }
      }}
      type="checkbox"
      name="d1_min_not_measurable"
    />
    nicht messbar</label
  >
  <div class="input-group" class:mb-4={!d1_min_error}>
    <span class="input-group-text font-monospace">cm</span>
    <input
      id="d1_min"
      class="form-control"
      disabled={d1_min_not_measurable}
      bind:value={d1_min}
      onchange={check_dmin_and_len}
      type="number"
      name="d1_min"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if d1_min_error}
    <div class="alert alert-warning form-text mb-4">{d1_min_error}</div>
  {/if}

  <label for="d2_min" class="form-label">Durchmesser 2 min.:</label>
  <label for="d2_min_not_measurable">
    <input
      id="d2_min_not_measurable"
      class="ms-2"
      bind:checked={d2_min_not_measurable}
      onchange={() => {
        if (d1_min_not_measurable) {
          d1_min_not_measurable = false;
          d1_min = null;
          check_d1_min();
        }

        if (d2_min_not_measurable) {
          d2_min = null;
          d2_min_error = null;
        } else {
          d2_min = null;
          check_d2_min();
        }
      }}
      type="checkbox"
      name="d2_min_not_measurable"
    />
    nicht messbar</label
  >
  <div class="input-group" class:mb-4={!d2_min_error}>
    <span class="input-group-text font-monospace">cm</span>
    <input
      id="d2_min"
      class="form-control"
      disabled={d2_min_not_measurable}
      bind:value={d2_min}
      onchange={check_dmin_and_len}
      type="number"
      name="d2_min"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if d2_min_error}
    <div class="alert alert-warning form-text mb-4">{d2_min_error}</div>
  {/if}

  <label for="azimin" class="form-label">Azimuth min.:</label>
  <div class="input-group" class:mb-4={!azimin_error}>
    <span class="input-group-text font-monospace">gon</span>
    <input
      id="azimin"
      class="form-control"
      bind:value={azimin}
      onchange={check_azimin}
      type="number"
      name="azimin"
      min="0"
      max="400"
      step="1"
    />
  </div>
  {#if azimin_error}
    <div class="alert alert-warning form-text mb-4">{azimin_error}</div>
  {/if}

  <label for="distmin" class="form-label">Distanz min.:</label>
  <div class="input-group" class:mb-4={!distmin_error}>
    <span class="input-group-text font-monospace">m</span>
    <input
      id="distmin"
      class="form-control"
      bind:value={distmin}
      onchange={check_distmin}
      type="number"
      name="distmin"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if distmin_error}
    <div class="alert alert-warning form-text mb-4">{distmin_error}</div>
  {/if}

  <hr class="my-4" />

  <label for="length" class="form-label">Länge</label>
  <div class="input-group" class:mb-4={!length_error}>
    <span class="input-group-text font-monospace">m</span>
    <input
      id="length"
      class="form-control"
      bind:value={len}
      onchange={check_dmin_and_len}
      type="number"
      name="length"
      min="0"
      max="99"
      step="0.1"
    />
  </div>
  {#if length_error}
    <div class="alert alert-warning form-text">{length_error}</div>
  {/if}
  <!-- {#if len_and_dia_min_error}
    <div class="alert alert-warning from-text mb-4">{len_and_dia_min_error}</div>
  {/if} -->

  <!-- Todo: decay -->

  <label class="form-label" for="decay">Zerfall/Abbau:</label>
  <select class="form-select mb-4" id="species" bind:value={decay}>
    <option value="1">1</option>
    <option value="2">2</option>
    <option value="3">3</option>
    <option value="4">4</option>
    <option value="5">5</option>
  </select>

  <label class="form-label" for="ref_tree_nr">Referenz Baum ID:</label>
  <input
    class="form-control"
    class:mb-4={!ref_tree_id_error}
    bind:value={ref_tree_id}
    onchange={check_ref_tree_id}
    type="number"
    name="length"
    id="ref_tree_nr"
    min="1"
    max="999999"
    step="1"
  />
  {#if ref_tree_id_error}
    <div class="alert alert-warning form-text mb-4">{ref_tree_id_error}</div>
  {/if}

  <label class="form-label" for="former_tree_nr">Ehem. Baum ID:</label>
  <input
    class="form-control"
    class:mb-4={!former_tree_id_error}
    bind:value={former_tree_id}
    onchange={check_former_tree_id}
    type="number"
    name="length"
    id="length"
    min="1"
    max="999999"
    step="1"
  />
  {#if former_tree_id_error}
    <div class="alert alert-warning form-text mb-4">{former_tree_id_error}</div>
  {/if}

  <label for="comments" class="form-label">Bemerkung(en)</label>
  <textarea name="comments" id="comments" class="form-control mb-4" bind:value={comments}
  ></textarea>

  <button class="btn btn-primary mb-4 disabled">Daten aufnehmen</button>
</div>

<style>
</style>
