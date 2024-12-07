<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let addresses: string[] = [];
  let message: string = '';
  let selectedAddress: string = '';
  let balance: number | null = null;

  async function cmd_get_address_wrapp(event: Event) {
    try {
      addresses = await invoke("cmd_get_address_wrapper");
    } catch (error) {
      console.error("Error fetching addresses:", error);
    }
  }

  async function createWallet() {
    try {
      const newAddress = await invoke("cmd_create_wallet_tauri");
      message = `New wallet created with address: ${newAddress}`;
      // Refresh address list
      cmd_get_address_wrapp(new Event('click'));
    } catch (error) {
      console.error("Error creating wallet:", error);
      message = "Failed to create wallet";
    }
  }

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

<main class="container">
  <h1>Wallet Management</h1>
  <div class="buttons">
    <button on:click={cmd_get_address_wrapp}>Get Addresses</button>
    <button on:click={createWallet}>Create New Wallet</button>
  </div>

  {#if message}
    <p class="message">{message}</p>
  {/if}

  <div class="balance-section">
    <select bind:value={selectedAddress}>
      <option value="">Select an address</option>
      {#each addresses as address}
        <option value={address}>{address}</option>
      {/each}
    </select>
    <button on:click={getBalance}>Check Balance</button>
    {#if balance !== null}
      <p class="balance">Current Balance: {balance}</p>
    {/if}
  </div>

  <h2>All Addresses</h2>
  <ul>
    {#each addresses as address}
      <li>{address}</li>
    {/each}
  </ul>
</main>

<style>
  .buttons {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .message {
    color: green;
    margin: 1rem 0;
  }

  .balance-section {
    margin: 1rem 0;
    padding: 1rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  select {
    padding: 0.5rem;
    margin-right: 1rem;
    min-width: 300px;
  }

  .balance {
    margin-top: 1rem;
    font-weight: bold;
  }

  button {
    padding: 0.5rem 1rem;
    border-radius: 4px;
    border: 1px solid #ccc;
    background: #fff;
    cursor: pointer;
  }

  button:hover {
    background: #f0f0f0;
  }
</style>