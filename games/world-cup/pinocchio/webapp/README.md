# World Cup Webapp

A generic Solana app shell connected to the World Cup on-chain program. Provides wallet connection, cluster switching, program deploy/status tooling, token management, and a dev faucet.

## Features

- **Wallet Connection** - Solana wallet integration (tested with Phantom) with real-time SOL and token balance display
- **Cluster Switching** - Switch between localnet, devnet, testnet, mainnet, or a custom RPC endpoint
- **Program Deploy & Status** - Deploy and inspect the on-chain program (dev only)
- **Token Management** - Add and select tokens, view balances
- **Dev Faucet** - Request SOL/USDC airdrops for local testing (hidden on mainnet)
- **Theme Support** - Dark/light mode toggle

## Scripts

| Script            | Description                                           |
| ----------------- | ----------------------------------------------------- |
| `npm run dev`     | Start the Vite dev server with hot module replacement |
| `npm run build`   | Type-check with TypeScript and build for production   |
| `npm run preview` | Preview the production build locally                  |

From the project root, `just webapp-run` builds the program and clients, starts a local validator + API, and launches the webapp.

## Tech Stack

React 19, TypeScript, Vite, Tailwind CSS, Radix UI, jotai (state), TanStack Query (data fetching), Solana Kit, ConnectorKit.
