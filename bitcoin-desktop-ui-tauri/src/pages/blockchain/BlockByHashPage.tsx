import { useState } from 'react';
import { Search } from 'lucide-react';
import { useInvoke } from '../../hooks/useInvoke';
import { commands } from '../../lib/commands';
import JsonViewer from '../../components/JsonViewer';
import { useAppStore } from '../../store/useAppStore';

function BlockByHashPage() {
  const [hash, setHash] = useState('');
  const setStatus = useAppStore((state) => state.setStatus);

  const { data, loading, error, execute } = useInvoke({
    onSuccess: () => {
      setStatus('Block fetched successfully');
    },
    onError: (err) => {
      setStatus(`Error: ${err}`);
    },
  });

  const handleSearch = async () => {
    if (!hash.trim()) {
      setStatus('Please enter a block hash');
      return;
    }
    await execute('get_block_by_hash', { hash: hash.trim() });
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Block by Hash</h1>

      <div className="rounded-lg border border-slate-700 bg-slate-800 p-6">
        <label className="block text-sm font-medium mb-2">Block Hash</label>
        <div className="flex gap-2">
          <input
            type="text"
            value={hash}
            onChange={(e) => setHash(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Enter block hash..."
            className="flex-1 rounded-lg border border-slate-600 bg-slate-700 px-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
          />
          <button
            onClick={handleSearch}
            disabled={loading}
            className="flex items-center gap-2 rounded-lg bg-yellow-500 px-6 py-2 font-medium text-slate-900 hover:bg-yellow-600 disabled:opacity-50"
          >
            <Search className="h-5 w-5" />
            {loading ? 'Searching...' : 'Search'}
          </button>
        </div>
      </div>

      {error && (
        <div className="rounded-lg border border-red-700 bg-red-900/20 p-8 text-red-200">
          <p>Failed to fetch block</p>
          <p className="text-sm opacity-75">{error}</p>
        </div>
      )}

      {data && (
        <JsonViewer data={data} title="Block Details" maxHeight={700} />
      )}
    </div>
  );
}

export default BlockByHashPage;
