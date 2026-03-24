import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { getApiClient } from '../services/api';
import type {
  CreateWalletRequest,
  SendTransactionRequest,
  MiningRequest,
} from '../types/api';
import toast from 'react-hot-toast';

// Blockchain queries
export function useBlockchainInfo(refetchInterval?: number) {
  return useQuery({
    queryKey: ['blockchain', 'info'],
    queryFn: () => getApiClient().getBlockchainInfo(),
    refetchInterval,
    retry: 1,
  });
}

export function useLatestBlocks() {
  return useQuery({
    queryKey: ['blockchain', 'latest-blocks'],
    queryFn: () => getApiClient().getLatestBlocks(),
    retry: 1,
  });
}

export function useAllBlocks(page?: number, limit?: number) {
  return useQuery({
    queryKey: ['blockchain', 'all-blocks', page, limit],
    queryFn: () => getApiClient().getAllBlocks(page, limit),
    enabled: false,
    retry: 1,
  });
}

export function useBlockByHash(hash: string) {
  return useQuery({
    queryKey: ['blockchain', 'block', hash],
    queryFn: () => getApiClient().getBlockByHash(hash),
    enabled: !!hash,
    retry: 1,
  });
}

// Wallet queries
export function useCreateWallet() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: CreateWalletRequest) => getApiClient().createWallet(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['wallet', 'addresses'] });
      toast.success('Wallet created successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.error || 'Failed to create wallet');
    },
  });
}

export function useAddresses() {
  return useQuery({
    queryKey: ['wallet', 'addresses'],
    queryFn: () => getApiClient().getAddresses(),
    retry: 1,
  });
}

export function useWalletInfo(address: string) {
  return useQuery({
    queryKey: ['wallet', 'info', address],
    queryFn: () => getApiClient().getWalletInfo(address),
    enabled: !!address,
    retry: 1,
  });
}

export function useBalance(address: string) {
  return useQuery({
    queryKey: ['wallet', 'balance', address],
    queryFn: () => getApiClient().getBalance(address),
    enabled: !!address,
    retry: 1,
  });
}

export function useSendTransaction() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: SendTransactionRequest) => getApiClient().sendTransaction(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['transactions'] });
      queryClient.invalidateQueries({ queryKey: ['wallet'] });
      toast.success('Transaction sent successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.error || 'Failed to send transaction');
    },
  });
}

export function useAddressTransactions(address: string) {
  return useQuery({
    queryKey: ['transactions', 'address', address],
    queryFn: () => getApiClient().getAddressTransactions(address),
    enabled: !!address,
    retry: 1,
  });
}

// Transaction queries
export function useMempool() {
  return useQuery({
    queryKey: ['transactions', 'mempool'],
    queryFn: () => getApiClient().getMempool(),
    retry: 1,
  });
}

export function useMempoolTransaction(txid: string) {
  return useQuery({
    queryKey: ['transactions', 'mempool', txid],
    queryFn: () => getApiClient().getMempoolTransaction(txid),
    enabled: !!txid,
    retry: 1,
  });
}

export function useAllTransactions(page: number = 1, limit: number = 100) {
  return useQuery({
    queryKey: ['transactions', 'all', page, limit],
    queryFn: () => getApiClient().getAllTransactions(page, limit),
    enabled: false,
    retry: 1,
  });
}

// Mining queries
export function useMiningInfo() {
  return useQuery({
    queryKey: ['mining', 'info'],
    queryFn: () => getApiClient().getMiningInfo(),
    retry: 1,
  });
}

export function useGenerateBlocks() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: MiningRequest) => getApiClient().generateToAddress(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['blockchain'] });
      queryClient.invalidateQueries({ queryKey: ['mining'] });
      toast.success('Blocks generated successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.error || 'Failed to generate blocks');
    },
  });
}

// Health queries
export function useHealth() {
  return useQuery({
    queryKey: ['health'],
    queryFn: () => getApiClient().getHealth(),
    retry: 1,
  });
}

export function useLiveness() {
  return useQuery({
    queryKey: ['health', 'liveness'],
    queryFn: () => getApiClient().getLiveness(),
    retry: 1,
  });
}

export function useReadiness() {
  return useQuery({
    queryKey: ['health', 'readiness'],
    queryFn: () => getApiClient().getReadiness(),
    retry: 1,
  });
}

