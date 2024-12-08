<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    let port = '';
    let address = '';
    let isLoading = false;
    let error = '';
  
    async function startMiner() {
      isLoading = true;
      error = '';
      
      try {
        // Create a timeout to prevent infinite waiting
        const timeoutPromise = new Promise((_, reject) => {
          setTimeout(() => reject(new Error('Operation timed out')), 10000);
        });
        
        // Race between the mining operation and timeout
        await Promise.race([
          invoke('cmd_start_miner_tauri', { port, address }),
          timeoutPromise
        ]);
        
        alert('Miner started successfully');
      } catch (err) {
        console.error('Failed to start miner:', err);
        if (err instanceof Error) {
          error = err.message || 'Failed to start miner';
        } else {
          error = 'Failed to start miner';
        }
      } finally {
        isLoading = false;
      }
    }
  </script>
  
  <div class="miner-card">
    <h2>Blockchain Miner</h2>
    
    {#if error}
      <div class="error-message">{error}</div>
    {/if}
    
    <div class="input-group">
      <label for="port">Port:</label>
      <input 
        id="port" 
        bind:value={port} 
        type="text" 
        placeholder="Enter port (e.g. 7777)"
        disabled={isLoading} 
      />
    </div>
  
    <div class="input-group">
      <label for="address">Address:</label>
      <input 
        id="address" 
        bind:value={address} 
        type="text" 
        placeholder="Enter your wallet address"
        disabled={isLoading} 
      />
    </div>
  
    <button 
      on:click={startMiner} 
      disabled={isLoading || !port || !address}
    >
      {#if isLoading}
        <span class="loader"></span>
      {:else}
        <span class="button-text">Start Mining</span>
        <span class="button-icon">⛏️</span>
      {/if}
    </button>
  </div>

  <style>
    .miner-card {
      background: linear-gradient(145deg, #1a1a1a, #2a2a2a);
      border-radius: 15px;
      padding: 2rem;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
      max-width: 500px;
      margin: 1rem auto;
      border: 1px solid rgba(255, 255, 255, 0.1);
    }
  
    h2 {
      color: #00ff88;
      margin-bottom: 1.5rem;
      font-size: 1.5rem;
      text-align: center;
      text-transform: uppercase;
      letter-spacing: 2px;
    }
  
    .input-group {
      margin-bottom: 1.5rem;
    }
  
    label {
      display: block;
      margin-bottom: 0.5rem;
      color: #00ff88;
      font-size: 0.9rem;
      letter-spacing: 1px;
    }
  
    input {
      width: 100%;
      padding: 0.8rem;
      background: rgba(255, 255, 255, 0.05);
      border: 1px solid rgba(0, 255, 136, 0.3);
      border-radius: 8px;
      color: white;
      font-size: 1rem;
      transition: all 0.3s ease;
    }
  
    input:focus {
      outline: none;
      border-color: #00ff88;
      box-shadow: 0 0 10px rgba(0, 255, 136, 0.2);
    }
  
    input::placeholder {
      color: rgba(255, 255, 255, 0.3);
    }
  
    button {
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
  
    button:hover {
      transform: translateY(-2px);
      box-shadow: 0 5px 15px rgba(0, 255, 136, 0.4);
    }
  
    button:active {
      transform: translateY(0);
    }
  
    .button-icon {
      font-size: 1.2rem;
    }
  
    @media (max-width: 600px) {
      .miner-card {
        margin: 1rem;
        padding: 1.5rem;
      }
    }

    .error-message {
    background: rgba(255, 0, 0, 0.1);
    border: 1px solid rgba(255, 0, 0, 0.3);
    color: #ff4444;
    padding: 0.8rem;
    border-radius: 8px;
    margin-bottom: 1rem;
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

  button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
  }
  </style>