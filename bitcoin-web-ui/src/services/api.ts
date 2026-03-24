import axios, { AxiosInstance } from 'axios';
import type {
  ApiResponse,
  BlockchainInfo,
  BlockSummary,
  CreateWalletRequest,
  CreateWalletResponse,
  SendTransactionRequest,
  SendTransactionResponse,
  JsonValue,
  MiningRequest,
  PaginatedResponse,
} from '../types/api';

export class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string, apiKey?: string) {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
        ...(apiKey && { 'X-API-Key': apiKey }),
      },
    });
  }

  updateApiKey(apiKey: string) {
    this.client.defaults.headers['X-API-Key'] = apiKey;
  }

  removeApiKey() {
    delete this.client.defaults.headers['X-API-Key'];
  }

  // Blockchain endpoints
  async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
    const response = await this.client.get('/api/admin/blockchain');
    return response.data;
  }

  async getLatestBlocks(): Promise<ApiResponse<BlockSummary[]>> {
    const response = await this.client.get('/api/admin/blockchain/blocks/latest');
    return response.data;
  }

  async getAllBlocks(page?: number, limit?: number): Promise<ApiResponse<PaginatedResponse<BlockSummary>>> {
    const params = new URLSearchParams();
    if (page !== undefined) params.append('page', page.toString());
    if (limit !== undefined) params.append('limit', limit.toString());
    const queryString = params.toString();
    const url = `/api/admin/blockchain/blocks${queryString ? `?${queryString}` : ''}`;
    const response = await this.client.get(url);
    return response.data;
  }

  async getBlockByHash(hash: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/blockchain/blocks/${hash}`);
    return response.data;
  }

  // Wallet endpoints
  async createWallet(req: CreateWalletRequest): Promise<ApiResponse<CreateWalletResponse>> {
    const response = await this.client.post('/api/admin/wallet', req);
    return response.data;
  }

  async getAddresses(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/wallet/addresses');
    return response.data;
  }

  async getWalletInfo(address: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/wallet/${address}`);
    return response.data;
  }

  async getBalance(address: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/wallet/${address}/balance`);
    return response.data;
  }

  async sendTransaction(req: SendTransactionRequest): Promise<ApiResponse<SendTransactionResponse>> {
    const response = await this.client.post('/api/admin/transactions', req);
    return response.data;
  }

  async getAddressTransactions(address: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/transactions/address/${address}`);
    return response.data;
  }

  // Transaction endpoints
  async getMempool(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/transactions/mempool');
    return response.data;
  }

  async getMempoolTransaction(txid: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/transactions/mempool/${txid}`);
    return response.data;
  }

  async getAllTransactions(page: number = 1, limit: number = 100): Promise<ApiResponse<PaginatedResponse<JsonValue>>> {
    const response = await this.client.get('/api/admin/transactions', {
      params: { page, limit },
    });
    return response.data;
  }

  // Mining endpoints
  async getMiningInfo(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/mining/info');
    return response.data;
  }

  async generateToAddress(req: MiningRequest): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.post('/api/admin/mining/generatetoaddress', req);
    return response.data;
  }

  // Health endpoints
  async getHealth(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/health');
    return response.data;
  }

  async getLiveness(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/health/live');
    return response.data;
  }

  async getReadiness(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/health/ready');
    return response.data;
  }
}

// Singleton instance
let apiClient: ApiClient | null = null;

export function getApiClient(): ApiClient {
  if (!apiClient) {
    const baseURL = localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080';
    const apiKey = localStorage.getItem('api_key') || undefined;
    apiClient = new ApiClient(baseURL, apiKey);
  }
  return apiClient;
}

export function updateApiClient(baseURL: string, apiKey?: string) {
  apiClient = new ApiClient(baseURL, apiKey);
  if (apiKey) {
    localStorage.setItem('api_key', apiKey);
  } else {
    localStorage.removeItem('api_key');
  }
  localStorage.setItem('api_base_url', baseURL);
}

