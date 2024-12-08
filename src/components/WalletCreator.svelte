<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fade } from "svelte/transition";
  
  let message: string = '';
  let loading = false;
  
  async function createWallet() {
    loading = true;
    try {
      const newAddress = await invoke("cmd_create_wallet_tauri");
      message = `New wallet created with address: ${newAddress}`;
    } catch (error) {
      console.error("Error creating wallet:", error);
      message = "Failed to create wallet";
    } finally {
      loading = false;
    }
  }
</script>

<div class="wallet-container">
  <h2>Wallet Creator <span class="subtitle">Digital Forge</span></h2>
  
  <button class="cyber-button" on:click={createWallet} disabled={loading}>
    {#if loading}
      <span class="loader"></span>
    {:else}
      <span class="button-content">
        <span class="button-text">Create New Wallet</span>
        <span class="button-icon">💎</span>
      </span>
    {/if}
  </button>

  {#if message}
    <div class="message {message.includes('Failed') ? 'error' : 'success'}" transition:fade>
      {message}
    </div>
  {/if}
</div>

<style>
  .wallet-container {
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
    position: relative;
    overflow: hidden;
  }

  .cyber-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 0 20px rgba(0, 255, 136, 0.4);
  }

  .cyber-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .button-content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .message {
    margin-top: 1.5rem;
    padding: 1rem;
    border-radius: 8px;
    font-family: 'Fira Code', monospace;
    animation: fadeIn 0.3s ease;
    word-break: break-all;
  }

  .success {
    background: rgba(0, 255, 136, 0.1);
    border: 1px solid rgba(0, 255, 136, 0.3);
    color: #00ff88;
  }

  .error {
    background: rgba(255, 68, 68, 0.1);
    border: 1px solid rgba(255, 68, 68, 0.3);
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

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 600px) {
    .wallet-container {
      margin: 1rem;
      padding: 1.5rem;
    }
  }
</style>