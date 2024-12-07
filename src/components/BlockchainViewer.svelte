<!-- In BlockchainViewer.svelte -->
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    
    let chainData: string = '';
    let loading: boolean = false;
  
    async function viewChain() {
      loading = true;
      try {
        chainData = await invoke("cmd_print_chain_tauri");
      } catch (error) {
        console.error("Error viewing chain:", error);
      } finally {
        loading = false;
      }
    }
  </script>
  
  <div class="chain-container">
    <button on:click={viewChain} disabled={loading}>
      {loading ? 'Loading...' : 'View Blockchain'}
    </button>
    
    {#if chainData}
      <pre class="chain-data">{chainData}</pre>
    {/if}
  </div>
  
  <style>
    .chain-container {
      padding: 1rem;
    }
    
    .chain-data {
      background: #f5f5f5;
      padding: 1rem;
      border-radius: 4px;
      white-space: pre-wrap;
      font-family: monospace;
      margin-top: 1rem;
      overflow-x: auto;
    }
  </style>