import React from "react";
import { X, CheckCircle, AlertCircle, Info } from "lucide-react";
import { useToastStore, ToastType } from "../store/toastStore";

const icons: Record<ToastType, React.ReactNode> = {
  success: <CheckCircle size={16} className="text-green-400" />,
  error: <AlertCircle size={16} className="text-red-400" />,
  info: <Info size={16} className="text-blue-400" />,
};

const bgColors: Record<ToastType, string> = {
  success: "bg-green-900/80 border-green-700",
  error: "bg-red-900/80 border-red-700",
  info: "bg-blue-900/80 border-blue-700",
};

export default function ToastContainer() {
  const { toasts, removeToast } = useToastStore();

  if (toasts.length === 0) return null;

  return (
    <div className="fixed top-4 right-4 z-50 flex flex-col gap-2 max-w-sm">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`flex items-center gap-2 px-4 py-3 rounded-lg border shadow-lg text-sm animate-slide-in ${bgColors[toast.type]}`}
        >
          {icons[toast.type]}
          <span className="flex-1 text-gray-100">{toast.message}</span>
          <button onClick={() => removeToast(toast.id)} className="text-gray-400 hover:text-gray-200">
            <X size={14} />
          </button>
        </div>
      ))}
    </div>
  );
}
