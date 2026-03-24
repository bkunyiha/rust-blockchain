import { NavLink } from 'react-router-dom';
import {
  ChevronDown,
  Blocks,
  Wallet as WalletIcon,
  ArrowLeftRight,
  Pickaxe,
  Activity,
} from 'lucide-react';
import { useAppStore } from '../store/useAppStore';
import { cn } from '../lib/utils';

interface SidebarSection {
  id: string;
  label: string;
  icon: React.ReactNode;
  items: Array<{
    label: string;
    path: string;
  }>;
}

const sections: SidebarSection[] = [
  {
    id: 'blockchain',
    label: 'Blockchain',
    icon: <Blocks className="h-5 w-5" />,
    items: [
      { label: 'Info', path: '/blockchain/info' },
      { label: 'Latest Blocks', path: '/blockchain/latest' },
      { label: 'All Blocks', path: '/blockchain/all' },
      { label: 'Block by Hash', path: '/blockchain/by-hash' },
    ],
  },
  {
    id: 'wallet',
    label: 'Wallet',
    icon: <WalletIcon className="h-5 w-5" />,
    items: [
      { label: 'Create', path: '/wallet/create' },
      { label: 'Info', path: '/wallet/info' },
      { label: 'Balance', path: '/wallet/balance' },
      { label: 'Send', path: '/wallet/send' },
      { label: 'History', path: '/wallet/history' },
      { label: 'Addresses', path: '/wallet/addresses' },
    ],
  },
  {
    id: 'transactions',
    label: 'Transactions',
    icon: <ArrowLeftRight className="h-5 w-5" />,
    items: [
      { label: 'Mempool', path: '/transactions/mempool' },
      { label: 'Mempool TX', path: '/transactions/mempool-tx' },
      { label: 'All', path: '/transactions/all' },
      { label: 'By Address', path: '/transactions/by-address' },
    ],
  },
  {
    id: 'mining',
    label: 'Mining',
    icon: <Pickaxe className="h-5 w-5" />,
    items: [
      { label: 'Info', path: '/mining/info' },
      { label: 'Generate', path: '/mining/generate' },
    ],
  },
  {
    id: 'health',
    label: 'Health',
    icon: <Activity className="h-5 w-5" />,
    items: [{ label: 'Status', path: '/health' }],
  },
];

function Sidebar() {
  const { expandedSections, toggleSection } = useAppStore();

  return (
    <aside className="w-64 border-r border-slate-700 bg-slate-900">
      <nav className="p-4 space-y-2">
        {sections.map((section) => (
          <div key={section.id}>
            <button
              onClick={() => toggleSection(section.id)}
              className={cn(
                'w-full flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
                'hover:bg-slate-800'
              )}
            >
              <div className="text-slate-400">{section.icon}</div>
              <span className="flex-1 text-left">{section.label}</span>
              <ChevronDown
                className={cn(
                  'h-4 w-4 transition-transform',
                  expandedSections.includes(section.id) && 'rotate-180'
                )}
              />
            </button>

            {expandedSections.includes(section.id) && (
              <div className="ml-2 mt-1 space-y-1 border-l border-slate-700 pl-3">
                {section.items.map((item) => (
                  <NavLink
                    key={item.path}
                    to={item.path}
                    className={({ isActive }) =>
                      cn(
                        'block rounded-lg px-3 py-1.5 text-sm transition-colors',
                        isActive
                          ? 'bg-yellow-500 text-slate-900 font-medium'
                          : 'text-slate-300 hover:bg-slate-800 hover:text-slate-100'
                      )
                    }
                  >
                    {item.label}
                  </NavLink>
                ))}
              </div>
            )}
          </div>
        ))}
      </nav>
    </aside>
  );
}

export default Sidebar;
