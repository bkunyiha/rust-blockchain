// API Types matching Rust API responses

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

export interface BlockchainInfo {
  height: number;
  difficulty: number;
  total_blocks: number;
  total_transactions: number;
  mempool_size: number;
  last_block_hash: string;
  last_block_timestamp: string;
}

export interface BlockSummary {
  hash: string;
  previous_hash: string;
  timestamp: string;
  height: number;
  nonce: number;
  difficulty: number;
  transaction_count: number;
  merkle_root: string;
  size_bytes: number;
}

export interface CreateWalletRequest {
  label?: string;
}

export interface CreateWalletResponse {
  address: string;
}

export interface SendTransactionRequest {
  from_address: string;
  to_address: string;
  amount: number;
}

export interface SendTransactionResponse {
  txid: string;
}

export interface BalanceResponse {
  address: string;
  confirmed: number;
  unconfirmed: number;
}

export interface MiningRequest {
  address: string;
  nblocks: number;
  maxtries?: number;
}

export type JsonValue = any;

export interface PaginatedResponse<T> {
  items: T[];
  page: number;
  limit: number;
  total: number;
  total_pages: number;
  has_next: boolean;
  has_prev: boolean;
}

