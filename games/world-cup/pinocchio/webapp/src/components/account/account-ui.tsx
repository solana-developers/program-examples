import { useCluster, useWallet } from '@solana/connector/react';
import { address } from '@solana/kit';
import type { Address, Lamports } from '@solana/kit';
import { Button, CopyButton, TextInput } from '@solana/design-system';

const lamportsToSol = (lamports: bigint | number): number => Number(lamports) / 1e9;
import { AppAlert } from '@/components/app-alert';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useMemo, useState } from 'react';
import { RefreshCw, Wallet, DollarSign } from 'lucide-react';
import { toast } from 'sonner';
import type { TokenAccountEntry } from '@/lib/types';
import { useGetBalanceQuery, useGetTokenAccountsQuery, useAirdropSol, useAirdropUsdc } from './account-data-access';
import { useSelectedToken } from '@/hooks/use-selected-token';
import { TokenPicker } from '@/components/token/token-picker';
import { cn } from '@/lib/utils';
import { useProgramAddress } from '@/hooks/use-token-config';

export function AccountChecker() {
    const { account } = useWallet();
    if (!account) {
        return null;
    }
    return <AccountBalanceCheck address={address(account)} />;
}

export function AccountBalanceCheck({ address: addr }: { address: Address }) {
    const { cluster } = useCluster();
    const query = useGetBalanceQuery({ address: addr });

    if (query.isLoading) {
        return null;
    }
    if (query.isError || !query.data?.value) {
        if (cluster?.id !== 'solana:localnet' && cluster?.id !== 'solana:devnet') return null;
        return (
            <AppAlert>
                You are connected to <strong>{cluster?.label ?? 'this cluster'}</strong> but your account is not found
                on this cluster.
            </AppAlert>
        );
    }
    return null;
}

export function AccountBalance({ address: addr }: { address: Address }) {
    const query = useGetBalanceQuery({ address: addr });

    return (
        <h1 className="text-5xl font-bold cursor-pointer" onClick={() => query.refetch()}>
            {query.data?.value ? <BalanceSol balance={query.data?.value} /> : '...'} SOL
        </h1>
    );
}

