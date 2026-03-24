import { AlertCircle } from 'lucide-react';

interface ConfirmDialogProps {
  open: boolean;
  onConfirm: () => void;
  onCancel: () => void;
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: 'default' | 'destructive';
}

function ConfirmDialog({
  open,
  onConfirm,
  onCancel,
  title,
  description,
  confirmLabel = 'Confirm',
  cancelLabel = 'Cancel',
  variant = 'default',
}: ConfirmDialogProps) {
  if (!open) return null;

  const confirmButtonClass =
    variant === 'destructive'
      ? 'bg-red-600 hover:bg-red-700'
      : 'bg-yellow-500 hover:bg-yellow-600 text-slate-900';

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-md rounded-lg border border-slate-700 bg-slate-800 p-6">
        <div className="mb-4 flex items-start gap-3">
          <AlertCircle className="h-6 w-6 flex-shrink-0 text-yellow-500" />
          <div>
            <h2 className="text-lg font-bold">{title}</h2>
            <p className="mt-2 text-sm text-slate-400">{description}</p>
          </div>
        </div>

        <div className="flex gap-2 pt-4">
          <button
            onClick={onCancel}
            className="flex-1 rounded-lg bg-slate-700 px-4 py-2 font-medium hover:bg-slate-600"
          >
            {cancelLabel}
          </button>
          <button
            onClick={onConfirm}
            className={`flex-1 rounded-lg px-4 py-2 font-medium text-white ${confirmButtonClass}`}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}

export default ConfirmDialog;
