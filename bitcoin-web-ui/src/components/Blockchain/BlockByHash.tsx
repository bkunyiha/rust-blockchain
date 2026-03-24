import { useState } from 'react';
import { useBlockByHash } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function BlockByHash() {
  const [hash, setHash] = useState('');
  const { data, error, isLoading, refetch } = useBlockByHash(hash);

  const handleSearch = () => {
    if (hash.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Block by Hash</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={hash}
            onChange={(e) => setHash(e.target.value)}
            placeholder="Enter block hash"
            className="input-field flex-1"
            onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          />
          <button onClick={handleSearch} className="btn-primary" disabled={!hash.trim() || isLoading}>
            Search
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Block Details" />
      )}
      
      {!hash && !isLoading && !data && (
        <div className="text-gray-400">Enter a block hash to search</div>
      )}
    </div>
  );
}

