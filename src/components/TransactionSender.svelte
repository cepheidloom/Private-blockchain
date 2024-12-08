<!-- TransactionSender.svelte -->
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    
    export let addresses: string[] = [];
    let fromAddress: string = '';
    let toAddress: string = '';
    let amount: number = 0;
    let mineNow: boolean = false;
    let sending: boolean = false;
    let message: string = '';
  
    async function sendTransaction() {
      if (!fromAddress || !toAddress || amount <= 0) {
        message = "Please fill all fields and ensure amount is greater than 0";
        return;
      }
    
      if (fromAddress === toAddress) {
      message = "Cannot send to same address";
      return;
    }
    
    try {
      sending = true;
      message = "Sending transaction...";
      
      const result = await invoke("cmd_send_tauri", { 
        from: fromAddress,
        to: toAddress,
        amount,
        mineNow
      });
      
      console.log("Transaction result:", result); // Debug log
      
      message = `Transaction sent successfully${mineNow ? ' and mined' : ''}!`;
      amount = 0;
      
    } catch (error: any) { // Type as any to access error properties
      console.error("Full error details:", error); // Debug log
      
      // Use specific error message if available
      const errorMessage = error.message || error.toString();
      message = `Transaction failed: ${errorMessage}`;
      
    } finally {
      sending = false;
    }
  }
  </script>


<div class="transaction-container">
<h2>Transaction Portal <span class="subtitle">Digital Transfer</span></h2>
<div class="transaction-form">
  <div class="form-group">
    <label for="from">From Address:</label>
    <div class="select-wrapper">
      <select 
        id="from"
        class="address-select" 
        bind:value={fromAddress}
        disabled={sending}
      >
        <option value="">Select sender address</option>
        {#each addresses as address}
          <option value={address}>{address}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="form-group">
    <label for="to">To Address:</label>
    <div class="select-wrapper">
      <select 
        id="to"
        class="address-select" 
        bind:value={toAddress}
        disabled={sending}
      >
        <option value="">Select recipient address</option>
        {#each addresses as address}
          <option value={address}>{address}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="form-group">
    <label for="amount">Amount:</label>
    <input 
      id="amount"
      type="number" 
      bind:value={amount}
      min="1"
      disabled={sending}
      class="amount-input"
      placeholder="Enter amount"
    />
  </div>

  <div class="form-group checkbox">
    <label class="cyber-checkbox">
      <input 
        type="checkbox" 
        bind:checked={mineNow} 
        disabled={sending}
      />
      <span class="checkbox-text">Mine Immediately</span>
    </label>
  </div>

  <button 
    class="cyber-button"
    on:click={sendTransaction} 
    disabled={sending}
  >
    {#if sending}
      <span class="loader"></span>
    {:else}
      <span class="button-content">
        <span class="button-text">Send Transaction</span>
        <span class="button-icon">💸</span>
      </span>
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
.transaction-container {
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

.transaction-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

label {
  color: #00ff88;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.select-wrapper {
  position: relative;
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

.address-select, .amount-input {
  width: 100%;
  padding: 0.8rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(0, 255, 136, 0.3);
  border-radius: 8px;
  color: #00ff88;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.address-select:focus, .amount-input:focus {
  outline: none;
  border-color: #00ff88;
  box-shadow: 0 0 10px rgba(0, 255, 136, 0.2);
}

.address-select option {
  background: #1a1a1a;
  color: #00ff88;
}

.cyber-checkbox {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.checkbox-text {
  color: #ff61d8;
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
  .transaction-container {
    margin: 1rem;
    padding: 1.5rem;
  }
}
</style>