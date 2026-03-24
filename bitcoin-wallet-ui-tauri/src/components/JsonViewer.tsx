import React, { useState } from "react";
import { Copy, Check, ChevronDown, ChevronRight } from "lucide-react";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useToastStore } from "../store/toastStore";
import { useWalletStore } from "../store/walletStore";
import { cn } from "../lib/utils";

interface JsonViewerProps {
  data: any;
  title?: string;
}

export default function JsonViewer({ data, title }: JsonViewerProps) {
  const [collapsed, setCollapsed] = useState(false);
  const [copied, setCopied] = useState(false);
  const addToast = useToastStore((s) => s.addToast);
  const theme = useWalletStore((s) => s.theme);

  const jsonString = JSON.stringify(data, null, 2);

  const handleCopy = async () => {
    try {
      await writeText(jsonString);
      setCopied(true);
      addToast("success", "JSON copied to clipboard");
      setTimeout(() => setCopied(false), 2000);
    } catch {
      addToast("error", "Failed to copy");
    }
  };

  return (
    <div className={cn("rounded-lg border", theme === "light" ? "border-gray-200 bg-gray-50" : "border-gray-700 bg-gray-800")}>
      <div className={cn("flex items-center justify-between px-4 py-2 border-b",
        theme === "light" ? "border-gray-200" : "border-gray-700"
      )}>
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="flex items-center gap-2 text-sm font-medium"
        >
          {collapsed ? <ChevronRight size={14} /> : <ChevronDown size={14} />}
          {title || "Response Data"}
        </button>
        <button onClick={handleCopy} className="p-1 hover:text-bitcoin-orange transition-colors" title="Copy JSON">
          {copied ? <Check size={14} className="text-green-400" /> : <Copy size={14} />}
        </button>
      </div>
      {!collapsed && (
        <pre className={cn("p-4 text-xs font-mono overflow-auto max-h-96",
          theme === "light" ? "text-gray-800" : "text-gray-300"
        )}>
          {jsonString}
        </pre>
      )}
    </div>
  );
}
