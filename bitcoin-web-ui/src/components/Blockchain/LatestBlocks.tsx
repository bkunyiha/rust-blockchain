import { useLatestBlocks } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';
import { formatDate } from '../../utils/date';

export function LatestBlocks() {
  const { data, error, isLoading, refetch } = useLatestBlocks();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  const blocks = data?.data || [];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Latest Blocks</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {blocks.length > 0 ? (
        <div className="space-y-4">
          {blocks.map((block) => (
            <div key={block.hash} className="card">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-4 mb-2">
                    <span className="text-bitcoin-orange font-semibold">Block #{block.height}</span>
                    <span className="text-sm text-gray-400">
                      {formatDate(block.timestamp)}
                    </span>
                  </div>
                  <div className="text-sm space-y-1">
                    <div>
                      <span className="text-gray-400">Hash: </span>
                      <span className="text-gray-300 font-mono text-xs break-all">{block.hash}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Transactions: </span>
                      <span className="text-white">{block.transaction_count}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Difficulty: </span>
                      <span className="text-white">{block.difficulty}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="text-gray-400">No blocks found</div>
      )}
      
      {blocks.length > 0 && (
        <div className="mt-6">
          <JsonViewer data={blocks} title="Raw JSON Data" />
        </div>
      )}
    </div>
  );
}

