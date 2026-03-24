import { useState, useEffect } from 'react';
import { useAllBlocks } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';
import type { BlockSummary } from '../../types/api';
import { getApiClient } from '../../services/api';

export function AllBlocks() {
  const [allBlocks, setAllBlocks] = useState<BlockSummary[]>([]);
  const [currentPage] = useState(0);
  const [limit] = useState(100); // Fetch 100 blocks per page
  const { data, error, isLoading, refetch } = useAllBlocks(currentPage, limit);
  const [isLoadingData, setIsLoadingData] = useState(false);
  const [isLoadingAll, setIsLoadingAll] = useState(false);

  const handleLoad = async () => {
    setIsLoadingData(true);
    await refetch();
    setIsLoadingData(false);
  };

  const handleLoadAll = async () => {
    setIsLoadingAll(true);
    setAllBlocks([]);
    
    try {
      const allBlocksData: BlockSummary[] = [];
      let page = 0;
      let hasMore = true;

      while (hasMore) {
        const response = await getApiClient().getAllBlocks(page, limit);
        if (response.data?.items) {
          allBlocksData.push(...response.data.items);
          hasMore = response.data.has_next;
          page++;
        } else {
          hasMore = false;
        }
      }

      setAllBlocks(allBlocksData);
    } catch (err) {
      console.error('Error loading all blocks:', err);
    } finally {
      setIsLoadingAll(false);
    }
  };

  // Update allBlocks when data changes
  useEffect(() => {
    if (data?.data?.items) {
      setAllBlocks(data.data.items);
    }
  }, [data]);

  if (isLoading || isLoadingData) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  const paginatedData = data?.data;
  const blocksToShow = allBlocks.length > 0 ? allBlocks : paginatedData?.items || [];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">All Blocks</h1>
        <div className="flex gap-2">
          <button onClick={handleLoad} className="btn-primary" disabled={isLoading}>
            Load Page
          </button>
          <button onClick={handleLoadAll} className="btn-primary" disabled={isLoadingAll}>
            {isLoadingAll ? 'Loading All...' : 'Load All Blocks'}
          </button>
        </div>
      </div>

      {paginatedData && (
        <div className="mb-4 text-sm text-gray-400">
          Showing page {paginatedData.page + 1} of {paginatedData.total_pages} 
          ({paginatedData.total} total blocks)
        </div>
      )}
      
      {blocksToShow.length > 0 ? (
        <JsonViewer data={blocksToShow} title={allBlocks.length > 0 ? `All Blocks (${allBlocks.length})` : 'Blocks'} />
      ) : (
        <div className="text-gray-400">Click "Load All Blocks" to fetch data</div>
      )}
    </div>
  );
}

