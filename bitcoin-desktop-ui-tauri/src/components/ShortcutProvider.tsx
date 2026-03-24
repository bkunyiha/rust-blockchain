import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAppStore } from '../store/useAppStore';
import CommandPalette from './CommandPalette';

interface ShortcutProviderProps {
  children: React.ReactNode;
}

export function ShortcutProvider({ children }: ShortcutProviderProps) {
  const navigate = useNavigate();
  const { toggleTheme } = useAppStore();
  const [commandPaletteOpen, setCommandPaletteOpen] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const isMeta = e.ctrlKey || e.metaKey;

      // Cmd/Ctrl+K: Command palette
      if (isMeta && e.key === 'k') {
        e.preventDefault();
        setCommandPaletteOpen((prev) => !prev);
      }

      // Cmd/Ctrl+1-5: Navigate sections
      if (isMeta && e.key >= '1' && e.key <= '5') {
        e.preventDefault();
        const sections = [
          '/blockchain/info',
          '/wallet/create',
          '/transactions/mempool',
          '/mining/info',
          '/health',
        ];
        const idx = parseInt(e.key) - 1;
        if (idx < sections.length) {
          navigate(sections[idx]);
        }
      }

      // Cmd/Ctrl+,: Settings
      if (isMeta && e.key === ',') {
        e.preventDefault();
        // Settings dialog is opened in TopBar
        const settingsBtn = document.querySelector('[title="Settings (Cmd+,)"]');
        if (settingsBtn) {
          (settingsBtn as HTMLButtonElement).click();
        }
      }

      // Cmd/Ctrl+D: Toggle theme
      if (isMeta && e.key === 'd') {
        e.preventDefault();
        toggleTheme();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [navigate, toggleTheme]);

  return (
    <>
      {children}
      <CommandPalette
        open={commandPaletteOpen}
        onClose={() => setCommandPaletteOpen(false)}
      />
    </>
  );
}
