<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  export let addresses: string[] = [];
  let loading = false;
  let message: string = '';

  async function getAddresses() {
    loading = true;
    try {
      addresses = await invoke("cmd_get_address_wrapper");
    } catch (error) {
      console.error("Error fetching addresses:", error);
    } finally {
      loading = false;
    }
  }

  async function copyAddress(address: string) {
    await navigator.clipboard.writeText(address);
    message = 'Address copied!';
    setTimeout(() => message = '', 2000);
  }
</script>

<div class="address-container">
  <h2>Wallet Addresses <span class="subtitle">Collection</span></h2>
  
  <button class="cyber-button" on:click={getAddresses} disabled={loading}>
    {#if loading}
      <span class="loader"></span>
    {:else}
      <span class="button-content">
        <span class="button-text">Fetch Addresses</span>
        <span class="button-icon">📡</span>
      </span>
    {/if}
  </button>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <div class="address-list">
    {#if addresses.length === 0}
      <div class="empty-state">No addresses found</div>
    {:else}
      <ul>
        {#each addresses as address}
          <li class="address-item" on:click={() => copyAddress(address)}>
            <span class="address-text">{address}</span>
            <span class="copy-icon">📋</span>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .address-container {
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
      margin-bottom: 1.5rem;
      position: relative;
      overflow: hidden;
  }

  .cyber-button:hover {
      transform: translateY(-2px);
      box-shadow: 0 0 20px rgba(0, 255, 136, 0.4);
  }

  .cyber-button:active {
      transform: translateY(0);
  }

  .button-content {
      display: flex;
      align-items: center;
      gap: 0.5rem;
  }

  .address-list {
      background: rgba(255, 255, 255, 0.05);
      border-radius: 8px;
      border: 1px solid rgba(0, 255, 136, 0.3);
      padding: 0.5rem;
      max-height: 300px;
      overflow-y: auto;
  }

  .address-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 1rem;
      margin: 0.5rem 0;
      background: rgba(255, 255, 255, 0.03);
      border-radius: 8px;
      cursor: pointer;
      transition: all 0.3s ease;
      font-family: 'Fira Code', monospace;
      color: #00ff88;
  }

  .address-item:hover {
      background: rgba(0, 255, 136, 0.1);
      transform: translateX(5px);
  }

  .copy-icon {
      opacity: 0;
      transition: opacity 0.3s ease;
  }

  .address-item:hover .copy-icon {
      opacity: 1;
  }

  .message {
      background: rgba(0, 255, 136, 0.1);
      color: #00ff88;
      padding: 0.5rem;
      border-radius: 4px;
      text-align: center;
      margin-bottom: 1rem;
      animation: fadeIn 0.3s ease;
  }

  .empty-state {
        color: rgba(255, 255, 255, 0.5);
        text-align: center;
        padding: 2rem;
        font-style: italic;
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

    /* Custom Scrollbar */
    .address-list::-webkit-scrollbar {
        width: 8px;
    }

    .address-list::-webkit-scrollbar-track {
        background: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
    }

    .address-list::-webkit-scrollbar-thumb {
        background: #00ff88;
        border-radius: 4px;
    }

    /* Responsive Design */
    @media (max-width: 600px) {
        .address-container {
            margin: 1rem;
            padding: 1.5rem;
        }

        .address-text {
            font-size: 0.8rem;
            word-break: break-all;
        }
    }

    /* Neon text effect */
    .address-text {
        text-shadow: 0 0 5px rgba(0, 255, 136, 0.5);
        transition: text-shadow 0.3s ease;
    }

    .address-item:hover .address-text {
        text-shadow: 0 0 10px rgba(0, 255, 136, 0.8);
    }

    /* Disabled button state */
    .cyber-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
        transform: none;
    }
</style>

