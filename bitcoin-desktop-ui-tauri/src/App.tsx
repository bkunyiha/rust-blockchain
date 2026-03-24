import { Routes, Route, Navigate } from 'react-router-dom';
import { ShortcutProvider } from './components/ShortcutProvider';
import { ToastProvider } from './components/Toast';
import AppLayout from './components/AppLayout';
import ThemeProvider from './components/ThemeProvider';

// Blockchain pages
import BlockchainInfoPage from './pages/blockchain/BlockchainInfoPage';
import LatestBlocksPage from './pages/blockchain/LatestBlocksPage';
import AllBlocksPage from './pages/blockchain/AllBlocksPage';
import BlockByHashPage from './pages/blockchain/BlockByHashPage';

// Wallet pages
import CreateWalletPage from './pages/wallet/CreateWalletPage';
import WalletInfoPage from './pages/wallet/WalletInfoPage';
import BalancePage from './pages/wallet/BalancePage';
import SendBitcoinPage from './pages/wallet/SendBitcoinPage';
import TxHistoryPage from './pages/wallet/TxHistoryPage';
import AddressListPage from './pages/wallet/AddressListPage';

// Transaction pages
import MempoolPage from './pages/transactions/MempoolPage';
import MempoolTxPage from './pages/transactions/MempoolTxPage';
import AllTransactionsPage from './pages/transactions/AllTransactionsPage';
import AddressTxPage from './pages/transactions/AddressTxPage';

// Mining pages
import MiningInfoPage from './pages/mining/MiningInfoPage';
import GenerateBlocksPage from './pages/mining/GenerateBlocksPage';

// Health page
import HealthPage from './pages/health/HealthPage';

function App() {
  return (
    <ThemeProvider>
      <ToastProvider>
        <ShortcutProvider>
          <Routes>
            <Route path="/" element={<AppLayout />}>
              {/* Blockchain */}
              <Route path="blockchain/info" element={<BlockchainInfoPage />} />
              <Route path="blockchain/latest" element={<LatestBlocksPage />} />
              <Route path="blockchain/all" element={<AllBlocksPage />} />
              <Route path="blockchain/by-hash" element={<BlockByHashPage />} />

              {/* Wallet */}
              <Route path="wallet/create" element={<CreateWalletPage />} />
              <Route path="wallet/info" element={<WalletInfoPage />} />
              <Route path="wallet/balance" element={<BalancePage />} />
              <Route path="wallet/send" element={<SendBitcoinPage />} />
              <Route path="wallet/history" element={<TxHistoryPage />} />
              <Route path="wallet/addresses" element={<AddressListPage />} />

              {/* Transactions */}
              <Route path="transactions/mempool" element={<MempoolPage />} />
              <Route path="transactions/mempool-tx" element={<MempoolTxPage />} />
              <Route path="transactions/all" element={<AllTransactionsPage />} />
              <Route path="transactions/by-address" element={<AddressTxPage />} />

              {/* Mining */}
              <Route path="mining/info" element={<MiningInfoPage />} />
              <Route path="mining/generate" element={<GenerateBlocksPage />} />

              {/* Health */}
              <Route path="health" element={<HealthPage />} />

              {/* Redirect root to blockchain info */}
              <Route index element={<Navigate to="/blockchain/info" replace />} />
            </Route>
          </Routes>
        </ShortcutProvider>
      </ToastProvider>
    </ThemeProvider>
  );
}

export default App;
