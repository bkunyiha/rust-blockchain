import { create } from 'zustand';

interface AppStore {
  activeMenu: string;
  expandedSections: string[];
  theme: 'dark' | 'light';
  status: string;
  statusTimestamp: number;
  toggleTheme: () => void;
  setStatus: (message: string) => void;
  toggleSection: (section: string) => void;
  setActiveMenu: (menu: string) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  activeMenu: '/blockchain/info',
  expandedSections: ['blockchain', 'wallet'],
  theme: 'dark',
  status: '',
  statusTimestamp: 0,

  toggleTheme: () =>
    set((state) => {
      const newTheme = state.theme === 'dark' ? 'light' : 'dark';
      if (typeof document !== 'undefined') {
        document.documentElement.classList.toggle('dark', newTheme === 'dark');
        document.documentElement.classList.toggle('light', newTheme === 'light');
      }
      return { theme: newTheme };
    }),

  setStatus: (message: string) =>
    set({
      status: message,
      statusTimestamp: Date.now(),
    }),

  toggleSection: (section: string) =>
    set((state) => ({
      expandedSections: state.expandedSections.includes(section)
        ? state.expandedSections.filter((s) => s !== section)
        : [...state.expandedSections, section],
    })),

  setActiveMenu: (menu: string) => set({ activeMenu: menu }),
}));
