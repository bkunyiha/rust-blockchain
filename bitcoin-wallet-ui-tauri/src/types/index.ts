export interface WalletAddress {
  address: string;
  label: string | null;
  created_at: string;
}

export interface Settings {
  base_url: string;
  api_key: string;
}

export interface SendTxResult {
  txid: string;
}

export interface ConnectionStatus {
  connected: boolean;
  message: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
  timestamp: string;
}
