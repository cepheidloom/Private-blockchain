<!-- AddressList.svelte -->
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    
    export let addresses: string[] = [];
    let message: string = '';
  
    async function getAddresses() {
      try {
        addresses = await invoke("cmd_get_address_wrapper");
      } catch (error) {
        console.error("Error fetching addresses:", error);
      }
    }
  </script>
  
  <div class="address-container">
    <button class="btn" on:click={getAddresses}>Get Addresses</button>
    <h2>All Addresses</h2>
    <div class="address-list">
      <ul>
        {#each addresses as address}
          <li class="address-item">{address}</li>
        {/each}
      </ul>
    </div>
  </div>

  <style>
    .address-container {
      background: white;
      border-radius: 8px;
      padding: 1.5rem;
      box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }
  
    .address-list {
      max-height: 300px;
      overflow-y: auto;
      margin-top: 1rem;
      border: 1px solid #eee;
      border-radius: 4px;
    }
  
    ul {
      list-style: none;
      padding: 0;
      margin: 0;
    }
  
    .address-item {
      font-family: 'Courier New', monospace;
      padding: 0.75rem 1rem;
      border-bottom: 1px solid #eee;
      transition: background-color 0.2s;
    }
  
    .address-item:hover {
      background-color: #f8f9fa;
    }
  
    .address-item:last-child {
      border-bottom: none;
    }
  
    h2 {
      color: #2c3e50;
      margin: 1rem 0;
      font-size: 1.5rem;
    }
  
    .btn {
      background-color: #4a90e2;
      color: white;
      padding: 0.5rem 1rem;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      font-weight: 500;
      transition: background-color 0.2s;
    }
  
    .btn:hover {
      background-color: #357abd;
    }
  
    /* For webkit browsers */
    .address-list::-webkit-scrollbar {
      width: 8px;
    }
  
    .address-list::-webkit-scrollbar-track {
      background: #f1f1f1;
      border-radius: 4px;
    }
  
    .address-list::-webkit-scrollbar-thumb {
      background: #888;
      border-radius: 4px;
    }
  </style>