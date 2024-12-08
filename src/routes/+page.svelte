<!-- +page.svelte -->
<script lang="ts">
  import { fade } from 'svelte/transition';
  import AddressList from '../components/AddressList.svelte';
  import WalletCreator from '../components/WalletCreator.svelte';
  import BalanceChecker from '../components/BalanceChecker.svelte';
  import BlockchainCreator from '../components/BlockchainCreator.svelte';
  import TransactionSender from '../components/TransactionSender.svelte';
  import UTXOInfo from '../components/UTXOInfo.svelte';
  import BlockchainViewer from '../components/BlockchainViewer.svelte';
  import MineButton from '../components/MineButton.svelte';
  
  let addresses: string[] = [];
  let activeTab = 'wallet';
</script>

<main class="container">
  <h1 class="title">Blockchain Management System</h1>
  
  <nav class="tabs">
    <button 
      class="tab-button {activeTab === 'wallet' ? 'active' : ''}" 
      on:click={() => activeTab = 'wallet'}
    >
      Wallet Operations
    </button>
    <button 
      class="tab-button {activeTab === 'blockchain' ? 'active' : ''}" 
      on:click={() => activeTab = 'blockchain'}
    >
      Blockchain Operations
    </button>
    <button 
      class="tab-button {activeTab === 'transaction' ? 'active' : ''}" 
      on:click={() => activeTab = 'transaction'}
    >
      Transaction Operations
    </button>
  </nav>

  <div class="content-container">
    {#if activeTab === 'wallet'}
      <section class="section wallet-section" transition:fade>
        <h2 class="section-title">Wallet Operations</h2>
        <div class="section-content">
          <AddressList bind:addresses />
          <WalletCreator />
          <BalanceChecker {addresses} />
        </div>
      </section>
    {/if}

    {#if activeTab === 'blockchain'}
      <section class="section blockchain-section" transition:fade>
        <h2 class="section-title">Blockchain Operations</h2>
        <div class="section-content">
          <BlockchainCreator {addresses} />
          <BlockchainViewer />
        </div>
      </section>
    {/if}

    {#if activeTab === 'transaction'}
      <section class="section transaction-section" transition:fade>
        <h2 class="section-title">Transaction Operations</h2>
        <div class="section-content">
          <TransactionSender {addresses} />
          <UTXOInfo />
          <MineButton />
        </div>
      </section>
    {/if}
  </div>
</main>

<style>
  .container {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
  }

  .title {
    color: #00ff88;
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 2rem;
    text-transform: uppercase;
    letter-spacing: 3px;
    text-shadow: 0 0 10px rgba(0, 255, 136, 0.5);
  }

  .tabs {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .tab-button {
    padding: 0.8rem 1.5rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(0, 255, 136, 0.3);
    border-radius: 8px;
    color: #00ff88;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .tab-button:hover {
    background: rgba(0, 255, 136, 0.1);
    transform: translateY(-2px);
  }

  .tab-button.active {
    background: rgba(0, 255, 136, 0.2);
    border-color: #00ff88;
    box-shadow: 0 0 15px rgba(0, 255, 136, 0.3);
  }

  .section {
    background: linear-gradient(145deg, #1a1a1a, #2a2a2a);
    border-radius: 20px;
    padding: 2rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .section-title {
    color: #ff61d8;
    font-size: 1.2rem;
    text-transform: uppercase;
    letter-spacing: 2px;
    margin-bottom: 1.5rem;
    text-align: center;
    text-shadow: 0 0 10px rgba(255, 97, 216, 0.5);
  }

  .section-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .wallet-section { border-top: 3px solid #00ff88; }
  .blockchain-section { border-top: 3px solid #ff61d8; }
  .transaction-section { border-top: 3px solid #61ffff; }

  @media (max-width: 600px) {
    .container {
      padding: 1rem;
    }
    
    .tabs {
      flex-direction: column;
    }
  }
</style>