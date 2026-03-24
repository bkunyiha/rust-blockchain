import React from "react";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import {
  Wallet,
  PlusCircle,
  List,
  Info,
  DollarSign,
  Send,
  History,
  Settings,
  Sun,
  Moon,
  Wifi,
  WifiOff,
  Copy,
  Check,
} from "lucide-react";
import { useWalletStore } from "../store/walletStore";
import { useConnectionStatus } from "../hooks/useCommands";
import { truncateAddress, cn } from "../lib/utils";
import ToastContainer from "./ToastContainer";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useToastStore } from "../store/toastStore";

interface NavItem {
  path: string;
  label: string;
  icon: React.ReactNode;
  requiresWallet: boolean;
}

const navItems: NavItem[] = [
  { path: "/wallet/create", label: "Create Wallet", icon: <PlusCircle size={18} />, requiresWallet: false },
  { path: "/wallet/list", label: "My Wallets", icon: <List size={18} />, requiresWallet: false },
  { path: "/wallet/info", label: "Wallet Info", icon: <Info size={18} />, requiresWallet: true },
  { path: "/balance", label: "Get Balance", icon: <DollarSign size={18} />, requiresWallet: true },
  { path: "/send", label: "Send", icon: <Send size={18} />, requiresWallet: true },
  { path: "/history", label: "History", icon: <History size={18} />, requiresWallet: true },
  { path: "/settings", label: "Settings", icon: <Settings size={18} />, requiresWallet: false },
];

export default function AppLayout() {
  const location = useLocation();
  const navigate = useNavigate();
  const { activeWallet, theme, toggleTheme, status } = useWalletStore();
  const { data: isConnected } = useConnectionStatus();
  const addToast = useToastStore((s) => s.addToast);
  const [copied, setCopied] = React.useState(false);

  const handleCopyAddress = async () => {
    if (activeWallet) {
      try {
        await writeText(activeWallet.address);
        setCopied(true);
        addToast("success", "Address copied to clipboard");
        setTimeout(() => setCopied(false), 2000);
      } catch {
        addToast("error", "Failed to copy address");
      }
    }
  };

  return (
    <div className={cn("flex h-screen", theme === "light" ? "bg-white text-gray-900" : "bg-gray-900 text-gray-100")}>
      {/* Sidebar */}
      <aside className={cn(
        "w-64 flex flex-col border-r",
        theme === "light" ? "bg-gray-50 border-gray-200" : "bg-gray-800 border-gray-700"
      )}>
        {/* App Title + Connection */}
        <div className={cn("px-4 py-3 border-b flex items-center justify-between",
          theme === "light" ? "border-gray-200" : "border-gray-700"
        )}>
          <div className="flex items-center gap-2">
            <Wallet size={20} className="text-bitcoin-orange" />
            <span className="font-semibold text-sm">Bitcoin Wallet</span>
          </div>
          <div className="flex items-center gap-1" title={isConnected ? "Connected" : "Disconnected"}>
            {isConnected ? (
              <Wifi size={14} className="text-green-400" />
            ) : (
              <WifiOff size={14} className="text-red-400" />
            )}
            <span className={cn("w-2 h-2 rounded-full", isConnected ? "bg-green-400" : "bg-red-400")} />
          </div>
        </div>

        {/* Active Wallet Card */}
        {activeWallet && (
          <div className={cn(
            "mx-3 mt-3 p-3 rounded-lg border",
            theme === "light" ? "bg-white border-blue-200" : "bg-gray-700 border-blue-500/30"
          )}>
            <div className="text-xs text-gray-400 mb-1">Active Wallet</div>
            <div className="font-bold text-sm truncate">
              {activeWallet.label || "Unnamed Wallet"}
            </div>
            <div className="flex items-center gap-1 mt-1">
              <code className="text-xs font-mono text-gray-400 truncate flex-1">
                {truncateAddress(activeWallet.address)}
              </code>
              <button
                onClick={handleCopyAddress}
                className="p-1 hover:text-bitcoin-orange transition-colors"
                title="Copy address"
              >
                {copied ? <Check size={12} className="text-green-400" /> : <Copy size={12} />}
              </button>
            </div>
          </div>
        )}

        {/* Navigation */}
        <nav className="flex-1 mt-3 px-2 space-y-0.5 overflow-y-auto">
          {navItems.map((item) => {
            const isActive = location.pathname === item.path;
            const isDisabled = item.requiresWallet && !activeWallet;

            return (
              <button
                key={item.path}
                onClick={() => !isDisabled && navigate(item.path)}
                disabled={isDisabled}
                className={cn(
                  "w-full flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors",
                  isActive && (theme === "light"
                    ? "bg-blue-50 text-blue-700 font-medium"
                    : "bg-gray-700 text-bitcoin-orange font-medium"),
                  !isActive && !isDisabled && (theme === "light"
                    ? "hover:bg-gray-100 text-gray-700"
                    : "hover:bg-gray-700/50 text-gray-300"),
                  isDisabled && "opacity-40 cursor-not-allowed"
                )}
              >
                {item.icon}
                {item.label}
              </button>
            );
          })}
        </nav>

        {/* Theme Toggle */}
        <div className={cn("px-4 py-3 border-t", theme === "light" ? "border-gray-200" : "border-gray-700")}>
          <button
            onClick={toggleTheme}
            className={cn(
              "flex items-center gap-2 text-sm w-full px-3 py-2 rounded-md transition-colors",
              theme === "light" ? "hover:bg-gray-100 text-gray-600" : "hover:bg-gray-700 text-gray-400"
            )}
          >
            {theme === "dark" ? <Sun size={16} /> : <Moon size={16} />}
            {theme === "dark" ? "Light Mode" : "Dark Mode"}
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        <div className="flex-1 overflow-y-auto p-6">
          {!activeWallet && location.pathname !== "/wallet/create" && location.pathname !== "/wallet/list" && location.pathname !== "/settings" ? (
            <div className="flex flex-col items-center justify-center h-full text-center">
              <Wallet size={48} className="text-gray-500 mb-4" />
              <h2 className="text-xl font-semibold mb-2">No Wallet Selected</h2>
              <p className="text-gray-400 mb-4">Select or create a wallet to get started</p>
              <div className="flex gap-3">
                <button
                  onClick={() => navigate("/wallet/create")}
                  className="px-4 py-2 bg-bitcoin-orange text-white rounded-lg hover:bg-bitcoin-orange-dark transition-colors"
                >
                  Create Wallet
                </button>
                <button
                  onClick={() => navigate("/wallet/list")}
                  className={cn(
                    "px-4 py-2 rounded-lg border transition-colors",
                    theme === "light" ? "border-gray-300 hover:bg-gray-50" : "border-gray-600 hover:bg-gray-800"
                  )}
                >
                  My Wallets
                </button>
              </div>
            </div>
          ) : (
            <Outlet />
          )}
        </div>

        {/* Status Bar */}
        {status && (
          <div className={cn(
            "px-4 py-1.5 text-xs border-t",
            theme === "light" ? "bg-gray-50 border-gray-200 text-gray-500" : "bg-gray-800 border-gray-700 text-gray-400"
          )}>
            {status}
          </div>
        )}
      </main>

      {/* Toast Container */}
      <ToastContainer />
    </div>
  );
}
