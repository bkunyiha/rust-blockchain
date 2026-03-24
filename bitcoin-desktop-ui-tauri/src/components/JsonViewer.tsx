import { useState, useMemo } from 'react';
import { ChevronRight, ChevronDown, Copy, Search } from 'lucide-react';
import { useClipboard } from '../hooks/useClipboard';
import { stringifyJSON } from '../lib/utils';

interface JsonViewerProps {
  data: any;
  title?: string;
  maxHeight?: number;
}

type SearchMatches = Set<string>;

function JsonViewer({ data, title, maxHeight = 600 }: JsonViewerProps) {
  const [expanded, setExpanded] = useState<Set<string>>(new Set());
  const [searchTerm, setSearchTerm] = useState('');
  const { copy } = useClipboard();

  const jsonString = useMemo(
    () => stringifyJSON(data),
    [data]
  );

  const searchMatches: SearchMatches = useMemo(() => {
    if (!searchTerm) return new Set();

    const matches = new Set<string>();
    const lowerTerm = searchTerm.toLowerCase();

    const traverse = (obj: any, path: string = ''): void => {
      if (typeof obj === 'string' && obj.toLowerCase().includes(lowerTerm)) {
        matches.add(path || 'root');
      }
      if (typeof obj === 'object' && obj !== null) {
        Object.keys(obj).forEach((key) => {
          const newPath = path ? `${path}.${key}` : key;
          if (key.toLowerCase().includes(lowerTerm)) {
            matches.add(newPath);
          }
          traverse(obj[key], newPath);
        });
      }
    };

    traverse(data);
    return matches;
  }, [searchTerm, data]);

  const toggleNode = (path: string) => {
    const newExpanded = new Set(expanded);
    if (newExpanded.has(path)) {
      newExpanded.delete(path);
    } else {
      newExpanded.add(path);
    }
    setExpanded(newExpanded);
  };

  const expandAll = () => {
    const all = new Set<string>();
    const traverse = (obj: any, path: string = ''): void => {
      if (typeof obj === 'object' && obj !== null) {
        if (path) all.add(path);
        Object.keys(obj).forEach((key) => {
          const newPath = path ? `${path}.${key}` : key;
          traverse(obj[key], newPath);
        });
      }
    };
    traverse(data);
    setExpanded(all);
  };

  const collapseAll = () => setExpanded(new Set());

  const renderValue = (
    value: any,
    path: string = '',
    depth: number = 0
  ): React.ReactNode => {
    if (depth > 50) {
      return <span className="text-slate-500">...</span>;
    }

    const isObject = typeof value === 'object' && value !== null;
    const isArray = Array.isArray(value);
    const isExpandable = isObject && (isArray ? value.length > 0 : Object.keys(value).length > 0);
    const isExpanded = expanded.has(path);

    if (!isExpandable) {
      return <JsonPrimitive value={value} />;
    }

    return (
      <div key={path}>
        <button
          onClick={() => toggleNode(path)}
          className="text-slate-400 hover:text-slate-200 inline-flex items-center gap-1"
        >
          {isExpanded ? (
            <ChevronDown className="h-4 w-4" />
          ) : (
            <ChevronRight className="h-4 w-4" />
          )}
          {isArray ? `[${value.length}]` : `{${Object.keys(value).length}}`}
        </button>

        {isExpanded && (
          <div className="ml-4 border-l border-slate-700 pl-3">
            {isArray ? (
              value.map((item: any, idx: number) => (
                <div key={idx} className="py-1">
                  <span className="text-blue-400">[{idx}]:</span>{' '}
                  {renderValue(item, `${path}[${idx}]`, depth + 1)}
                </div>
              ))
            ) : (
              Object.entries(value).map(([key, val]: [string, any]) => {
                const childPath = path ? `${path}.${key}` : key;
                const isMatched = searchMatches.has(childPath);

                return (
                  <div
                    key={key}
                    className={isMatched ? 'bg-yellow-900/20' : ''}
                  >
                    <span className="text-purple-400">{key}</span>
                    <span className="text-slate-400">: </span>
                    {renderValue(val, childPath, depth + 1)}
                  </div>
                );
              })
            )}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="rounded-lg border border-slate-700 bg-slate-800 p-4">
      {title && <h3 className="mb-4 text-lg font-bold">{title}</h3>}

      <div className="mb-4 flex gap-2">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-500" />
          <input
            type="text"
            placeholder="Search..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full rounded-lg border border-slate-600 bg-slate-700 pl-10 pr-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
          />
        </div>
        <button
          onClick={expandAll}
          className="rounded-lg bg-slate-700 px-3 py-2 text-sm hover:bg-slate-600"
        >
          Expand
        </button>
        <button
          onClick={collapseAll}
          className="rounded-lg bg-slate-700 px-3 py-2 text-sm hover:bg-slate-600"
        >
          Collapse
        </button>
        <button
          onClick={() => copy(jsonString)}
          className="flex items-center gap-2 rounded-lg bg-yellow-500 px-3 py-2 text-sm font-medium text-slate-900 hover:bg-yellow-600"
        >
          <Copy className="h-4 w-4" />
          Copy
        </button>
      </div>

      <div
        className="overflow-auto rounded-lg bg-slate-900 p-4 font-mono text-sm"
        style={{ maxHeight: `${maxHeight}px` }}
      >
        <div className="space-y-1">
          {renderValue(data)}
        </div>
      </div>
    </div>
  );
}

function JsonPrimitive({ value }: { value: any }) {
  if (value === null) {
    return <span className="text-slate-500">null</span>;
  }
  if (typeof value === 'boolean') {
    return <span className="text-pink-400">{String(value)}</span>;
  }
  if (typeof value === 'number') {
    return <span className="text-orange-400">{value}</span>;
  }
  if (typeof value === 'string') {
    return <span className="text-green-400">"{value}"</span>;
  }
  return <span className="text-slate-400">{String(value)}</span>;
}

export default JsonViewer;
