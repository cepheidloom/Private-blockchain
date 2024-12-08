<!-- +page.svelte -->
<script lang="ts">
  import AddressList from '../components/AddressList.svelte';
  import WalletCreator from '../components/WalletCreator.svelte';
  import BalanceChecker from '../components/BalanceChecker.svelte';
  import BlockchainCreator from '../components/BlockchainCreator.svelte';
  import TransactionSender from '../components/TransactionSender.svelte';
  import UTXOInfo from '../components/UTXOInfo.svelte';
  import BlockchainViewer from '../components/BlockchainViewer.svelte';
  import MineButton from '../components/MineButton.svelte';

  
  let addresses: string[] = [];
</script>

<main class="container">
  <h1>Wallet Management</h1>
  
  <div class="components">
    <AddressList bind:addresses />
    <WalletCreator />
    <BalanceChecker {addresses} />
    <BlockchainCreator {addresses} />
    <TransactionSender {addresses} />
    <UTXOInfo />
    <BlockchainViewer />
    <MineButton />
  </div>
</main>

<style>
  .components {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
</style>