import { useMempool } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function Mempool() {
  const { data, error, isLoading, refetch } = useMempool();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Mempool</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && <JsonViewer data={data.data} title="Mempool Transactions" />}
    </div>
  );
}

