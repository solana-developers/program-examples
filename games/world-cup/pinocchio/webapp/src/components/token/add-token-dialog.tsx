import { useState } from 'react';
import { Button, TextInput } from '@solana/design-system';
import { address, createSolanaRpc } from '@solana/kit';
import { useQueryClient } from '@tanstack/react-query';
import { Trash2 } from 'lucide-react';
import { toast } from 'sonner';

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';
import { useClusterConfig } from '@/hooks/use-cluster-config';
import { useSelectedToken } from '@/hooks/use-selected-token';
import { clusterIdToNetwork } from '@/lib/cluster';
import { addCustomToken, readCustomTokens, removeCustomToken } from '@/lib/custom-tokens';
import { ellipsify } from '@/lib/utils';

async function fetchMintDecimals(rpcUrl: string, mint: string): Promise<number> {
    const rpc = createSolanaRpc(rpcUrl);
    const res = await rpc.getAccountInfo(address(mint), { commitment: 'confirmed', encoding: 'jsonParsed' }).send();
    const data = res.value?.data as { parsed?: { info?: { decimals?: number }; type?: string } } | undefined;
    if (!data || data.parsed?.type !== 'mint') throw new Error('Address is not an SPL token mint');
    const decimals = data.parsed.info?.decimals;
    if (typeof decimals !== 'number') throw new Error('Could not read mint decimals');
    return decimals;
}

export function AddTokenDialog({ open, onOpenChange }: { open: boolean; onOpenChange: (open: boolean) => void }) {
    const { id, url } = useClusterConfig();
    const network = clusterIdToNetwork(id);
    const queryClient = useQueryClient();
    const { setSelectedMint } = useSelectedToken();
    const [mint, setMint] = useState('');
    const [symbol, setSymbol] = useState('');
    const [name, setName] = useState('');
    const [saving, setSaving] = useState(false);
    const [listVersion, setListVersion] = useState(0);

    void listVersion;
    const customTokens = readCustomTokens(network);

    const refresh = async () => {
        await queryClient.invalidateQueries({ queryKey: ['network-config'] });
        setListVersion(v => v + 1);
    };

    async function handleAdd() {
        const trimmedMint = mint.trim();
        const trimmedSymbol = symbol.trim();
        if (!trimmedMint || !trimmedSymbol) {
            toast.error('Enter a mint address and a symbol');
            return;
        }
        setSaving(true);
        try {
            const decimals = await fetchMintDecimals(url, trimmedMint);
            addCustomToken(network, {
                decimals,
                mint: trimmedMint,
                name: name.trim() || trimmedSymbol,
                symbol: trimmedSymbol,
            });
            await refresh();
            setSelectedMint(trimmedMint);
            toast.success(`Added ${trimmedSymbol}`);
            setMint('');
            setSymbol('');
            setName('');
            onOpenChange(false);
        } catch (error) {
            toast.error(error instanceof Error ? error.message : 'Failed to add token');
        } finally {
            setSaving(false);
        }
    }

    async function handleRemove(tokenMint: string) {
        removeCustomToken(network, tokenMint);
        await refresh();
    }

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Add token</DialogTitle>
                    <DialogDescription>
                        Track an SPL token on {network}. Decimals are read from the mint; symbol and name are yours.
                    </DialogDescription>
                </DialogHeader>

                <div className="space-y-3">
                    <TextInput
                        value={mint}
                        onChange={e => setMint(e.currentTarget.value)}
                        placeholder="Mint address"
                        inputClassName="font-mono"
                    />
                    <div className="flex gap-3">
                        <TextInput
                            value={symbol}
                            onChange={e => setSymbol(e.currentTarget.value)}
                            placeholder="Symbol"
                        />
                        <TextInput
                            value={name}
                            onChange={e => setName(e.currentTarget.value)}
                            placeholder="Name (optional)"
                        />
                    </div>
                </div>

                {customTokens.length > 0 && (
                    <div className="space-y-2 pt-2 border-t border-sand-200">
                        <p className="text-xs font-medium text-sand-1000 uppercase tracking-wider">Custom tokens</p>
                        {customTokens.map(token => (
                            <div key={token.mint} className="flex items-center justify-between text-sm">
                                <span>
                                    <span className="font-medium text-foreground">{token.symbol}</span>{' '}
                                    <span className="font-mono text-xs text-sand-1000">{ellipsify(token.mint)}</span>
                                </span>
                                <Button
                                    variant="secondary"
                                    size="sm"
                                    iconOnly
                                    iconLeft={<Trash2 />}
                                    aria-label={`Remove ${token.symbol}`}
                                    onClick={() => void handleRemove(token.mint)}
                                />
                            </div>
                        ))}
                    </div>
                )}

                <DialogFooter>
                    <Button variant="secondary" disabled={saving} onClick={() => onOpenChange(false)}>
                        Cancel
                    </Button>
                    <Button loading={saving} onClick={() => void handleAdd()}>
                        Add token
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
}
