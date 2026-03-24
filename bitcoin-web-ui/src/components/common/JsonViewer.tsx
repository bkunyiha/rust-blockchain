import { useState } from 'react';
import toast from 'react-hot-toast';

interface JsonViewerProps {
  data: any;
  title?: string;
}

export function JsonViewer({ data, title }: JsonViewerProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const jsonString = JSON.stringify(data, null, 2);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(jsonString);
    toast.success('Copied to clipboard!');
  };

  return (
    <div className="bg-gray-800 rounded-lg border border-gray-700">
      {title && (
        <div className="flex items-center justify-between px-4 py-2 border-b border-gray-700">
          <h3 className="text-sm font-semibold text-gray-300">{title}</h3>
          <div className="flex gap-2">
            <button
              onClick={() => setIsExpanded(!isExpanded)}
              className="px-3 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded text-gray-300"
            >
              {isExpanded ? 'Collapse' : 'Expand'}
            </button>
            <button
              onClick={copyToClipboard}
              className="px-3 py-1 text-xs bg-bitcoin-orange hover:bg-bitcoin-gold rounded text-white"
            >
              Copy
            </button>
          </div>
        </div>
      )}
      <div className={`overflow-auto ${isExpanded ? 'max-h-[600px]' : 'max-h-[300px]'}`}>
        <pre className="p-4 text-xs text-gray-300 font-mono">
          {jsonString}
        </pre>
      </div>
    </div>
  );
}

