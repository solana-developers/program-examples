import type { ReactNode } from 'react';
import { useMemo } from 'react';
import {
    AppProvider,
    getDefaultConfig,
    useConnectWallet,
    useDisconnectWallet,
    useWallet,
    useWalletConnectors,
    useWalletInfo,
    type SolanaCluster,
    type SolanaClusterId,
} from '@solana/connector/react';
import { Button } from '@solana/design-system';
import { ChevronDown, LogOut, Wallet } from 'lucide-react';
import { toast } from 'sonner';

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { readCustomRpc } from '@/lib/custom-rpc';
import { ellipsify } from '@/lib/utils';

function defaultClusterId(): SolanaClusterId {
    const stored = localStorage.getItem('setup-cluster');
    const configured = import.meta.env.VITE_DEFAULT_CLUSTER;
    const id = stored || configured || 'solana:devnet';
    return id === 'solana:devnet' || id === 'solana:testnet' || id === 'solana:localnet' || id === 'solana:mainnet'
        ? (id as SolanaClusterId)
        : 'solana:devnet';
}

function networkFromClusterId(clusterId: SolanaClusterId): 'devnet' | 'localnet' | 'mainnet' | 'testnet' {
    if (clusterId === 'solana:devnet') return 'devnet';
    if (clusterId === 'solana:testnet') return 'testnet';
    if (clusterId === 'solana:mainnet') return 'mainnet';
    return 'localnet';
}

function buildClusters(): SolanaCluster[] {
    const clusters: SolanaCluster[] = [
        ...(import.meta.env.DEV ? [{ id: 'solana:localnet' as const, label: 'Localnet', url: '/rpc' }] : []),
        { id: 'solana:devnet', label: 'Devnet', url: 'https://api.devnet.solana.com' },
        { id: 'solana:testnet', label: 'Testnet', url: 'https://api.testnet.solana.com' },
        {
            id: 'solana:mainnet',
            label: 'Mainnet',
            url: import.meta.env.VITE_MAINNET_RPC_URL ?? 'https://api.mainnet-beta.solana.com',
        },
    ];

    const custom = readCustomRpc();
    if (custom) {
        const target = clusters.find(c => c.id === `solana:${custom.network}`);
        if (target) {
            target.url = custom.url;
            target.label = `${target.label} (${custom.label})`;
        }
    }

    return clusters;
}

export function WalletButton() {
    const { account, isConnected, isConnecting } = useWallet();
    const connectors = useWalletConnectors();
    const { connect, isConnecting: connectPending } = useConnectWallet();
    const { disconnect, isDisconnecting } = useDisconnectWallet();
    const walletInfo = useWalletInfo();

    const pending = isConnecting || connectPending || isDisconnecting;

    async function handleConnect(connectorId: (typeof connectors)[number]['id']) {
        try {
            await connect(connectorId);
        } catch (error) {
            toast.error(error instanceof Error ? error.message : 'Wallet connection failed');
        }
    }

    async function handleDisconnect() {
        try {
            await disconnect();
        } catch (error) {
            toast.error(error instanceof Error ? error.message : 'Wallet disconnect failed');
        }
    }

    if (isConnected && account) {
        return (
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button
                        disabled={pending}
                        iconLeft={<Wallet />}
                        iconRight={<ChevronDown className="opacity-60" />}
                        size="sm"
                        variant="secondary"
                    >
                        {ellipsify(account, 4)}
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" className="w-64">
                    <DropdownMenuLabel className="space-y-1">
                        <div className="text-sm">{walletInfo.name ?? 'Connected wallet'}</div>
                        <div className="font-mono text-xs text-muted-foreground">{account}</div>
                    </DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem
                        className="text-destructive focus:text-destructive"
                        disabled={pending}
                        onClick={() => void handleDisconnect()}
                    >
                        <LogOut className="h-4 w-4" />
                        Disconnect
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        );
    }

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                <Button
                    iconLeft={<Wallet />}
                    iconRight={<ChevronDown className="opacity-60" />}
                    loading={pending}
                    size="sm"
                    variant="secondary"
                >
                    Connect Wallet
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-64">
                <DropdownMenuLabel>Connect wallet</DropdownMenuLabel>
                <DropdownMenuSeparator />
                {connectors.length === 0 && (
                    <DropdownMenuItem disabled>No Wallet Standard wallets detected</DropdownMenuItem>
                )}
                {connectors.map(walletConnector => (
                    <DropdownMenuItem
                        disabled={pending || !walletConnector.ready}
                        key={walletConnector.id}
                        onClick={() => void handleConnect(walletConnector.id)}
                    >
                        {walletConnector.icon && (
                            <img src={walletConnector.icon} alt="" className="h-4 w-4 rounded-sm" />
                        )}
                        <span>{walletConnector.name}</span>
                        {!walletConnector.ready && (
                            <span className="ml-auto text-xs text-muted-foreground">Not ready</span>
                        )}
                    </DropdownMenuItem>
                ))}
            </DropdownMenuContent>
        </DropdownMenu>
    );
}

export function SolanaProvider({ children }: { children: ReactNode }) {
    const connectorConfig = useMemo(() => {
        const initialCluster = defaultClusterId();
        return getDefaultConfig({
            appName: 'World Cup',
            autoConnect: true,
            clusters: buildClusters(),
            enableMobile: true,
            network: networkFromClusterId(initialCluster),
            persistClusterSelection: false,
        });
    }, []);

    return <AppProvider connectorConfig={connectorConfig}>{children}</AppProvider>;
}
