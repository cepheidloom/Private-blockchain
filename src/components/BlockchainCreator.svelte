<!-- BlockchainCreator.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  export let addresses: string[] = [];
  let genesisAddress: string = '';
  let message: string = '';

  async function createBlockchain() {
    if (!genesisAddress) {
      message = "Please select an address for genesis block";
      return;
    }
    try {
      await invoke("cmd_create_blockchain_tauri", { address: genesisAddress });
      message = "Blockchain created successfully with genesis block";
    } catch (error) {
      console.error("Error creating blockchain:", error);
      message = "Failed to create blockchain";
    }
  }
</script>

<div class="blockchain-container">
  <h3>Create Blockchain</h3>
  <div class="blockchain-form">
    <select 
      class="address-select" 
      bind:value={genesisAddress}
      aria-label="Select genesis address"
    >
      <option value="">Select genesis address</option>
      {#each addresses as address}
        <option value={address}>{address}</option>
      {/each}
    </select>
    <button on:click={createBlockchain} class="create-blockchain-btn">
      Create Blockchain
    </button>
  </div>
  
  {#if message}
    <div class="message {message.includes('successfully') ? 'success' : 'error'}">
      {message}
    </div>
  {/if}
</div>

<style>
  .blockchain-container {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .blockchain-form {
    display: flex;
    gap: 1rem;
    margin: 1rem 0;
  }

  h3 {
    color: #2c3e50;
    margin: 0 0 1rem 0;
    font-size: 1.3rem;
  }

  .address-select {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
    background-color: #fff;
    transition: border-color 0.2s;
  }

  .address-select:focus {
    outline: none;
    border-color: #9b59b6;
  }

  .create-blockchain-btn {
    background-color: #9b59b6;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
    min-width: 160px;
  }

  .create-blockchain-btn:hover {
    background-color: #8e44ad;
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
    .blockchain-form {
      flex-direction: column;
    }
    
    .create-blockchain-btn {
      width: 100%;
    }
  }
</style>