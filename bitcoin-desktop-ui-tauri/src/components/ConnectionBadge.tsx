import { useEffect, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { commands } from '../lib/commands';

function ConnectionBadge() {
  const [isConnected, setIsConnected] = useState(false);

  const { data } = useQuery({
    queryKey: ['connection'],
    queryFn: async () => {
      try {
        const result = await commands.checkConnection();
        return result;
      } catch {
        return { connected: false };
      }
    },
    refetchInterval: 30000, // 30 seconds
    retry: false,
  });

  useEffect(() => {
    if (data && typeof data === 'object' && 'connected' in data) {
      setIsConnected(data.connected as boolean);
    }
  }, [data]);

  return (
    <div className="flex items-center gap-2 rounded-full bg-slate-800 px-3 py-1.5">
      <div
        className={`h-2 w-2 rounded-full ${
          isConnected ? 'bg-green-500' : 'bg-red-500'
        }`}
      />
      <span className="text-sm font-medium">
        {isConnected ? 'Connected' : 'Disconnected'}
      </span>
    </div>
  );
}

export default ConnectionBadge;
