<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  export let addresses: string[] = [];
  let genesisAddress: string = '';
  let message: string = '';
  let loading = false;

  async function createBlockchain() {
    if (!genesisAddress) {
      message = "Please select an address for genesis block";
      return;
    }
    loading = true;
    try {
      await invoke("cmd_create_blockchain_tauri", { address: genesisAddress });
      message = "Blockchain created successfully with genesis block";
    } catch (error) {
      console.error("Error creating blockchain:", error);
      message = "Failed to create blockchain";
    } finally {
      loading = false;
    }
  }
</script>

<div class="blockchain-container">
  <h2>Blockchain Creator <span class="subtitle">Genesis Forge</span></h2>
  <div class="blockchain-form">
    <div class="select-wrapper">
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
    </div>
    <button class="cyber-button" on:click={createBlockchain} disabled={loading || !genesisAddress}>
      {#if loading}
        <span class="loader"></span>
      {:else}
        <span class="button-text">Forge Genesis Block</span>
        <span class="button-icon">⚡</span>
      {/if}
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

  .blockchain-form {
    display: flex;
    gap: 1rem;
    margin: 1.5rem 0;
  }

  .select-wrapper {
    position: relative;
    flex: 1;
  }

  .select-wrapper::after {
    content: '▼';
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    color: #00ff88;
    pointer-events: none;
  }

  .address-select {
    width: 100%;
    padding: 0.8rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(0, 255, 136, 0.3);
    border-radius: 8px;
    color: #00ff88;
    font-size: 1rem;
    appearance: none;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .address-select:focus {
    outline: none;
    border-color: #00ff88;
    box-shadow: 0 0 10px rgba(0, 255, 136, 0.2);
  }

  .address-select option {
    background: #1a1a1a;
    color: #00ff88;
  }

  .cyber-button {
    padding: 0.8rem 1.5rem;
    background: linear-gradient(45deg, #00ff88, #00cc6f);
    border: none;
    border-radius: 8px;
    color: #1a1a1a;
    font-weight: bold;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    min-width: 200px;
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

  .message {
    margin-top: 1.5rem;
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
    .blockchain-form {
      flex-direction: column;
    }
    
    .cyber-button {
      width: 100%;
    }
  }
</style>