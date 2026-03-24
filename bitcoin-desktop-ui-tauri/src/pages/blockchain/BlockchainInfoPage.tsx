import { useQuery } from '@tanstack/react-query';
import { commands } from '../../lib/commands';
import DataCard, { CardItem } from '../../components/DataCard';
import { BlockchainInfo } from '../../types';
import { useAppStore } from '../../store/useAppStore';

function BlockchainInfoPage() {
  const setStatus = useAppStore((state) => state.setStatus);

  const { data, isLoading, error } = useQuery({
    queryKey: ['blockchainInfo'],
    queryFn: async () => {
      const result = await commands.getBlockchainInfo();
      return result as BlockchainInfo;
    },
    refetchInterval: 30000, // 30 seconds
    onSuccess: () => {
      setStatus('Blockchain info fetched successfully');
    },
    onError: (err) => {
      setStatus(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`);
    },
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-slate-400">Loading blockchain info...</div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="rounded-lg border border-red-700 bg-red-900/20 p-8 text-red-200">
        <p>Failed to load blockchain info</p>
        {error && <p className="text-sm opacity-75">{String(error)}</p>}
      </div>
    );
  }

  const items: CardItem[] = [
    {
      label: 'Current Height',
      value: data.height,
      format: 'number',
      copyable: true,
    },
    {
      label: 'Total Blocks',
      value: data.total_blocks,
      format: 'number',
      copyable: true,
    },
    {
      label: 'Difficulty',
      value: data.difficulty,
      format: 'number',
      copyable: true,
    },
    {
      label: 'Best Block Hash',
      value: data.best_block_hash,
      format: 'hash',
      copyable: true,
    },
  ];

  return <DataCard title="Blockchain Information" items={items} />;
}

export default BlockchainInfoPage;
