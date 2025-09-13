let area_group = $state();
let subarea = $state(0);
let start_date = $state("");

export function set_area_group(n: number): void {
  area_group = n
}

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

let len = $state(0);
let length_error = $state("Kein Wert vorhanden");

let min_vals_error = $state("Keine Werte vorhanden");

let decay = $state(0);

let ref_tree_nr = $state(0);
let ref_tree_nr_error = $state("Kein Wert vorhanden");

let former_tree_nr = $state(0);
let former_tree_nr_error = $state("");

let comments = $state("");
