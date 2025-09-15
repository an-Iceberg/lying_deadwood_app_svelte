export const today = new Date().toISOString().split("T")[0];

export const MAX_DIAMETER = 300; // [cm]
export const MAX_DISTANCE = 25; // [m]
export const MAX_LENGTH = 50; // [m]
export const MIN_DIAMETER_1 = 7; // [cm]
export const MIN_LENGTH_1 = 2; // [m]
export const MIN_DIAMETER_2 = 36; // [cm]
export const MIN_LENGTH_2 = 0.5; // [m]

// Todo: load from file
export const tree_ids = [
  1, 2, 3, 4, 5, 1200
]

// Todo: load from file
export const tree_species = [
  [-1, "unknown"],
  [100, "some species"],
];

export const team = [
  [9, "firstname lastname"]
]

export const decay_states = [1, 2, 3, 4, 5];
