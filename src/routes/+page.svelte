<script lang="ts">
import {invoke} from "@tauri-apps/api/core";
import Header from "$lib/Header.svelte";
import Tabs from "$lib/Tabs.svelte";
import OneTimeTab from "$lib/OneTimeTab.svelte";
import InputTab from "$lib/InputTab.svelte";
import DataTab from "$lib/DataTab.svelte";

let name = "Pauli";
let greetMsg = $state("");
let result = $state();

async function greet(event: Event) {
  event.preventDefault();
  event.stopPropagation();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", {name});
}

async function inc(event: Event) {
  event.preventDefault();
  event.stopPropagation();
  await invoke("increment", {amount: 10})
    .then(() => (result = "success"))
    .catch(error => (result = `Error: ${error}`));
}
</script>

<!-- Todo: styling: set primary color to WSL logo color -->

<main class="container-fluid">
  <button class="btn btn-primary" onclick={inc}>Increment</button>
  <button class="btn btn-primary" onclick={greet}>Greet</button>
  <p>greetMsg: {greetMsg}</p>
  <p>Result: {result}</p>
  <Header />
  <Tabs />
  <OneTimeTab />
  <InputTab />
  <DataTab />
</main>

<style>
</style>
