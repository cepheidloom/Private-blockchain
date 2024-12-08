<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  let chainData: string = '';
  let loading: boolean = false;

  async function viewChain() {
    loading = true;
    try {
      console.log("Fetching blockchain data...");
      chainData = await invoke("cmd_print_chain_tauri");
    } catch (error) {
      console.error("Error viewing chain:", error);
    } finally {
      loading = false;
    }
  }

  function formatChainData(data: string) {
    return data
      .replace(
        /(Block \d+):/g,
        '<span class="block-header">$1:</span>'
      )
      .replace(
        /(Hash|Timestamp|Previous Hash|Transaction|ID|Inputs|Outputs|TxID|Vout|Signature|Value|PubKeyHash):\s*/g,
        '<span class="property">$1:</span> '
      )
      .replace(
        /(?<=:\s)([\da-fA-F]+|[\d-]+|\{.*?\}|\[.*?\]|[\w\s]+)(?=\n|$)/g,
        '<span class="value">$1</span>'
      );
  }
</script>

<div class="viewer-card">
  <h2>Blockchain Explorer</h2>
  
  <button on:click={viewChain} disabled={loading}>
    {#if loading}
      <span class="loader"></span>
    {:else}
      <span class="button-text">View Blockchain</span>
      <span class="button-icon">🔍</span>
    {/if}
  </button>

  {#if chainData}
    <div class="chain-data-container">
      <pre class="chain-data">{@html formatChainData(chainData)}</pre>
    </div>
  {/if}
</div>

<style>
  .viewer-card {
    background: linear-gradient(145deg, #1a1a1a, #2a2a2a);
    border-radius: 15px;
    padding: 2rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    margin: 1rem 2rem;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  h2 {
    color: #00ff88;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 2px;
  }

  button {
    width: 100%;
    padding: 1rem;
    background: linear-gradient(45deg, #00ff88, #00cc6f);
    border: none;
    border-radius: 8px;
    color: #1a1a1a;
    font-weight: bold;
    font-size: 1.1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  button:hover {
    transform: translateY(-2px);
    box-shadow: 0 5px 15px rgba(0, 255, 136, 0.4);
  }

  button:active {
    transform: translateY(0);
  }

  button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
  }

  .chain-data-container {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    border: 1px solid rgba(0, 255, 136, 0.3);
    padding: 1rem;
    margin-top: 1rem;
    max-height: 600px;
    overflow-y: auto;
  }

  .chain-data {
    font-family: 'Fira Code', monospace;
    white-space: pre-wrap;
    font-size: 0.9rem;
    line-height: 1.5;
    margin: 0;
  }

  .property {
    color: #ff6b6b;
    font-weight: bold;
  }

  .value {
    color: #00ff88;
  }

  .loader {
    width: 20px;
    height: 20px;
    border: 2px solid #1a1a1a;
    border-bottom-color: transparent;
    border-radius: 50%;
    display: inline-block;
    animation: rotation 1s linear infinite;
  }

  @keyframes rotation {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .chain-data-container::-webkit-scrollbar {
    width: 8px;
  }

  .chain-data-container::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
  }

  .chain-data-container::-webkit-scrollbar-thumb {
    background: #00ff88;
    border-radius: 4px;
  }

  block-header {
    color: #ffd700;
    font-weight: bold;
    font-size: 1.1em;
  }

  .property {
    color: #ff6b6b;
    font-weight: bold;
  }

  .value {
    color: #00ff88;
    text-shadow: 0 0 5px rgba(0, 255, 136, 0.2);
  }

  .chain-data {
    color: #ffffff; /* Set default text color */
  }

  @media (max-width: 600px) {
    .viewer-card {
      margin: 1rem;
      padding: 1.5rem;
    }
  }
</style>