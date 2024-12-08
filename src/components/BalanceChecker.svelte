<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  export let addresses: string[] = [];
  let selectedAddress: string = '';
  let balance: number | null = null;
  let message: string = '';
  let loading = false;

  async function getBalance() {
    if (!selectedAddress) {
      message = "Please select an address first";
      return;
    }
    loading = true;
    try {
      balance = await invoke("cmd_get_balance_tauri", { address: selectedAddress });
      message = `Balance fetched successfully`;
    } catch (error) {
      console.error("Error getting balance:", error);
      message = "Failed to get balance";
    } finally {
      loading = false;
    }
  }
</script>

<div class="balance-container">
  <h2>Balance Checker <span class="subtitle">Crypto Vault</span></h2>
  
  <div class="balance-form">
    <div class="select-wrapper">
      <select 
        class="address-select" 
        bind:value={selectedAddress}
        aria-label="Select wallet address"
      >
        <option value="">Select an address</option>
        {#each addresses as address}
          <option value={address}>{address}</option>
        {/each}
      </select>
    </div>
    
    <button class="cyber-button" on:click={getBalance} disabled={loading || !selectedAddress}>
      {#if loading}
        <span class="loader"></span>
      {:else}
        <span class="button-content">
          <span class="button-text">Check Balance</span>
          <span class="button-icon">💰</span>
        </span>
      {/if}
    </button>
  </div>
  
  {#if message}
    <div class="message" class:error={message.includes('Failed')}>
      {message}
    </div>
  {/if}
  
  {#if balance !== null}
    <div class="balance-display" in:fade>
      <div class="balance-header">Current Balance</div>
      <div class="balance-amount">{balance}</div>
      <div class="balance-decoration"></div>
    </div>
  {/if}
</div>

<style>
  .balance-container {
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
  }

  .subtitle {
      font-size: 0.8em;
      opacity: 0.7;
  }

  .balance-form {
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
      min-width: 150px;
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

  .balance-display {
      background: rgba(255, 255, 255, 0.05);
      border-radius: 8px;
      padding: 1.5rem;
      margin-top: 1.5rem;
      position: relative;
      overflow: hidden;
      border: 1px solid rgba(0, 255, 136, 0.3);
  }

  .balance-header {
      color: #00ff88;
      font-size: 0.9rem;
      text-transform: uppercase;
      letter-spacing: 1px;
      margin-bottom: 0.5rem;
  }

  .balance-amount {
      font-size: 2.5rem;
      font-weight: bold;
      color: #00ff88;
      text-shadow: 0 0 10px rgba(0, 255, 136, 0.5);
  }

  .balance-decoration {
      position: absolute;
      right: -50px;
      bottom: -50px;
      width: 100px;
      height: 100px;
      background: linear-gradient(45deg, transparent, rgba(0, 255, 136, 0.1));
      border-radius: 50%;
  }

  .message {
      padding: 0.8rem;
      border-radius: 8px;
      margin-top: 1rem;
      background: rgba(0, 255, 136, 0.1);
      color: #00ff88;
      text-align: center;
  }

  .message.error {
      background: rgba(255, 0, 0, 0.1);
      color: #ff4444;
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

  @media (max-width: 600px) {
      .balance-form {
          flex-direction: column;
      }
      
      .cyber-button {
          width: 100%;
      }
  }
</style>