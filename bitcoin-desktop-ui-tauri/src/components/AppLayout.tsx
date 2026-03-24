import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';
import TopBar from './TopBar';
import StatusBar from './StatusBar';

function AppLayout() {
  return (
    <div className="flex h-screen flex-col bg-slate-950 text-slate-100">
      <TopBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-auto">
          <div className="p-6">
            <Outlet />
          </div>
        </main>
      </div>
      <StatusBar />
    </div>
  );
}

export default AppLayout;
