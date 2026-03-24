export interface ApiConfig {
  base_url: string;
  api_key: string;
}

export interface WalletAddress {
  address: string;
  label: string | null;
  created_at: string;
}

export interface BlockchainInfo {
  height: number;
  best_block_hash: string;
  difficulty: number;
  total_blocks: number;
}

export interface BlockSummary {
  hash: string;
  height: number;
  time: number;
  tx_count: number;
}

export interface SendTransactionRequest {
  from_address: string;
  to_address: string;
  amount_satoshis: number;
}

export interface SendTransactionResponse {
  txid: string;
}

export interface CreateWalletResponse {
  address: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}
