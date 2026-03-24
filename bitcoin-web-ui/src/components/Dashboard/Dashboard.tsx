import { useBlockchainInfo } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { StatCard } from '../common/StatCard';
import { formatDate } from '../../utils/date';

export function Dashboard() {
  const { data, error, isLoading } = useBlockchainInfo(5000); // Auto-refresh every 5 seconds

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  const info = data?.data;

  if (!info) {
    return <div className="text-gray-400">No blockchain data available</div>;
  }

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Dashboard</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <StatCard
          label="Block Height"
          value={info.height}
          icon={<span className="text-2xl">📊</span>}
        />
        <StatCard
          label="Total Blocks"
          value={info.total_blocks}
          icon={<span className="text-2xl">🧱</span>}
        />
        <StatCard
          label="Total Transactions"
          value={info.total_transactions}
          icon={<span className="text-2xl">💸</span>}
        />
        <StatCard
          label="Mempool Size"
          value={info.mempool_size}
          icon={<span className="text-2xl">⏳</span>}
        />
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <h2 className="text-lg font-semibold text-white mb-4">Blockchain Info</h2>
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-400">Difficulty:</span>
              <span className="text-white font-mono">{info.difficulty}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Last Block Hash:</span>
              <span className="text-white font-mono text-xs break-all">{info.last_block_hash}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Last Block Time:</span>
              <span className="text-white">{formatDate(info.last_block_timestamp)}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

