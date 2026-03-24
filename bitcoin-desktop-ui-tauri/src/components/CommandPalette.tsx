import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Search, ArrowRight } from 'lucide-react';

interface Command {
  id: string;
  label: string;
  path: string;
  section: string;
}

const commands: Command[] = [
  // Blockchain
  { id: 'blockchain-info', label: 'Blockchain Info', path: '/blockchain/info', section: 'Blockchain' },
  { id: 'latest-blocks', label: 'Latest Blocks', path: '/blockchain/latest', section: 'Blockchain' },
  { id: 'all-blocks', label: 'All Blocks', path: '/blockchain/all', section: 'Blockchain' },
  { id: 'block-hash', label: 'Block by Hash', path: '/blockchain/by-hash', section: 'Blockchain' },

  // Wallet
  { id: 'create-wallet', label: 'Create Wallet', path: '/wallet/create', section: 'Wallet' },
  { id: 'wallet-info', label: 'Wallet Info', path: '/wallet/info', section: 'Wallet' },
  { id: 'wallet-balance', label: 'Wallet Balance', path: '/wallet/balance', section: 'Wallet' },
  { id: 'send-bitcoin', label: 'Send Bitcoin', path: '/wallet/send', section: 'Wallet' },
  { id: 'tx-history', label: 'Transaction History', path: '/wallet/history', section: 'Wallet' },
  { id: 'addresses', label: 'All Addresses', path: '/wallet/addresses', section: 'Wallet' },

  // Transactions
  { id: 'mempool', label: 'Mempool', path: '/transactions/mempool', section: 'Transactions' },
  { id: 'mempool-tx', label: 'Mempool Transaction', path: '/transactions/mempool-tx', section: 'Transactions' },
  { id: 'all-txs', label: 'All Transactions', path: '/transactions/all', section: 'Transactions' },
  { id: 'address-txs', label: 'Transactions by Address', path: '/transactions/by-address', section: 'Transactions' },

  // Mining
  { id: 'mining-info', label: 'Mining Info', path: '/mining/info', section: 'Mining' },
  { id: 'generate-blocks', label: 'Generate Blocks', path: '/mining/generate', section: 'Mining' },

  // Health
  { id: 'health', label: 'Health Status', path: '/health', section: 'Health' },
];

interface CommandPaletteProps {
  open: boolean;
  onClose: () => void;
}

function CommandPalette({ open, onClose }: CommandPaletteProps) {
  const navigate = useNavigate();
  const [search, setSearch] = useState('');
  const [selectedIdx, setSelectedIdx] = useState(0);

  const filtered = search
    ? commands.filter(
        (cmd) =>
          cmd.label.toLowerCase().includes(search.toLowerCase()) ||
          cmd.section.toLowerCase().includes(search.toLowerCase())
      )
    : commands;

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!open) return;

      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIdx(Math.min(selectedIdx + 1, filtered.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIdx(Math.max(selectedIdx - 1, 0));
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (filtered[selectedIdx]) {
          navigate(filtered[selectedIdx].path);
          onClose();
        }
      } else if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [open, selectedIdx, filtered, navigate, onClose]);

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center bg-black/50 pt-32">
      <div className="w-full max-w-lg rounded-lg border border-slate-700 bg-slate-800 shadow-lg">
        <div className="border-b border-slate-700 p-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-slate-500" />
            <input
              type="text"
              autoFocus
              placeholder="Type to search commands..."
              value={search}
              onChange={(e) => {
                setSearch(e.target.value);
                setSelectedIdx(0);
              }}
              className="w-full rounded-lg border border-slate-600 bg-slate-700 pl-10 pr-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
            />
          </div>
        </div>

        <div className="max-h-96 overflow-y-auto">
          {filtered.length === 0 ? (
            <div className="p-8 text-center text-slate-400">
              No commands found
            </div>
          ) : (
            <div className="p-2">
              {filtered.map((cmd, idx) => (
                <button
                  key={cmd.id}
                  onClick={() => {
                    navigate(cmd.path);
                    onClose();
                  }}
                  onMouseEnter={() => setSelectedIdx(idx)}
                  className={`w-full rounded-lg px-4 py-2 text-left transition-colors ${
                    selectedIdx === idx
                      ? 'bg-yellow-500 text-slate-900'
                      : 'text-slate-100 hover:bg-slate-700'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <div>
                      <div className="font-medium">{cmd.label}</div>
                      <div className="text-xs opacity-70">{cmd.section}</div>
                    </div>
                    <ArrowRight className="h-4 w-4 opacity-50" />
                  </div>
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="border-t border-slate-700 bg-slate-900 px-4 py-2 text-xs text-slate-500">
          <span className="mr-4">↑↓ Navigate</span>
          <span className="mr-4">↵ Select</span>
          <span>Esc Close</span>
        </div>
      </div>
    </div>
  );
}

export default CommandPalette;
