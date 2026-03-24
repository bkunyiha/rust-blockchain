import { useState } from 'react';
import { useAddressTransactions } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function AddressTransactions() {
  const [address, setAddress] = useState('');
  const { data, error, isLoading, refetch } = useAddressTransactions(address);

  const handleLoad = () => {
    if (address.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Address Transactions</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            placeholder="Enter address"
            className="input-field flex-1"
            onKeyPress={(e) => e.key === 'Enter' && handleLoad()}
          />
          <button onClick={handleLoad} className="btn-primary" disabled={!address.trim() || isLoading}>
            Load Transactions
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Address Transactions" />
      )}
      
      {!address && !isLoading && !data && (
        <div className="text-gray-400">Enter an address to view transactions</div>
      )}
    </div>
  );
}

