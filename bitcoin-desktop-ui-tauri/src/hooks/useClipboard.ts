import { useState, useCallback } from 'react';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useToast } from '../components/Toast';

export function useClipboard() {
  const { showToast } = useToast();
  const [copied, setCopied] = useState(false);

  const copy = useCallback(
    async (text: string) => {
      try {
        await writeText(text);
        setCopied(true);
        showToast({
          type: 'success',
          message: 'Copied to clipboard',
        });
        setTimeout(() => setCopied(false), 2000);
      } catch (error) {
        const errorMessage =
          error instanceof Error ? error.message : 'Failed to copy';
        showToast({
          type: 'error',
          message: errorMessage,
        });
      }
    },
    [showToast]
  );

  return { copy, copied };
}
