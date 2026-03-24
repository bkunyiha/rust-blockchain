import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from 'react-hot-toast';
import { ApiConfigProvider } from './contexts/ApiConfigContext';
import { Layout } from './components/Layout/Layout';
import { Home } from './pages/Home';

// Blockchain routes
import { BlockchainInfo } from './components/Blockchain/BlockchainInfo';
import { LatestBlocks } from './components/Blockchain/LatestBlocks';
import { AllBlocks } from './components/Blockchain/AllBlocks';
import { BlockByHash } from './components/Blockchain/BlockByHash';

// Wallet routes
import { CreateWallet } from './components/Wallet/CreateWallet';
import { WalletInfo } from './components/Wallet/WalletInfo';
import { Balance } from './components/Wallet/Balance';
import { SendTransaction } from './components/Wallet/SendTransaction';
import { TransactionHistory } from './components/Wallet/TransactionHistory';
import { AllAddresses } from './components/Wallet/AllAddresses';

// Transaction routes
import { Mempool } from './components/Transactions/Mempool';
import { MempoolTx } from './components/Transactions/MempoolTx';
import { AllTransactions } from './components/Transactions/AllTransactions';
import { AddressTransactions } from './components/Transactions/AddressTransactions';

// Mining routes
import { MiningInfo } from './components/Mining/MiningInfo';
import { GenerateBlocks } from './components/Mining/GenerateBlocks';

// Health routes
import { HealthCheck } from './components/Health/HealthCheck';
import { Liveness } from './components/Health/Liveness';
import { Readiness } from './components/Health/Readiness';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ApiConfigProvider>
        <BrowserRouter>
          <Layout>
            <Routes>
              <Route path="/" element={<Home />} />
              
              {/* Blockchain routes */}
              <Route path="/blockchain/info" element={<BlockchainInfo />} />
              <Route path="/blockchain/latest" element={<LatestBlocks />} />
              <Route path="/blockchain/all" element={<AllBlocks />} />
              <Route path="/blockchain/hash" element={<BlockByHash />} />
              
              {/* Wallet routes */}
              <Route path="/wallet/create" element={<CreateWallet />} />
              <Route path="/wallet/info" element={<WalletInfo />} />
              <Route path="/wallet/balance" element={<Balance />} />
              <Route path="/wallet/send" element={<SendTransaction />} />
              <Route path="/wallet/history" element={<TransactionHistory />} />
              <Route path="/wallet/addresses" element={<AllAddresses />} />
              
              {/* Transaction routes */}
              <Route path="/transactions/mempool" element={<Mempool />} />
              <Route path="/transactions/mempool-tx" element={<MempoolTx />} />
              <Route path="/transactions/all" element={<AllTransactions />} />
              <Route path="/transactions/address" element={<AddressTransactions />} />
              
              {/* Mining routes */}
              <Route path="/mining/info" element={<MiningInfo />} />
              <Route path="/mining/generate" element={<GenerateBlocks />} />
              
              {/* Health routes */}
              <Route path="/health/check" element={<HealthCheck />} />
              <Route path="/health/liveness" element={<Liveness />} />
              <Route path="/health/readiness" element={<Readiness />} />
            </Routes>
          </Layout>
        </BrowserRouter>
        <Toaster
          position="top-right"
          toastOptions={{
            duration: 4000,
            style: {
              background: '#1f2937',
              color: '#fff',
              border: '1px solid #374151',
            },
            success: {
              iconTheme: {
                primary: '#f7931a',
                secondary: '#fff',
              },
            },
            error: {
              iconTheme: {
                primary: '#ef4444',
                secondary: '#fff',
              },
            },
          }}
        />
      </ApiConfigProvider>
    </QueryClientProvider>
  );
}

export default App;

