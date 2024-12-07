<!-- WalletCreator.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  let message: string = '';
  
  async function createWallet() {
    try {
      const newAddress = await invoke("cmd_create_wallet_tauri");
      message = `New wallet created with address: ${newAddress}`;
    } catch (error) {
      console.error("Error creating wallet:", error);
      message = "Failed to create wallet";
    }
  }
</script>

<div class="wallet-container">
  <h3>Create Wallet</h3>
  <button class="create-btn" on:click={createWallet}>
    Create New Wallet
  </button>
  {#if message}
    <div class="message {message.includes('Failed') ? 'error' : 'success'}">
      {message}
    </div>
  {/if}
</div>

<style>
  .wallet-container {
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

  .create-btn {
    background-color: #3498db;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
  }

  .create-btn:hover {
    background-color: #2980b9;
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

  .error {
    background-color: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
  }

  @media (max-width: 768px) {
    .wallet-container {
      padding: 1rem;
    }
    
    .create-btn {
      width: 100%;
    }
  }
</style>