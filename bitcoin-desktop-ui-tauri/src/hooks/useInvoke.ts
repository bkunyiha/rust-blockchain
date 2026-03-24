import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface UseInvokeOptions {
  onSuccess?: (data: any) => void;
  onError?: (error: string) => void;
}

interface UseInvokeResult<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
  execute: (command: string, payload?: Record<string, any>) => Promise<T | null>;
}

export function useInvoke<T = any>(
  options: UseInvokeOptions = {}
): UseInvokeResult<T> {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const execute = useCallback(
    async (command: string, payload?: Record<string, any>) => {
      setLoading(true);
      setError(null);

      try {
        const result = await invoke<T>(command, payload);
        setData(result);
        options.onSuccess?.(result);
        return result;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : String(err);
        setError(errorMessage);
        options.onError?.(errorMessage);
        return null;
      } finally {
        setLoading(false);
      }
    },
    [options]
  );

  return { data, loading, error, execute };
}