export function WalletBalanceCards({ address: addr }: { address: Address }) {
    const solQuery = useGetBalanceQuery({ address: addr });
    const tokenQuery = useGetTokenAccountsQuery({ address: addr });
    const { selectedMint, selectedToken } = useSelectedToken();
    const symbol = selectedToken?.symbol ?? '';
    const progAddr = useProgramAddress();

    const tokenAccount = useMemo(() => {
        return (tokenQuery.data as TokenAccountEntry[] | undefined)?.find(entry => {
            return entry.account?.data?.parsed?.info?.mint === selectedMint;
        });
    }, [tokenQuery.data, selectedMint]);

    const tokenBalance = tokenAccount?.account?.data?.parsed?.info?.tokenAmount?.uiAmount ?? 0;

    const [spinning, setSpinning] = useState(false);
    const isFetching = solQuery.isFetching || tokenQuery.isFetching;
    const isRefreshing = isFetching || spinning;

    const handleRefresh = async () => {
        setSpinning(true);
        const minSpin = new Promise(r => setTimeout(r, 600));
        await Promise.all([solQuery.refetch(), tokenQuery.refetch(), minSpin]);
        setSpinning(false);
    };

    return (
        <div className="space-y-6">
            <div className="flex items-center justify-between">
                <div className="space-y-1">
                    <h2 className="text-[28px] font-bold tracking-tight text-foreground">Wallet Overview</h2>
                    <div className="flex items-center gap-1.5 text-xs text-sand-1000">
                        <span>Program:</span>
                        <span className="font-mono text-sand-1100">
                            {progAddr ? `${progAddr.slice(0, 8)}...${progAddr.slice(-4)}` : '...'}
                        </span>
                        <CopyButton value={progAddr ?? ''} />
                    </div>
                </div>
                <div className="flex items-center gap-2">
                    <TokenPicker />
                    <Button
                        variant="secondary"
                        size="sm"
                        iconOnly
                        iconLeft={<RefreshCw className={isRefreshing ? 'animate-spin' : ''} />}
                        aria-label="Refresh wallet balances"
                        onClick={handleRefresh}
                        disabled={isRefreshing}
                    />
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <Card className="relative overflow-hidden bg-card rounded-2xl border-all-dashed-medium border-0">
                    <CardHeader className="relative pb-2">
                        <CardTitle className="flex items-center justify-between">
                            <span className="text-sm font-medium text-sand-1100">Solana Balance</span>
                            <div className="p-2 bg-sand-100 rounded-lg">
                                <Wallet className="h-5 w-5 text-foreground" />
                            </div>
                        </CardTitle>
                    </CardHeader>
                    <CardContent className="relative pt-4">
                        <div className="space-y-1">
                            <div className="text-xl sm:text-3xl lg:text-[40px] leading-tight font-semibold tracking-tight text-foreground">
                                {solQuery.data?.value ? (
                                    <span>{Number(lamportsToSol(solQuery.data.value)).toFixed(4)}</span>
                                ) : (
                                    <span className="text-muted-foreground">...</span>
                                )}
                            </div>
                            <div className="text-sm font-medium text-sand-1000 tracking-wide">SOL</div>
                        </div>
                    </CardContent>
                </Card>

                <Card className="relative overflow-hidden bg-card rounded-2xl border-all-dashed-medium border-0">
                    <CardHeader className="relative pb-2">
                        <CardTitle className="flex items-center justify-between">
                            <div>
                                <span className="text-sm font-medium text-sand-1100">{symbol} Balance</span>
                                {selectedMint && (
                                    <p className="flex items-center gap-1 text-[10px] font-mono text-sand-900 mt-0.5">
                                        {selectedMint.slice(0, 8)}...{selectedMint.slice(-4)}
                                        <CopyButton value={selectedMint} />
                                    </p>
                                )}
                            </div>
                            <div className="p-2 bg-sand-100 rounded-lg">
                                <DollarSign className="h-5 w-5 text-foreground" />
                            </div>
                        </CardTitle>
                    </CardHeader>
                    <CardContent className="relative pt-4">
                        <div className="space-y-3">
                            {tokenQuery.isLoading ? (
                                <div className="text-lg sm:text-2xl lg:text-[36px] leading-tight font-semibold text-muted-foreground">
                                    ...
                                </div>
                            ) : (
                                <div className="min-w-0">
                                    <div className="text-lg sm:text-2xl lg:text-[36px] leading-tight font-semibold tracking-tight text-foreground">
                                        {tokenBalance.toLocaleString(undefined, {
                                            minimumFractionDigits: 2,
                                            maximumFractionDigits: 2,
                                        })}
                                    </div>
                                    <div className="text-sm font-medium text-sand-1000 tracking-wide">Wallet</div>
                                </div>
                            )}
                        </div>
                    </CardContent>
                </Card>
            </div>
        </div>
    );
}

export function SolFaucetCard() {
    const [amount, setAmount] = useState('1');
    const { cluster } = useCluster();
    const isDevnet = cluster?.id === 'solana:devnet';
    const airdrop = useAirdropSol();

    const handleAirdrop = async () => {
        const val = parseFloat(amount);
        if (!amount || !Number.isFinite(val) || val <= 0) {
            toast.error('Please enter a valid SOL amount');
            return;
        }
        if (isDevnet && val > 2) {
            toast.error('Devnet limits airdrops to 2 SOL per request');
            return;
        }
        await airdrop.mutateAsync(val);
    };

    return (
        <Card
            className={`relative overflow-hidden border bg-gradient-to-br transition-all duration-300 ${isDevnet ? 'border-sand-300 from-sand-100 via-sand-100 to-transparent opacity-60' : 'border-sand-300 from-sand-100 via-sand-50 to-transparent hover:border-sand-400'}`}
        >
            <CardHeader className="relative pb-2">
                <CardTitle className="flex items-center justify-between">
                    <span className="text-sm font-medium text-sand-1100">SOL Airdrop</span>
                    <Wallet className={`h-5 w-5 ${isDevnet ? 'text-sand-1000' : 'text-foreground'}`} />
                </CardTitle>
            </CardHeader>
            <CardContent className="relative space-y-4">
                {isDevnet ? (
                    <p className="text-sm text-sand-1000 py-4">
                        SOL airdrop is not available on devnet. Use{' '}
                        <a
                            href="https://faucet.solana.com"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-foreground underline hover:no-underline"
                        >
                            faucet.solana.com
                        </a>{' '}
                        instead.
                    </p>
                ) : (
                    <>
                        <TextInput
                            type="number"
                            placeholder="0"
                            value={amount}
                            onChange={e => setAmount(e.target.value)}
                            min="0.1"
                            step="0.1"
                            inputClassName="text-3xl font-bold"
                            size="xl"
                        />
                        <div className="flex flex-wrap gap-2">
                            {[1, 2, 5, 10].map(v => (
                                <Button
                                    key={v}
                                    variant="secondary"
                                    size="sm"
                                    radius="round"
                                    onClick={() => setAmount(String(v))}
                                >
                                    {v} SOL
                                </Button>
                            ))}
                        </div>
                        <Button
                            onClick={handleAirdrop}
                            disabled={airdrop.isPending}
                            loading={airdrop.isPending}
                            radius="round"
                            style={{ width: '100%' }}
                        >
                            Request Airdrop
                        </Button>
                    </>
                )}
            </CardContent>
        </Card>
    );
}

