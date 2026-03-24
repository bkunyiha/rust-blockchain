import { useQuery } from '@tanstack/react-query';
import { commands } from '../../lib/commands';
import DataTable, { Column } from '../../components/DataTable';
import { BlockSummary } from '../../types';
import { useAppStore } from '../../store/useAppStore';
import { truncateHash, formatDate } from '../../lib/utils';

function LatestBlocksPage() {
  const setStatus = useAppStore((state) => state.setStatus);

  const { data, isLoading, error } = useQuery({
    queryKey: ['latestBlocks'],
    queryFn: async () => {
      const result = await commands.getLatestBlocks();
      return result as BlockSummary[];
    },
    onSuccess: () => {
      setStatus('Latest blocks fetched successfully');
    },
    onError: (err) => {
      setStatus(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`);
    },
  });

  const columns: Column[] = [
    { key: 'height', label: 'Height', sortable: true },
    {
      key: 'hash',
      label: 'Hash',
      sortable: true,
      render: (value) => truncateHash(value),
    },
    {
      key: 'time',
      label: 'Time',
      sortable: true,
      render: (value) => {
        try {
          const date = new Date(value * 1000);
          return date.toLocaleString();
        } catch {
          return value;
        }
      },
    },
    { key: 'tx_count', label: 'Transactions', sortable: true },
  ];

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-slate-400">Loading latest blocks...</div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="rounded-lg border border-red-700 bg-red-900/20 p-8 text-red-200">
        <p>Failed to load latest blocks</p>
        {error && <p className="text-sm opacity-75">{String(error)}</p>}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Latest Blocks</h1>
      <DataTable columns={columns} data={data} />
    </div>
  );
}

export default LatestBlocksPage;
