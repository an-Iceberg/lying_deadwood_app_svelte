<script lang="ts">
import {invoke} from "@tauri-apps/api/core";

const tree_species = [
  [-1, "unknown"],
  [100, "some species"],
];

const decay_states = [1, 2, 3, 4, 5];

let name = $state("");
let greetMsg = $state("");

let piece_id = $state(0);
let part_id = $state(0);
let species = $state(-1);

let d1_max = $state(0.0);
let d2_max = $state(0.0);
let azimax = $state(0);
let distmax = $state(0.0);

let d1_min = $state(0.0);
let d2_min = $state(0.0);
let azimin = $state(0);
let distmin = $state(0.0);

let length = $state(0);
let decay = $state(0);
let ref_tree_nr = $state(0);
let tree_nr = $state(0);

async function greet(event: Event) {
  event.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", {name});
}
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>

  <label>
    Label
    <input type="number" min="1" max="50" step="0.1" required />
  </label>

  <!-- Todo: use CSS:grid for this and split into 3 columns: label, input, error message -->
  <div class="vertical">
    <div class="horizontal">
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
    </div>

    <div class="horizontal">
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
    </div>

    <div class="horizontal">
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
    </div>

    <div class="horizontal">
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
    </div>

    <div class="horizontal">
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
  </div>
</main>

<style>
.horizontal {
  display: flex;
  flex-direction: row;
  gap: 2rem;
  justify-content: start;
  align-items: center;
}

.vertical {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  justify-content: space-between;
  align-items: center;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