export function UsdcFaucetCard() {
    const [amount, setAmount] = useState('1000');
    const [recipient, setRecipient] = useState('');
    const { account } = useWallet();
    const { cluster } = useCluster();
    const isDevnet = cluster?.id === 'solana:devnet';
    const airdrop = useAirdropUsdc();

    const handleAirdrop = async () => {
        const val = parseFloat(amount);
        if (!amount || !Number.isFinite(val) || val <= 0) {
            toast.error('Please enter a valid USDC amount');
            return;
        }
        await airdrop.mutateAsync({
            amount: val,
            recipient: isDevnet && recipient ? recipient : undefined,
        });
    };

    const showCircleLink = isDevnet && !import.meta.env.DEV;

    return (
        <Card
            className={cn(
                'relative overflow-hidden border-0 border-all-dashed-medium bg-card transition-all duration-300',
                showCircleLink && 'opacity-60',
            )}
        >
            <CardHeader className="relative pb-2">
                <CardTitle className="flex items-center justify-between">
                    <span className="text-sm font-medium text-sand-1100">USDC {isDevnet ? 'Faucet' : 'Airdrop'}</span>
                    <DollarSign className="h-5 w-5 text-foreground" />
                </CardTitle>
            </CardHeader>
            <CardContent className="relative space-y-4">
                {showCircleLink ? (
                    <p className="text-sm text-sand-1000 py-4">
                        USDC airdrop is not available on devnet. Use{' '}
                        <a
                            href="https://faucet.circle.com/"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-foreground underline hover:no-underline"
                        >
                            faucet.circle.com
                        </a>{' '}
                        instead.
                    </p>
                ) : (
                    <>
                        <TextInput
                            type="number"
                            placeholder="0"
                            value={amount}
                            onChange={e => setAmount(e.target.value)}
                            min="1"
                            step="100"
                            inputClassName="text-3xl font-bold"
                            size="xl"
                        />
                        {isDevnet && (
                            <TextInput
                                type="text"
                                placeholder={account ?? 'Recipient address (leave empty for self)'}
                                value={recipient}
                                onChange={e => setRecipient(e.target.value)}
                                inputClassName="font-mono text-xs"
                                size="lg"
                            />
                        )}
                        <div className="flex flex-wrap gap-2">
                            {[100, 1000, 5000, 10000].map(v => (
                                <Button
                                    key={v}
                                    variant="secondary"
                                    size="sm"
                                    radius="round"
                                    onClick={() => setAmount(String(v))}
                                >
                                    {v.toLocaleString()}
                                </Button>
                            ))}
                        </div>
                        {isDevnet && <p className="text-xs text-sand-1000">Mint authority wallet required</p>}
                        <Button
                            onClick={handleAirdrop}
                            disabled={airdrop.isPending}
                            loading={airdrop.isPending}
                            radius="round"
                            style={{ width: '100%' }}
                        >
                            {isDevnet ? 'Mint USDC' : 'Request Airdrop'}
                        </Button>
                    </>
                )}
            </CardContent>
        </Card>
    );
}

function BalanceSol({ balance }: { balance: Lamports }) {
    return <span>{lamportsToSol(balance)}</span>;
}
