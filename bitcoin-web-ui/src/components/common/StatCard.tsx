interface StatCardProps {
  label: string;
  value: string | number;
  icon?: React.ReactNode;
}

export function StatCard({ label, value, icon }: StatCardProps) {
  return (
    <div className="bg-gray-800 rounded-lg border border-gray-700 p-6 hover:border-bitcoin-orange/50 transition-colors">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-gray-400 mb-1">{label}</p>
          <p className="text-2xl font-bold text-white">{value}</p>
        </div>
        {icon && <div className="text-bitcoin-orange">{icon}</div>}
      </div>
    </div>
  );
}

