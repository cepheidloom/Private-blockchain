<!-- BalanceChecker.svelte -->
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    
    export let addresses: string[] = [];
    let selectedAddress: string = '';
    let balance: number | null = null;
    let message: string = '';
  
    async function getBalance() {
      if (!selectedAddress) {
        message = "Please select an address first";
        return;
      }
      try {
        balance = await invoke("cmd_get_balance_tauri", { address: selectedAddress });
        message = `Balance fetched successfully`;
      } catch (error) {
        console.error("Error getting balance:", error);
        message = "Failed to get balance";
      }
    }
  </script>
  
  <div class="balance-container">
    <h3>Check Balance</h3>
    <div class="balance-form">
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
      <button class="balance-btn" on:click={getBalance}>Check Balance</button>
    </div>
    
    {#if balance !== null}
      <div class="balance-display">
        <span class="balance-label">Current Balance:</span>
        <span class="balance-amount">{balance}</span>
      </div>
    {/if}
  </div>


<style>
  .balance-container {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .balance-form {
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
    border-color: #4a90e2;
  }

  .balance-btn {
    background-color: #4a90e2;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
    min-width: 120px;
  }

  .balance-btn:hover {
    background-color: #357abd;
  }

  .balance-display {
    margin-top: 1.5rem;
    padding: 1rem;
    background-color: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #e9ecef;
  }

  .balance-label {
    color: #6c757d;
    margin-right: 0.5rem;
  }

  .balance-amount {
    font-size: 1.2rem;
    font-weight: 600;
    color: #2c3e50;
  }

  @media (max-width: 768px) {
    .balance-form {
      flex-direction: column;
    }
    
    .balance-btn {
      width: 100%;
    }
  }
</style>