import { useState } from 'react';
import { useAllTransactions } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';
import { getApiClient } from '../../services/api';

export function AllTransactions() {
  const [page, setPage] = useState(1);
  const [limit] = useState(100); // Higher default limit for longer display
  const { data, error, isLoading, refetch } = useAllTransactions(page, limit);
  const [isLoadingData, setIsLoadingData] = useState(false);
  const [allTransactions, setAllTransactions] = useState<any[]>([]);
  const [isLoadingAll, setIsLoadingAll] = useState(false);

  const handleLoad = async () => {
    setIsLoadingData(true);
    await refetch();
    setIsLoadingData(false);
  };

  const handleLoadPage = async (pageNum: number) => {
    setPage(pageNum);
    setIsLoadingData(true);
    await refetch();
    setIsLoadingData(false);
  };

  const handleLoadAll = async () => {
    setIsLoadingAll(true);
    setAllTransactions([]);
    
    try {
      let currentPage = 1;
      let allItems: any[] = [];
      let hasMore = true;

      while (hasMore) {
        const response = await getApiClient().getAllTransactions(currentPage, limit);
        if (response.success && response.data) {
          const paginatedData = response.data;
          allItems = [...allItems, ...paginatedData.items];
          
          if (paginatedData.has_next) {
            currentPage++;
          } else {
            hasMore = false;
          }
        } else {
          hasMore = false;
        }
      }

      setAllTransactions(allItems);
    } catch (err) {
      console.error('Error loading all transactions:', err);
    } finally {
      setIsLoadingAll(false);
    }
  };

  if (isLoading || isLoadingData) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  const displayData = allTransactions.length > 0 ? allTransactions : (data?.data?.items || []);
  const paginationInfo = data?.data && allTransactions.length === 0 ? data.data : null;

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">All Transactions</h1>
        <div className="flex gap-2">
          <button onClick={handleLoad} className="btn-primary" disabled={isLoading}>
            Load Page {page}
          </button>
          <button onClick={handleLoadAll} className="btn-primary" disabled={isLoadingAll}>
            {isLoadingAll ? 'Loading All...' : 'Load All Transactions'}
          </button>
        </div>
      </div>

      {paginationInfo && allTransactions.length === 0 && (
        <div className="mb-4 text-sm text-gray-400">
          Showing {paginationInfo.items?.length || 0} of {paginationInfo.total} transactions
          {paginationInfo.total_pages > 1 && ` (Page ${paginationInfo.page} of ${paginationInfo.total_pages})`}
        </div>
      )}

      {paginationInfo && paginationInfo.total_pages > 1 && allTransactions.length === 0 && (
        <div className="mb-4 flex gap-2">
          <button
            onClick={() => handleLoadPage(paginationInfo.page - 1)}
            disabled={!paginationInfo.has_prev || isLoading}
            className="btn-secondary"
          >
            Previous
          </button>
          <button
            onClick={() => handleLoadPage(paginationInfo.page + 1)}
            disabled={!paginationInfo.has_next || isLoading}
            className="btn-secondary"
          >
            Next
          </button>
        </div>
      )}

      {allTransactions.length > 0 && (
        <div className="mb-4 text-sm text-gray-400">
          Loaded {allTransactions.length} transactions (all pages)
        </div>
      )}
      
      {displayData.length > 0 ? (
        <div className="max-h-[80vh] overflow-y-auto">
          <JsonViewer 
            data={displayData} 
            title={`All Transactions${allTransactions.length > 0 ? ` (${allTransactions.length} total)` : paginationInfo ? ` (${paginationInfo.total} total)` : ''}`} 
          />
        </div>
      ) : (
        <div className="text-gray-400">Click "Load Page {page}" or "Load All Transactions" to fetch data</div>
      )}
    </div>
  );
}

