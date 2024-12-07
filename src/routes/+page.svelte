<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let addresses: string[] = [];

  async function cmd_get_address_wrapp(event: Event) {
    // event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    try {
    addresses = await invoke("cmd_get_address_wrapper");
    } catch (error) {
    console.error("Error fetching addresses:", error);
    }
  }
</script>

<main class="container">
  <h1>Get Address</h1>
  <button on:click={cmd_get_address_wrapp}>Get Address</button>
  <ul>
    {#each addresses as address}
      <li>{address}</li>
    {/each}
  </ul>
</main>

<style>
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
}
</style> 