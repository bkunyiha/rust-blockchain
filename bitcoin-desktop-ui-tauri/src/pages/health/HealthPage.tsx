import { useEffect, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { RotateCcw, CheckCircle, XCircle } from 'lucide-react';
import { commands } from '../../lib/commands';
import { useAppStore } from '../../store/useAppStore';

interface HealthStatus {
  status: string;
  ok?: boolean;
  healthy?: boolean;
  ready?: boolean;
}

function HealthPage() {
  const setStatus = useAppStore((state) => state.setStatus);
  const [refetchInterval, setRefetchInterval] = useState(60000);

  // Health check
  const health = useQuery({
    queryKey: ['healthCheck'],
    queryFn: async () => {
      const result = await commands.healthCheck();
      return result as HealthStatus;
    },
    refetchInterval,
  });

  // Liveness check
  const liveness = useQuery({
    queryKey: ['livenessCheck'],
    queryFn: async () => {
      const result = await commands.livenessCheck();
      return result as HealthStatus;
    },
    refetchInterval,
  });

  // Readiness check
  const readiness = useQuery({
    queryKey: ['readinessCheck'],
    queryFn: async () => {
      const result = await commands.readinessCheck();
      return result as HealthStatus;
    },
    refetchInterval,
  });

  useEffect(() => {
    setStatus('Health checks completed');
  }, [health.data, liveness.data, readiness.data, setStatus]);

  const handleRefresh = () => {
    health.refetch();
    liveness.refetch();
    readiness.refetch();
  };

  const renderCheck = (
    title: string,
    data: HealthStatus | undefined,
    isLoading: boolean,
    error: Error | null
  ) => {
    let isHealthy = false;
    if (data) {
      isHealthy = !!(data.healthy || data.ok || data.ready || data.status === 'ok');
    }

    return (
      <div className="rounded-lg border border-slate-700 bg-slate-800 p-6">
        <div className="flex items-center gap-4">
          {isLoading ? (
            <div className="h-12 w-12 rounded-full bg-slate-700 flex items-center justify-center animate-spin">
              <RotateCcw className="h-6 w-6 text-slate-400" />
            </div>
          ) : error ? (
            <div className="h-12 w-12 rounded-full bg-red-900 flex items-center justify-center">
              <XCircle className="h-6 w-6 text-red-400" />
            </div>
          ) : isHealthy ? (
            <div className="h-12 w-12 rounded-full bg-green-900 flex items-center justify-center">
              <CheckCircle className="h-6 w-6 text-green-400" />
            </div>
          ) : (
            <div className="h-12 w-12 rounded-full bg-red-900 flex items-center justify-center">
              <XCircle className="h-6 w-6 text-red-400" />
            </div>
          )}

          <div className="flex-1">
            <h3 className="text-lg font-bold">{title}</h3>
            <p className="text-sm text-slate-400 mt-1">
              {isLoading
                ? 'Checking...'
                : error
                ? 'Failed to check'
                : isHealthy
                ? 'Healthy'
                : 'Unhealthy'}
            </p>
            {data && (
              <p className="text-xs text-slate-500 mt-2">
                Status: {data.status || 'unknown'}
              </p>
            )}
          </div>

          <button
            onClick={handleRefresh}
            className="rounded-lg bg-slate-700 p-2 hover:bg-slate-600 transition-colors"
            title="Refresh all checks"
          >
            <RotateCcw className="h-5 w-5" />
          </button>
        </div>
      </div>
    );
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Health Status</h1>
        <button
          onClick={handleRefresh}
          className="flex items-center gap-2 rounded-lg bg-yellow-500 px-4 py-2 font-medium text-slate-900 hover:bg-yellow-600 transition-colors"
        >
          <RotateCcw className="h-5 w-5" />
          Refresh All
        </button>
      </div>

      <div className="grid gap-4 sm:grid-cols-1 lg:grid-cols-3">
        {renderCheck(
          'Health Check',
          health.data,
          health.isLoading,
          health.error as Error | null
        )}
        {renderCheck(
          'Liveness Check',
          liveness.data,
          liveness.isLoading,
          liveness.error as Error | null
        )}
        {renderCheck(
          'Readiness Check',
          readiness.data,
          readiness.isLoading,
          readiness.error as Error | null
        )}
      </div>

      <div className="rounded-lg border border-slate-700 bg-slate-800 p-6">
        <h3 className="mb-4 font-bold">Information</h3>
        <ul className="space-y-2 text-sm text-slate-400">
          <li>
            <strong>Health Check:</strong> Verifies basic service availability
          </li>
          <li>
            <strong>Liveness Check:</strong> Confirms the service is running
          </li>
          <li>
            <strong>Readiness Check:</strong> Ensures the service is ready to serve requests
          </li>
          <li>
            <strong>Auto-refresh:</strong> Checks are automatically refreshed every 60 seconds
          </li>
        </ul>
      </div>
    </div>
  );
}

export default HealthPage;
