import { useQuery } from '@tanstack/react-query';
import { commands } from '../../lib/commands';
import JsonViewer from '../../components/JsonViewer';
import { useAppStore } from '../../store/useAppStore';

function AllTransactionsPage() {
  const setStatus = useAppStore((state) => state.setStatus);

  const { data, isLoading, error } = useQuery({
    queryKey: ['allTransactions'],
    queryFn: async () => {
      const result = await commands.getAllTransactions();
      return result;
    },
    onSuccess: () => {
      setStatus('All transactions fetched successfully');
    },
    onError: (err) => {
      setStatus(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`);
    },
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-slate-400">Loading all transactions...</div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="rounded-lg border border-red-700 bg-red-900/20 p-8 text-red-200">
        <p>Failed to load transactions</p>
        {error && <p className="text-sm opacity-75">{String(error)}</p>}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">All Transactions</h1>
      <JsonViewer data={data} maxHeight={700} />
    </div>
  );
}

export default AllTransactionsPage;
