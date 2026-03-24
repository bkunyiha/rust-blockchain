import { Link, useLocation } from 'react-router-dom';
import { Transition } from '@headlessui/react';
import { Fragment, useState, useEffect } from 'react';
import clsx from 'clsx';

interface NavItem {
  name: string;
  path: string;
  subItems?: { name: string; path: string }[];
}

const navItems: NavItem[] = [
  { name: 'Dashboard', path: '/' },
  {
    name: 'Blockchain',
    path: '/blockchain',
    subItems: [
      { name: 'Info', path: '/blockchain/info' },
      { name: 'Latest Blocks', path: '/blockchain/latest' },
      { name: 'All Blocks', path: '/blockchain/all' },
      { name: 'Block by Hash', path: '/blockchain/hash' },
    ],
  },
  {
    name: 'Wallet',
    path: '/wallet',
    subItems: [
      { name: 'Create Wallet', path: '/wallet/create' },
      { name: 'Wallet Info', path: '/wallet/info' },
      { name: 'Balance', path: '/wallet/balance' },
      { name: 'Send Bitcoin', path: '/wallet/send' },
      { name: 'Transaction History', path: '/wallet/history' },
      { name: 'All Addresses', path: '/wallet/addresses' },
    ],
  },
  {
    name: 'Transactions',
    path: '/transactions',
    subItems: [
      { name: 'Mempool', path: '/transactions/mempool' },
      { name: 'Mempool Transaction', path: '/transactions/mempool-tx' },
      { name: 'All Transactions', path: '/transactions/all' },
      { name: 'Address Transactions', path: '/transactions/address' },
    ],
  },
  {
    name: 'Mining',
    path: '/mining',
    subItems: [
      { name: 'Mining Info', path: '/mining/info' },
      { name: 'Generate Blocks', path: '/mining/generate' },
    ],
  },
  {
    name: 'Health',
    path: '/health',
    subItems: [
      { name: 'Health Check', path: '/health/check' },
      { name: 'Liveness', path: '/health/liveness' },
      { name: 'Readiness', path: '/health/readiness' },
    ],
  },
];

export function Sidebar() {
  const location = useLocation();
  
  // Determine which menu should be open based on current location
  const getOpenMenuPath = (): string | null => {
    for (const item of navItems) {
      if (item.subItems) {
        // Check if current path matches this menu or any of its sub-items
        if (location.pathname === item.path || 
            item.subItems.some(sub => location.pathname === sub.path)) {
          return item.path;
        }
      }
    }
    return null;
  };

  const [openMenuPath, setOpenMenuPath] = useState<string | null>(getOpenMenuPath());

  // Update open menu when location changes
  useEffect(() => {
    const newOpenMenuPath = getOpenMenuPath();
    setOpenMenuPath(newOpenMenuPath);
  }, [location.pathname]);

  return (
    <aside className="w-64 bg-gray-800 border-r border-gray-700 min-h-screen">
      <nav className="p-4 space-y-2">
        {navItems.map((item) => {
          const isActive = location.pathname === item.path || 
            (item.subItems && item.subItems.some(sub => location.pathname === sub.path));
          
          if (item.subItems) {
            const isOpen = openMenuPath === item.path;
            
            return (
              <div key={item.path} className="relative">
                <button
                  onClick={() => {
                    // Toggle menu, but keep it open if navigating within same menu
                    if (isOpen && openMenuPath === item.path) {
                      // If clicking the same menu button while it's open, keep it open
                      // (only close when navigating to different main menu)
                      return;
                    }
                    setOpenMenuPath(isOpen ? null : item.path);
                  }}
                  className={clsx(
                    'w-full text-left px-4 py-2 rounded-lg transition-colors',
                    isActive
                      ? 'bg-bitcoin-orange/20 text-bitcoin-orange border border-bitcoin-orange/30'
                      : 'text-gray-300 hover:bg-gray-700'
                  )}
                >
                  {item.name}
                </button>
                <Transition
                  as={Fragment}
                  show={isOpen}
                  enter="transition ease-out duration-100"
                  enterFrom="transform opacity-0 scale-95"
                  enterTo="transform opacity-100 scale-100"
                  leave="transition ease-in duration-75"
                  leaveFrom="transform opacity-100 scale-100"
                  leaveTo="transform opacity-0 scale-95"
                >
                  <div className="mt-1 ml-4 space-y-1">
                    {item.subItems.map((subItem) => {
                      const isSubItemActive = location.pathname === subItem.path;
                      return (
                        <Link
                          key={subItem.path}
                          to={subItem.path}
                          onClick={() => {
                            // Keep menu open when clicking sub-items
                            setOpenMenuPath(item.path);
                          }}
                          className={clsx(
                            'block px-4 py-2 rounded-lg text-sm transition-colors',
                            isSubItemActive
                              ? 'bg-bitcoin-orange/20 text-bitcoin-orange'
                              : 'text-gray-400 hover:bg-gray-700 hover:text-gray-200'
                          )}
                        >
                          {subItem.name}
                        </Link>
                      );
                    })}
                  </div>
                </Transition>
              </div>
            );
          }

          return (
            <Link
              key={item.path}
              to={item.path}
              className={clsx(
                'block px-4 py-2 rounded-lg transition-colors',
                isActive
                  ? 'bg-bitcoin-orange/20 text-bitcoin-orange border border-bitcoin-orange/30'
                  : 'text-gray-300 hover:bg-gray-700'
              )}
            >
              {item.name}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}

