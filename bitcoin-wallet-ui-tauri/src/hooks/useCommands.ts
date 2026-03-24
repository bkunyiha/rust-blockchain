import { invoke } from "@tauri-apps/api/core";
import { WalletAddress, Settings, SendTxResult } from "../types";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

// ---- Raw invoke wrappers ----
export const commands = {
  createWallet: (label?: string) =>
    invoke<WalletAddress>("create_wallet", { label: label || null }),
  getSavedWallets: () =>
    invoke<WalletAddress[]>("get_saved_wallets"),
  setActiveWallet: (address: string) =>
    invoke<void>("set_active_wallet", { address }),
  deleteWalletAddress: (address: string) =>
    invoke<void>("delete_wallet_address", { address }),
  updateWalletLabel: (address: string, label: string) =>
    invoke<void>("update_wallet_label", { address, label }),
  getWalletInfo: (address: string) =>
    invoke<any>("get_wallet_info", { address }),
  getBalance: (address: string) =>
    invoke<any>("get_balance", { address }),
  sendTransaction: (from: string, to: string, amount: number) =>
    invoke<SendTxResult>("send_transaction", { from, to, amount }),
  getTxHistory: (address: string) =>
    invoke<any>("get_tx_history", { address }),
  getSettings: () =>
    invoke<Settings>("get_settings"),
  saveSettings: (baseUrl: string, apiKey: string) =>
    invoke<void>("save_settings", { baseUrl, apiKey }),
  checkConnection: () =>
    invoke<boolean>("check_connection"),
  healthCheck: () =>
    invoke<boolean>("health_check"),
};

// ---- React Query hooks ----
export function useSavedWallets() {
  return useQuery({
    queryKey: ["savedWallets"],
    queryFn: commands.getSavedWallets,
  });
}

export function useWalletInfo(address: string | undefined) {
  return useQuery({
    queryKey: ["walletInfo", address],
    queryFn: () => commands.getWalletInfo(address!),
    enabled: !!address,
  });
}

export function useBalance(address: string | undefined) {
  return useQuery({
    queryKey: ["balance", address],
    queryFn: () => commands.getBalance(address!),
    enabled: !!address,
  });
}

export function useTxHistory(address: string | undefined) {
  return useQuery({
    queryKey: ["txHistory", address],
    queryFn: () => commands.getTxHistory(address!),
    enabled: !!address,
  });
}

export function useSettings() {
  return useQuery({
    queryKey: ["settings"],
    queryFn: commands.getSettings,
  });
}

export function useConnectionStatus() {
  return useQuery({
    queryKey: ["connectionStatus"],
    queryFn: commands.checkConnection,
    refetchInterval: 30000, // Poll every 30 seconds
  });
}

export function useCreateWallet() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (label?: string) => commands.createWallet(label),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["savedWallets"] });
    },
  });
}

export function useDeleteWallet() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (address: string) => commands.deleteWalletAddress(address),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["savedWallets"] });
    },
  });
}

export function useSendTransaction() {
  return useMutation({
    mutationFn: ({ from, to, amount }: { from: string; to: string; amount: number }) =>
      commands.sendTransaction(from, to, amount),
  });
}

export function useSaveSettings() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ baseUrl, apiKey }: { baseUrl: string; apiKey: string }) =>
      commands.saveSettings(baseUrl, apiKey),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["settings"] });
      queryClient.invalidateQueries({ queryKey: ["connectionStatus"] });
    },
  });
}
