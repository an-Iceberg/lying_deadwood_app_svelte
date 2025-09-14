<script lang="ts">
import {invoke} from "@tauri-apps/api/core";
import Header from "$lib/Header.svelte";
import Tabs from "$lib/Tabs.svelte";
import OneTimeTab from "$lib/OneTimeTab.svelte";
import InputTab from "$lib/InputTab.svelte";
import DataTab from "$lib/DataTab.svelte";

let name = $state("");
let greetMsg = $state("");

async function greet(event: Event) {
  event.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg = await invoke("greet", {name});
}
</script>

<!-- Todo: styling: set primary color to WSL logo color -->

<main class="container-fluid">
  <Header />
  <Tabs />
  <OneTimeTab />
  <InputTab />
  <DataTab />
</main>

<style>
</style>
