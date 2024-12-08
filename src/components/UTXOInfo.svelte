<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fade } from 'svelte/transition';
  
  let transactionCount: number | null = null;
  let message: string = '';
  let loading = false;

  async function reindexUTXO() {
    loading = true;
    try {
      transactionCount = await invoke("cmd_reindex_tauri");
      message = `UTXO set reindexed successfully`;
    } catch (error) {
      console.error("Error reindexing UTXO:", error);
      message = "Failed to reindex UTXO";
    } finally {
      loading = false;
    }
  }
</script>

<div class="utxo-container">
  <h2>UTXO Management <span class="subtitle">Transaction Index</span></h2>
  
  <button class="cyber-button" on:click={reindexUTXO} disabled={loading}>
    {#if loading}
      <span class="loader"></span>
    {:else}
      <span class="button-content">
        <span class="button-text">Reindex UTXO</span>
        <span class="button-icon">🔄</span>
      </span>
    {/if}
  </button>

  {#if message}
    <div class="message {message.includes('successfully') ? 'success' : 'error'}" transition:fade>
      {message}
    </div>
  {/if}

  {#if transactionCount !== null}
    <div class="utxo-info" transition:fade>
      <h3>UTXO Information</h3>
      <div class="transaction-count">
        <span class="label">Total Transactions:</span>
        <span class="count">{transactionCount}</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .utxo-container {
      background: linear-gradient(145deg, #1a1a1a, #2a2a2a);
      border-radius: 15px;
      padding: 2rem;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
      border: 1px solid rgba(255, 255, 255, 0.1);
      color: #fff;
  }

  h2 {
      color: #00ff88;
      margin-bottom: 1.5rem;
      font-size: 1.5rem;
      text-transform: uppercase;
      letter-spacing: 2px;
      text-align: center;
      text-shadow: 0 0 10px rgba(0, 255, 136, 0.5);
  }

  .subtitle {
      font-size: 0.8em;
      color: #ff61d8;
      opacity: 0.7;
  }

  h3 {
      color: #ff61d8;
      margin: 0 0 1rem 0;
      font-size: 1.2rem;
      text-transform: uppercase;
      letter-spacing: 1px;
  }

  .cyber-button {
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
  }

  .cyber-button:hover:not(:disabled) {
      transform: translateY(-2px);
      box-shadow: 0 0 20px rgba(0, 255, 136, 0.4);
  }

  .cyber-button:disabled {
      opacity: 0.7;
      cursor: not-allowed;
  }

  .utxo-info {
      margin-top: 1.5rem;
      padding: 1.5rem;
      background: rgba(255, 255, 255, 0.05);
      border-radius: 8px;
      border: 1px solid rgba(0, 255, 136, 0.3);
  }

  .transaction-count {
      display: flex;
      align-items: center;
      gap: 1rem;
  }

  .label {
      color: #ff61d8;
      font-size: 1rem;
  }

  .count {
      font-size: 1.5rem;
      font-weight: 600;
      color: #00ff88;
      text-shadow: 0 0 10px rgba(0, 255, 136, 0.5);
  }

  .message {
      margin-top: 1rem;
      padding: 1rem;
      border-radius: 8px;
      font-family: 'Fira Code', monospace;
      animation: fadeIn 0.3s ease;
  }

  .success {
      background: rgba(0, 255, 136, 0.1);
      border: 1px solid rgba(0, 255, 136, 0.3);
      color: #00ff88;
  }

  .error {
      background: rgba(255, 61, 216, 0.1);
      border: 1px solid rgba(255, 61, 216, 0.3);
      color: #ff61d8;
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

  @keyframes fadeIn {
      from { opacity: 0; transform: translateY(-10px); }
      to { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 600px) {
      .utxo-container {
          margin: 1rem;
          padding: 1.5rem;
      }
  }
</style>