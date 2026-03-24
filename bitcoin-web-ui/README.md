# Bitcoin Blockchain Web UI

A modern, professional React-based web interface for managing the Bitcoin blockchain node. Built with React, TypeScript, Vite, Tailwind CSS, and React Query.

## Features

- **Dashboard**: Real-time blockchain statistics with auto-refresh
- **Blockchain Management**: View blockchain info, latest blocks, all blocks, and search by hash
- **Wallet Operations**: Create wallets, view info, check balances, send transactions, view history
- **Transaction Management**: Browse mempool, search transactions, view address transactions
- **Mining**: View mining info and generate blocks
- **Health Monitoring**: Health checks, liveness, and readiness endpoints
- **Modern UI**: Dark theme with Bitcoin-inspired colors, responsive design
- **API Configuration**: Easy API key and base URL configuration

## Prerequisites

- Node.js 18+ and npm/yarn/pnpm
- Rust blockchain server running on `http://127.0.0.1:8080` (or configured base URL)

## Installation

1. Install dependencies:
```bash
npm install
```

2. Configure API settings:
   - The UI will prompt you to configure the API base URL and API key on first use
   - Default base URL: `http://127.0.0.1:8080`
   - Default admin API key: `admin-secret` (set via `BITCOIN_API_ADMIN_KEY` env var on server)

## Development

Run the development server:

```bash
npm run dev
```

The app will be available at `http://localhost:3000` with hot module replacement.

### API Configuration (Development Only)

When running in development mode, you need to configure the API key to access the backend:

1. **Open the app** at `http://localhost:3000`
2. **Click "Configure API"** button in the top-right navbar
3. **Enter the API key**: 
   - Default admin key: `admin-secret`
   - Or set via environment variable: `BITCOIN_API_ADMIN_KEY`
4. **Base URL** should be: `http://127.0.0.1:8080` (default)

The API key is saved in browser localStorage and will be used for all API requests.

**Note**: The web routes (`/` and `/assets/*`) are public and don't require authentication. Only the API routes (`/api/admin/*`) require the `X-API-Key` header. If you see "Unauthorized" errors in the browser console, it means the API key hasn't been configured yet.

**Default API Keys** (from Rust backend):
- **Admin key**: `admin-secret` (or `BITCOIN_API_ADMIN_KEY` env var)
- **Wallet key**: `wallet-secret` (or `BITCOIN_API_WALLET_KEY` env var)

## Building for Production

Build the React app for production:

```bash
npm run build
```

This creates an optimized build in the `dist/` directory.

## Integration with Rust Server

The Rust server is configured to serve the React app automatically:

1. Build the React app: `npm run build`
2. Start the Rust blockchain server
3. Access the web UI at `http://localhost:8080`

The server will:
- Serve the React app from `bitcoin-web-ui/dist/`
- Handle client-side routing (all routes fallback to `index.html`)
- Serve API endpoints at `/api/admin/*` and `/api/v1/*`

## Project Structure

```
bitcoin-web-ui/
├── src/
│   ├── components/      # React components
│   │   ├── Layout/     # Navigation and layout
│   │   ├── Dashboard/  # Dashboard page
│   │   ├── Blockchain/ # Blockchain management
│   │   ├── Wallet/     # Wallet operations
│   │   ├── Transactions/ # Transaction management
│   │   ├── Mining/     # Mining operations
│   │   ├── Health/     # Health monitoring
│   │   └── common/     # Shared components
│   ├── contexts/       # React contexts (API config)
│   ├── hooks/          # React Query hooks
│   ├── services/       # API client
│   ├── types/          # TypeScript types
│   └── App.tsx         # Main app component
├── dist/               # Production build (generated)
└── package.json
```

## API Endpoints

The UI uses the following API endpoints (all require `X-API-Key` header):

### Blockchain
- `GET /api/admin/blockchain` - Get blockchain info
- `GET /api/admin/blockchain/blocks/latest` - Get latest blocks
- `GET /api/admin/blockchain/blocks` - Get all blocks
- `GET /api/admin/blockchain/blocks/{hash}` - Get block by hash

### Wallet
- `POST /api/admin/wallet` - Create wallet
- `GET /api/admin/wallet/addresses` - Get all addresses
- `GET /api/admin/wallet/{address}` - Get wallet info
- `GET /api/admin/wallet/{address}/balance` - Get balance
- `POST /api/admin/transactions` - Send transaction
- `GET /api/admin/transactions/address/{address}` - Get address transactions

### Transactions
- `GET /api/admin/transactions/mempool` - Get mempool
- `GET /api/admin/transactions/mempool/{txid}` - Get mempool transaction
- `GET /api/admin/transactions` - Get all transactions

### Mining
- `GET /api/admin/mining/info` - Get mining info
- `POST /api/admin/mining/generatetoaddress` - Generate blocks

### Health
- `GET /api/admin/health` - Health check
- `GET /api/admin/health/live` - Liveness check
- `GET /api/admin/health/ready` - Readiness check

## Technologies Used

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **React Router** - Client-side routing
- **React Query (TanStack Query)** - Server state management
- **Tailwind CSS** - Styling
- **Headless UI** - Accessible UI components
- **Axios** - HTTP client
- **React Hot Toast** - Toast notifications

## License

Same as the main blockchain project.

