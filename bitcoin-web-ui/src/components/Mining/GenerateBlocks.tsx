import { useState } from 'react';
import { useGenerateBlocks } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { JsonViewer } from '../common/JsonViewer';

export function GenerateBlocks() {
  const [address, setAddress] = useState('');
  const [nblocks, setNblocks] = useState('1');
  const [maxtries, setMaxtries] = useState('');
  const generateBlocks = useGenerateBlocks();

  const handleGenerate = () => {
    if (!address.trim() || !nblocks.trim()) {
      return;
    }

    const nblocksNum = parseInt(nblocks);
    if (isNaN(nblocksNum) || nblocksNum <= 0) {
      return;
    }

    const maxtriesNum = maxtries.trim() ? parseInt(maxtries) : undefined;
    if (maxtries.trim() && (isNaN(maxtriesNum!) || maxtriesNum! <= 0)) {
      return;
    }

    generateBlocks.mutate({
      address: address.trim(),
      nblocks: nblocksNum,
      maxtries: maxtriesNum,
    });
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Generate Blocks</h1>
      
      <div className="card mb-6">
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-300 mb-2">Mining Reward Address</label>
            <input
              type="text"
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              placeholder="Enter address to receive mining rewards"
              className="input-field"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-300 mb-2">Number of Blocks</label>
            <input
              type="number"
              value={nblocks}
              onChange={(e) => setNblocks(e.target.value)}
              placeholder="1"
              className="input-field"
              min="1"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-300 mb-2">Max Tries (optional)</label>
            <input
              type="number"
              value={maxtries}
              onChange={(e) => setMaxtries(e.target.value)}
              placeholder="Leave empty for default"
              className="input-field"
              min="1"
            />
          </div>
          <button
            onClick={handleGenerate}
            className="btn-primary"
            disabled={generateBlocks.isPending || !address.trim() || !nblocks.trim()}
          >
            {generateBlocks.isPending ? 'Generating...' : 'Generate Blocks'}
          </button>
        </div>
      </div>

      {generateBlocks.isPending && <LoadingSpinner />}
      
      {generateBlocks.data?.data && (
        <div>
          <div className="card mb-4 bg-green-900/20 border-green-500/50">
            <p className="text-green-400 font-semibold mb-2">Blocks Generated Successfully!</p>
          </div>
          <JsonViewer data={generateBlocks.data.data} title="Generation Result" />
        </div>
      )}
    </div>
  );
}

