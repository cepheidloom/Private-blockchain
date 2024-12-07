<!-- UTXOInfo.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  let transactionCount: number | null = null;
  let message: string = '';

  async function reindexUTXO() {
    try {
      transactionCount = await invoke("cmd_reindex_tauri");
      message = `UTXO set reindexed successfully`;
    } catch (error) {
      console.error("Error reindexing UTXO:", error);
      message = "Failed to reindex UTXO";
    }
  }
</script>

<div class="utxo-container">
  <h3>UTXO Management</h3>
  <button on:click={reindexUTXO} class="reindex-btn">
    Reindex UTXO
  </button>

  {#if message}
    <div class="message {message.includes('successfully') ? 'success' : 'error'}">
      {message}
    </div>
  {/if}

  {#if transactionCount !== null}
    <div class="utxo-info">
      <h4>UTXO Information</h4>
      <div class="transaction-count">
        <span class="label">Total Transactions:</span>
        <span class="count">{transactionCount}</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .utxo-container {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  h3 {
    color: #2c3e50;
    margin: 0 0 1rem 0;
    font-size: 1.3rem;
  }

  h4 {
    color: #34495e;
    margin: 0 0 0.5rem 0;
    font-size: 1.1rem;
  }

  .reindex-btn {
    background-color: #2ecc71;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
  }

  .reindex-btn:hover {
    background-color: #27ae60;
  }

  .utxo-info {
    margin-top: 1.5rem;
    padding: 1rem;
    background-color: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #e9ecef;
  }

  .transaction-count {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .label {
    color: #6c757d;
    font-size: 0.9rem;
  }

  .count {
    font-size: 1.2rem;
    font-weight: 600;
    color: #2c3e50;
  }

  .message {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .success {
    background-color: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
  }

  @media (max-width: 768px) {
    .utxo-container {
      padding: 1rem;
    }
    
    .reindex-btn {
      width: 100%;
    }
  }
</style>