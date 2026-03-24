import { Routes, Route, Navigate } from "react-router-dom";
import AppLayout from "./components/AppLayout";
import CreateWalletPage from "./pages/wallet/CreateWalletPage";
import WalletListPage from "./pages/wallet/WalletListPage";
import WalletInfoPage from "./pages/wallet/WalletInfoPage";
import BalancePage from "./pages/wallet/BalancePage";
import SendPage from "./pages/wallet/SendPage";
import HistoryPage from "./pages/wallet/HistoryPage";
import SettingsPage from "./pages/settings/SettingsPage";

export default function App() {
  return (
    <Routes>
      <Route element={<AppLayout />}>
        <Route path="/" element={<Navigate to="/wallet/list" replace />} />
        <Route path="/wallet/create" element={<CreateWalletPage />} />
        <Route path="/wallet/list" element={<WalletListPage />} />
        <Route path="/wallet/info" element={<WalletInfoPage />} />
        <Route path="/balance" element={<BalancePage />} />
        <Route path="/send" element={<SendPage />} />
        <Route path="/history" element={<HistoryPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}
