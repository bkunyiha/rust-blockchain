import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { updateApiClient } from '../services/api';

interface ApiConfigContextType {
  baseURL: string;
  apiKey: string;
  setBaseURL: (url: string) => void;
  setApiKey: (key: string) => void;
  isConfigured: boolean;
}

const ApiConfigContext = createContext<ApiConfigContextType | undefined>(undefined);

export function ApiConfigProvider({ children }: { children: ReactNode }) {
  const [baseURL, setBaseURLState] = useState(() => {
    return localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080';
  });
  const [apiKey, setApiKeyState] = useState(() => {
    return localStorage.getItem('api_key') || '';
  });

  useEffect(() => {
    updateApiClient(baseURL, apiKey || undefined);
  }, [baseURL, apiKey]);

  const setBaseURL = (url: string) => {
    setBaseURLState(url);
    localStorage.setItem('api_base_url', url);
    updateApiClient(url, apiKey || undefined);
  };

  const setApiKey = (key: string) => {
    setApiKeyState(key);
    if (key) {
      localStorage.setItem('api_key', key);
    } else {
      localStorage.removeItem('api_key');
    }
    updateApiClient(baseURL, key || undefined);
  };

  return (
    <ApiConfigContext.Provider
      value={{
        baseURL,
        apiKey,
        setBaseURL,
        setApiKey,
        isConfigured: !!apiKey,
      }}
    >
      {children}
    </ApiConfigContext.Provider>
  );
}

export function useApiConfig() {
  const context = useContext(ApiConfigContext);
  if (context === undefined) {
    throw new Error('useApiConfig must be used within an ApiConfigProvider');
  }
  return context;
}

