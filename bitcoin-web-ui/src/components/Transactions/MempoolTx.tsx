import { useState } from 'react';
import { useMempoolTransaction } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function MempoolTx() {
  const [txid, setTxid] = useState('');
  const { data, error, isLoading, refetch } = useMempoolTransaction(txid);

  const handleSearch = () => {
    if (txid.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Mempool Transaction</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={txid}
            onChange={(e) => setTxid(e.target.value)}
            placeholder="Enter transaction ID"
            className="input-field flex-1"
            onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          />
          <button onClick={handleSearch} className="btn-primary" disabled={!txid.trim() || isLoading}>
            Search
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Transaction Details" />
      )}
      
      {!txid && !isLoading && !data && (
        <div className="text-gray-400">Enter a transaction ID to search</div>
      )}
    </div>
  );
}

