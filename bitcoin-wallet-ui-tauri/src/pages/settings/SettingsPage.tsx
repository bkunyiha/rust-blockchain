import React, { useState, useEffect } from "react";
import { Loader, AlertCircle, Check } from "lucide-react";
import { useSettings, useSaveSettings, commands } from "../../hooks/useCommands";
import { useWalletStore } from "../../store/walletStore";
import { useToastStore } from "../../store/toastStore";
import { cn } from "../../lib/utils";

export default function SettingsPage() {
  const { data: settings, isLoading, error } = useSettings();
  const { mutate: saveSettings, isPending } = useSaveSettings();
  const theme = useWalletStore((s) => s.theme);
  const addToast = useToastStore((s) => s.addToast);

  const [baseUrl, setBaseUrl] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [connectionStatus, setConnectionStatus] = useState<boolean | null>(null);
  const [isTesting, setIsTesting] = useState(false);
  const [testError, setTestError] = useState<string | null>(null);

  useEffect(() => {
    if (settings) {
      setBaseUrl(settings.base_url || "");
      setApiKey(settings.api_key || "");
    }
  }, [settings]);

  const handleTestConnection = async () => {
    setIsTesting(true);
    setTestError(null);
    try {
      const connected = await commands.checkConnection();
      setConnectionStatus(connected);
      if (connected) {
        addToast("success", "Connection successful");
      } else {
        setTestError("Connection failed");
        addToast("error", "Connection failed");
      }
    } catch (err: any) {
      setTestError(err.message || "Connection test failed");
      addToast("error", err.message || "Connection test failed");
    } finally {
      setIsTesting(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!baseUrl.trim()) {
      addToast("error", "Base URL is required");
      return;
    }

    saveSettings(
      { baseUrl, apiKey },
      {
        onSuccess: () => {
          addToast("success", "Settings saved successfully");
        },
        onError: (err: any) => {
          addToast("error", err.message || "Failed to save settings");
        },
      }
    );
  };

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center h-full">
        <Loader size={32} className="animate-spin text-bitcoin-orange mb-4" />
        <p className={cn("text-sm", theme === "light" ? "text-gray-600" : "text-gray-400")}>
          Loading settings...
        </p>
      </div>
    );
  }

  if (error && !settings) {
    return (
      <div className="max-w-2xl mx-auto">
        <div className={cn(
          "flex items-start gap-3 p-4 rounded-lg border",
          theme === "light" ? "bg-red-50 border-red-200" : "bg-red-900/20 border-red-800/50"
        )}>
          <AlertCircle size={20} className={cn(
            "flex-shrink-0 mt-0.5",
            theme === "light" ? "text-red-600" : "text-red-400"
          )} />
          <div>
            <h3 className={cn("font-semibold", theme === "light" ? "text-red-800" : "text-red-200")}>
              Error Loading Settings
            </h3>
            <p className={cn("text-sm mt-1", theme === "light" ? "text-red-700" : "text-red-300")}>
              {error instanceof Error ? error.message : "An unexpected error occurred"}
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto">
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">Settings</h1>
        <p className={cn("text-sm", theme === "light" ? "text-gray-600" : "text-gray-400")}>
          Configure wallet connection and API settings
        </p>
      </div>

      <form onSubmit={handleSubmit} className={cn(
        "rounded-lg border p-6 mb-6",
        theme === "light" ? "bg-white border-gray-200" : "bg-gray-800 border-gray-700"
      )}>
        {/* Connection Status */}
        <div className="mb-6">
          <h2 className="text-lg font-semibold mb-3">Connection Status</h2>
          <div className="flex items-center gap-3">
            <div className={cn(
              "w-3 h-3 rounded-full",
              connectionStatus === true
                ? "bg-green-400"
                : connectionStatus === false
                  ? "bg-red-400"
                  : "bg-gray-400"
            )} />
            <span className={cn(
              "text-sm",
              connectionStatus === true
                ? "text-green-400"
                : connectionStatus === false
                  ? "text-red-400"
                  : "text-gray-400"
            )}>
              {connectionStatus === true
                ? "Connected"
                : connectionStatus === false
                  ? "Disconnected"
                  : "Not tested"}
            </span>
            <button
              type="button"
              onClick={handleTestConnection}
              disabled={isTesting}
              className="ml-auto px-4 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 flex items-center gap-2"
            >
              {isTesting && <Loader size={14} className="animate-spin" />}
              Test Connection
            </button>
          </div>
          {testError && (
            <p className={cn("text-sm mt-2", theme === "light" ? "text-red-600" : "text-red-400")}>
              {testError}
            </p>
          )}
        </div>

        <hr className={cn("my-6", theme === "light" ? "border-gray-200" : "border-gray-700")} />

        {/* Base URL */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">API Base URL</label>
          <input
            type="url"
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder="https://api.example.com"
            disabled={isPending}
            className={cn(
              "w-full px-3 py-2 rounded-lg border transition-colors disabled:opacity-50",
              theme === "light"
                ? "bg-white border-gray-300 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                : "bg-gray-700 border-gray-600 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            )}
          />
          <p className={cn("text-xs mt-1", theme === "light" ? "text-gray-500" : "text-gray-400")}>
            The URL for your Bitcoin backend API
          </p>
        </div>

        {/* API Key */}
        <div className="mb-6">
          <label className="block text-sm font-medium mb-2">API Key</label>
          <input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder="Your API key"
            disabled={isPending}
            className={cn(
              "w-full px-3 py-2 rounded-lg border transition-colors disabled:opacity-50",
              theme === "light"
                ? "bg-white border-gray-300 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                : "bg-gray-700 border-gray-600 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
            )}
          />
          <p className={cn("text-xs mt-1", theme === "light" ? "text-gray-500" : "text-gray-400")}>
            API authentication key (if required)
          </p>
        </div>

        <button
          type="submit"
          disabled={isPending}
          className="w-full bg-bitcoin-orange text-white py-2 rounded-lg font-medium hover:bg-bitcoin-orange-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        >
          {isPending && <Loader size={16} className="animate-spin" />}
          {isPending ? "Saving..." : "Save Settings"}
        </button>
      </form>

      {/* Additional Info */}
      <div className={cn(
        "rounded-lg border p-4",
        theme === "light" ? "bg-blue-50 border-blue-200" : "bg-blue-900/20 border-blue-800/50"
      )}>
        <h3 className={cn(
          "text-sm font-semibold mb-2",
          theme === "light" ? "text-blue-800" : "text-blue-200"
        )}>
          Tips
        </h3>
        <ul className={cn(
          "text-xs space-y-1",
          theme === "light" ? "text-blue-700" : "text-blue-300"
        )}>
          <li>Keep your API key secure and never share it</li>
          <li>Use HTTPS URLs for secure communication</li>
          <li>Test your connection after updating settings</li>
        </ul>
      </div>
    </div>
  );
}
