import { useState } from 'react';
import { X, Eye, EyeOff, Check, AlertCircle } from 'lucide-react';
import { useInvoke } from '../hooks/useInvoke';
import { commands } from '../lib/commands';

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const [baseUrl, setBaseUrl] = useState('');
  const [apiKey, setApiKey] = useState('');
  const [showApiKey, setShowApiKey] = useState(false);
  const [testResult, setTestResult] = useState<{
    success: boolean;
    message: string;
  } | null>(null);

  const { execute: updateConfig, loading: updating } = useInvoke({
    onSuccess: () => {
      setTestResult({
        success: true,
        message: 'Configuration saved successfully',
      });
      setTimeout(() => {
        onOpenChange(false);
      }, 1500);
    },
    onError: (error) => {
      setTestResult({
        success: false,
        message: `Failed to save: ${error}`,
      });
    },
  });

  const { execute: checkConn, loading: testing } = useInvoke({
    onSuccess: () => {
      setTestResult({
        success: true,
        message: 'Connection successful',
      });
    },
    onError: (error) => {
      setTestResult({
        success: false,
        message: `Connection failed: ${error}`,
      });
    },
  });

  const handleSave = async () => {
    if (!baseUrl || !apiKey) {
      setTestResult({
        success: false,
        message: 'Base URL and API Key are required',
      });
      return;
    }
    await updateConfig('update_config', { baseUrl, apiKey });
  };

  const handleTestConnection = async () => {
    await checkConn('check_connection');
  };

  const handleReset = () => {
    setBaseUrl('');
    setApiKey('');
    setTestResult(null);
    setShowApiKey(false);
  };

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-md rounded-lg border border-slate-700 bg-slate-800 p-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-lg font-bold">Settings</h2>
          <button
            onClick={() => onOpenChange(false)}
            className="rounded-lg p-1 hover:bg-slate-700"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <div className="space-y-4">
          {/* Base URL */}
          <div>
            <label className="block text-sm font-medium mb-2">
              Base URL
            </label>
            <input
              type="text"
              value={baseUrl}
              onChange={(e) => setBaseUrl(e.target.value)}
              placeholder="https://api.example.com"
              className="w-full rounded-lg border border-slate-600 bg-slate-700 px-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
            />
          </div>

          {/* API Key */}
          <div>
            <label className="block text-sm font-medium mb-2">
              API Key
            </label>
            <div className="relative">
              <input
                type={showApiKey ? 'text' : 'password'}
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="Enter your API key"
                className="w-full rounded-lg border border-slate-600 bg-slate-700 px-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
              />
              <button
                onClick={() => setShowApiKey(!showApiKey)}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-300"
              >
                {showApiKey ? (
                  <EyeOff className="h-5 w-5" />
                ) : (
                  <Eye className="h-5 w-5" />
                )}
              </button>
            </div>
          </div>

          {/* Test Result */}
          {testResult && (
            <div
              className={`flex items-center gap-2 rounded-lg p-3 text-sm ${
                testResult.success
                  ? 'bg-green-900 text-green-200'
                  : 'bg-red-900 text-red-200'
              }`}
            >
              {testResult.success ? (
                <Check className="h-4 w-4 flex-shrink-0" />
              ) : (
                <AlertCircle className="h-4 w-4 flex-shrink-0" />
              )}
              {testResult.message}
            </div>
          )}

          {/* Buttons */}
          <div className="flex gap-2 pt-4">
            <button
              onClick={handleTestConnection}
              disabled={testing || updating}
              className="flex-1 rounded-lg bg-slate-700 px-4 py-2 text-sm font-medium hover:bg-slate-600 disabled:opacity-50"
            >
              {testing ? 'Testing...' : 'Test Connection'}
            </button>
            <button
              onClick={handleSave}
              disabled={updating || testing}
              className="flex-1 rounded-lg bg-yellow-500 px-4 py-2 text-sm font-medium text-slate-900 hover:bg-yellow-600 disabled:opacity-50"
            >
              {updating ? 'Saving...' : 'Save'}
            </button>
          </div>

          <button
            onClick={handleReset}
            className="w-full rounded-lg bg-slate-700 px-4 py-2 text-sm font-medium text-slate-300 hover:bg-slate-600"
          >
            Reset to Defaults
          </button>
        </div>
      </div>
    </div>
  );
}

export default SettingsDialog;
