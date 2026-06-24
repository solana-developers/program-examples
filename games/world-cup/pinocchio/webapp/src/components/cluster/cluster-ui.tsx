import type { ReactNode } from 'react';
import type { Address } from '@solana/kit';
import { Button } from '@solana/design-system';
import { AppAlert } from '@/components/app-alert';
import { useCluster } from '@solana/connector/react';
import { useClusterVersion } from './use-cluster-version';

const SURFPOOL_STUDIO = 'http://127.0.0.1:18488';
const SOLANA_EXPLORER = 'https://explorer.solana.com';

function getExplorerUrl(link: Record<string, unknown>, clusterId: string): string {
    if (clusterId === 'solana:localnet') {
        if ('transaction' in link && link.transaction) return `${SURFPOOL_STUDIO}/transaction/${link.transaction}`;
        if ('address' in link && link.address) return `${SURFPOOL_STUDIO}/account/${link.address}`;
        if ('block' in link && link.block) return `${SURFPOOL_STUDIO}/block/${link.block}`;
        return SURFPOOL_STUDIO;
    }

    const clusterParam =
        clusterId === 'solana:mainnet' ? '' : `?cluster=${clusterId === 'solana:devnet' ? 'devnet' : 'testnet'}`;
    if ('transaction' in link && link.transaction) return `${SOLANA_EXPLORER}/tx/${link.transaction}${clusterParam}`;
    if ('address' in link && link.address) return `${SOLANA_EXPLORER}/address/${link.address}${clusterParam}`;
    if ('block' in link && link.block) return `${SOLANA_EXPLORER}/block/${link.block}${clusterParam}`;
    return `${SOLANA_EXPLORER}${clusterParam}`;
}

export function ExplorerLink({
    className,
    label = '',
    ...link
}: {
    cluster?: 'mainnet' | 'mainnet-beta' | 'devnet' | 'testnet' | 'localnet';
    address?: Address | string;
    transaction?: string;
    block?: bigint | number;
    className?: string;
    label: string;
}) {
    const { cluster } = useCluster();
    return (
        <a
            href={getExplorerUrl(link, cluster?.id ?? 'solana:localnet')}
            target="_blank"
            rel="noopener noreferrer"
            className={className ? className : `link font-mono`}
        >
            {label}
        </a>
    );
}

export function ClusterChecker({ children }: { children: ReactNode }) {
    const { cluster } = useCluster();
    const query = useClusterVersion();

    if (query.isLoading) {
        return null;
    }

    if (query.isError || !query.data) {
        return (
            <AppAlert
                action={
                    <Button size="sm" variant="secondary" onClick={() => query.refetch()}>
                        Refresh
                    </Button>
                }
                className="mb-4"
            >
                Error connecting to cluster <span className="font-bold">{cluster?.label ?? 'Unknown'}</span>.
            </AppAlert>
        );
    }
    return children;
}
