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
  <h3>Send Transaction</h3>
  <div class="transaction-form">
    <div class="form-group">
      <label for="from">From Address:</label>
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

    <div class="form-group">
      <label for="to">To Address:</label>
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
      <label>
        <input 
          type="checkbox" 
          bind:checked={mineNow} 
          disabled={sending}
        />
        Mine Immediately
      </label>
    </div>

    <button 
      class="send-btn"
      on:click={sendTransaction} 
      disabled={sending}
    >
      {sending ? 'Sending...' : 'Send Transaction'}
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
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .transaction-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin: 1rem 0;
  }

  h3 {
    color: #2c3e50;
    margin: 0 0 1rem 0;
    font-size: 1.3rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    color: #4a5568;
    font-size: 0.9rem;
  }

  .address-select, .amount-input {
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
    background-color: #fff;
    transition: border-color 0.2s;
  }

  .address-select:focus, .amount-input:focus {
    outline: none;
    border-color: #e74c3c;
  }

  .checkbox {
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
  }

  .send-btn {
    background-color: #e74c3c;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
    margin-top: 1rem;
  }

  .send-btn:hover:not([disabled]) {
    background-color: #c0392b;
  }

  .send-btn[disabled] {
    background-color: #95a5a6;
    cursor: not-allowed;
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
    .transaction-form {
      gap: 1.5rem;
    }
    
    .send-btn {
      width: 100%;
    }
  }
</style>