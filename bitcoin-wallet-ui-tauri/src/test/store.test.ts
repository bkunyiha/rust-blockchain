import { describe, it, expect, beforeEach } from "vitest";
import { useWalletStore } from "../store/walletStore";
import { useToastStore } from "../store/toastStore";

describe("walletStore", () => {
  beforeEach(() => {
    useWalletStore.setState({
      activeWallet: null,
      savedWallets: [],
      theme: "dark",
      status: "",
    });
  });

  it("sets active wallet", () => {
    const wallet = { address: "test_addr", label: "Test", created_at: "2025-01-01" };
    useWalletStore.getState().setActiveWallet(wallet);
    expect(useWalletStore.getState().activeWallet).toEqual(wallet);
  });

  it("sets saved wallets", () => {
    const wallets = [
      { address: "addr1", label: "W1", created_at: "2025-01-01" },
      { address: "addr2", label: "W2", created_at: "2025-01-02" },
    ];
    useWalletStore.getState().setSavedWallets(wallets);
    expect(useWalletStore.getState().savedWallets).toHaveLength(2);
  });

  it("sets status message", () => {
    useWalletStore.getState().setStatus("Connected");
    expect(useWalletStore.getState().status).toBe("Connected");
  });

  it("toggles theme", () => {
    expect(useWalletStore.getState().theme).toBe("dark");
    useWalletStore.getState().toggleTheme();
    expect(useWalletStore.getState().theme).toBe("light");
    useWalletStore.getState().toggleTheme();
    expect(useWalletStore.getState().theme).toBe("dark");
  });
});

describe("toastStore", () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it("adds a toast", () => {
    useToastStore.getState().addToast("success", "Test message");
    expect(useToastStore.getState().toasts).toHaveLength(1);
    expect(useToastStore.getState().toasts[0].type).toBe("success");
    expect(useToastStore.getState().toasts[0].message).toBe("Test message");
  });

  it("removes a toast", () => {
    useToastStore.getState().addToast("info", "Toast to remove");
    const id = useToastStore.getState().toasts[0].id;
    useToastStore.getState().removeToast(id);
    expect(useToastStore.getState().toasts).toHaveLength(0);
  });
});
