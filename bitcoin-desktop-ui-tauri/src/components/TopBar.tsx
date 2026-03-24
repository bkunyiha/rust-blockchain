import { useState } from 'react';
import { Settings, Sun, Moon } from 'lucide-react';
import ConnectionBadge from './ConnectionBadge';
import SettingsDialog from './SettingsDialog';
import { useAppStore } from '../store/useAppStore';

function TopBar() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const { theme, toggleTheme } = useAppStore();

  return (
    <>
      <header className="border-b border-slate-700 bg-slate-900 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="h-8 w-8 rounded-lg bg-gradient-to-br from-yellow-400 to-yellow-600 flex items-center justify-center">
              <span className="text-sm font-bold text-slate-900">₿</span>
            </div>
            <h1 className="text-xl font-bold">Bitcoin Desktop UI</h1>
          </div>

          <div className="flex items-center gap-4">
            <ConnectionBadge />

            <button
              onClick={toggleTheme}
              className="rounded-lg bg-slate-800 p-2 hover:bg-slate-700 transition-colors"
              title="Toggle theme"
            >
              {theme === 'dark' ? (
                <Sun className="h-5 w-5 text-yellow-400" />
              ) : (
                <Moon className="h-5 w-5 text-blue-400" />
              )}
            </button>

            <button
              onClick={() => setSettingsOpen(true)}
              className="rounded-lg bg-slate-800 p-2 hover:bg-slate-700 transition-colors"
              title="Settings (Cmd+,)"
            >
              <Settings className="h-5 w-5" />
            </button>
          </div>
        </div>
      </header>

      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
    </>
  );
}

export default TopBar;
