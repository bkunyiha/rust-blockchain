import { useAppStore } from '../store/useAppStore';
import { formatDate } from '../lib/utils';

function StatusBar() {
  const { status, statusTimestamp } = useAppStore();

  if (!status) {
    return (
      <div className="border-t border-slate-700 bg-slate-900 px-6 py-2 text-sm text-slate-500">
        Ready
      </div>
    );
  }

  return (
    <div className="border-t border-slate-700 bg-slate-900 px-6 py-2 text-sm">
      <div className="flex items-center justify-between">
        <span className="text-slate-100">{status}</span>
        {statusTimestamp > 0 && (
          <span className="text-xs text-slate-500">
            {formatDate(new Date(statusTimestamp).toISOString())}
          </span>
        )}
      </div>
    </div>
  );
}

export default StatusBar;
