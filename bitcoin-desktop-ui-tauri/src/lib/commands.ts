import { invoke } from '@tauri-apps/api/core';

export const commands = {
  // Blockchain
  getBlockchainInfo: () => invoke('get_blockchain_info'),
  getLatestBlocks: () => invoke('get_latest_blocks'),
  getAllBlocks: () => invoke('get_all_blocks'),
  getBlockByHash: (hash: string) => invoke('get_block_by_hash', { hash }),

  // Wallet
  createWallet: (label?: string) => invoke('create_wallet', { label }),
  getWalletInfo: (address: string) =>
    invoke('get_wallet_info', { address }),
  getBalance: (address: string) => invoke('get_balance', { address }),
  sendTransaction: (
    fromAddress: string,
    toAddress: string,
    amount: number
  ) =>
    invoke('send_transaction', {
      fromAddress,
      toAddress,
      amount,
    }),
  getTxHistory: (address: string) =>
    invoke('get_tx_history', { address }),
  getAllAddresses: () => invoke('get_all_addresses'),

  // Transactions
  getMempool: () => invoke('get_mempool'),
  getMempoolTransaction: (txid: string) =>
    invoke('get_mempool_transaction', { txid }),
  getAllTransactions: () => invoke('get_all_transactions'),
  getAddressTransactions: (address: string) =>
    invoke('get_address_transactions', { address }),

  // Mining
  getMiningInfo: () => invoke('get_mining_info'),
  generateBlocks: (address: string, nblocks: number, maxtries?: number) =>
    invoke('generate_blocks', {
      address,
      nblocks,
      maxtries,
    }),

  // Health
  healthCheck: () => invoke('health_check'),
  livenessCheck: () => invoke('liveness_check'),
  readinessCheck: () => invoke('readiness_check'),

  // Settings
  getConfig: () => invoke('get_config'),
  updateConfig: (baseUrl: string, apiKey: string) =>
    invoke('update_config', {
      baseUrl,
      apiKey,
    }),
  checkConnection: () => invoke('check_connection'),
};
