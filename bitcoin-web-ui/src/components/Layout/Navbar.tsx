import { useApiConfig } from '../../contexts/ApiConfigContext';
import { Menu, Transition } from '@headlessui/react';
import { Fragment } from 'react';

export function Navbar() {
  const { baseURL, apiKey, setBaseURL, setApiKey } = useApiConfig();

  return (
    <nav className="bg-gray-800 border-b border-gray-700">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <h1 className="text-xl font-bold text-bitcoin-orange">Bitcoin Blockchain Admin</h1>
          </div>
          
          <div className="flex items-center gap-4">
            <Menu as="div" className="relative">
              <Menu.Button className="btn-secondary text-sm">
                Configure API
              </Menu.Button>
              <Transition
                as={Fragment}
                enter="transition ease-out duration-100"
                enterFrom="transform opacity-0 scale-95"
                enterTo="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leaveFrom="transform opacity-100 scale-100"
                leaveTo="transform opacity-0 scale-95"
              >
                <Menu.Items className="absolute right-0 mt-2 w-80 bg-gray-800 border border-gray-700 rounded-lg shadow-lg p-4 z-50">
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm text-gray-300 mb-1">Base URL</label>
                      <input
                        type="text"
                        value={baseURL}
                        onChange={(e) => setBaseURL(e.target.value)}
                        className="input-field"
                        placeholder="http://127.0.0.1:8080"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-300 mb-1">API Key</label>
                      <input
                        type="password"
                        value={apiKey}
                        onChange={(e) => setApiKey(e.target.value)}
                        className="input-field"
                        placeholder="admin-secret"
                      />
                    </div>
                    {apiKey && (
                      <div className="text-xs text-green-400">
                        ✓ API configured
                      </div>
                    )}
                  </div>
                </Menu.Items>
              </Transition>
            </Menu>
          </div>
        </div>
      </div>
    </nav>
  );
}

