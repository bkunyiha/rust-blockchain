import { useQuery } from '@tanstack/react-query';
import { commands } from '../../lib/commands';
import DataCard, { CardItem } from '../../components/DataCard';
import { useAppStore } from '../../store/useAppStore';

interface MiningInfo {
  difficulty: number;
  hashrate: number;
  blocks_mined: number;
  network_difficulty: number;
}

function MiningInfoPage() {
  const setStatus = useAppStore((state) => state.setStatus);

  const { data, isLoading, error } = useQuery({
    queryKey: ['miningInfo'],
    queryFn: async () => {
      const result = await commands.getMiningInfo();
      return result as MiningInfo;
    },
    onSuccess: () => {
      setStatus('Mining info fetched successfully');
    },
    onError: (err) => {
      setStatus(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`);
    },
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-slate-400">Loading mining info...</div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="rounded-lg border border-red-700 bg-red-900/20 p-8 text-red-200">
        <p>Failed to load mining info</p>
        {error && <p className="text-sm opacity-75">{String(error)}</p>}
      </div>
    );
  }

  const items: CardItem[] = [
    {
      label: 'Difficulty',
      value: data.difficulty,
      format: 'number',
      copyable: true,
    },
    {
      label: 'Hashrate',
      value: data.hashrate,
      format: 'number',
      copyable: true,
    },
    {
      label: 'Blocks Mined',
      value: data.blocks_mined,
      format: 'number',
      copyable: true,
    },
    {
      label: 'Network Difficulty',
      value: data.network_difficulty,
      format: 'number',
      copyable: true,
    },
  ];

  return <DataCard title="Mining Information" items={items} />;
}

export default MiningInfoPage;
