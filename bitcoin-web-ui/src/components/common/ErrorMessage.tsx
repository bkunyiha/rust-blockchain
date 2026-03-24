interface ErrorMessageProps {
  error: Error | string;
}

export function ErrorMessage({ error }: ErrorMessageProps) {
  const message = error instanceof Error ? error.message : error;
  return (
    <div className="bg-red-900/20 border border-red-500/50 rounded-lg p-4 text-red-200">
      <p className="font-semibold">Error</p>
      <p className="text-sm mt-1">{message}</p>
    </div>
  );
}

