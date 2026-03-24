import { useLiveness } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function Liveness() {
  const { data, error, isLoading, refetch } = useLiveness();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Liveness Check</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && (
        <div>
          <div className={`card mb-4 ${data.success ? 'bg-green-900/20 border-green-500/50' : 'bg-red-900/20 border-red-500/50'}`}>
            <p className={`font-semibold ${data.success ? 'text-green-400' : 'text-red-400'}`}>
              {data.success ? '✓ System is alive' : '✗ Liveness check failed'}
            </p>
          </div>
          <JsonViewer data={data.data} title="Liveness Check Details" />
        </div>
      )}
    </div>
  );
}

