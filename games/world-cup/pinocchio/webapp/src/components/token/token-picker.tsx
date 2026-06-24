import { useState } from 'react';
import { Button } from '@solana/design-system';
import { ChevronDown, Plus } from 'lucide-react';

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import type { TokenConfig } from '@/config/networks';
import { useSelectedToken } from '@/hooks/use-selected-token';
import { ellipsify } from '@/lib/utils';
import { AddTokenDialog } from './add-token-dialog';

function tokenLabel(token: TokenConfig): string {
    return token.symbol || token.name || ellipsify(token.mint);
}

export function TokenPicker() {
    const { tokens, selectedMint, selectedToken, setSelectedMint } = useSelectedToken();
    const [addOpen, setAddOpen] = useState(false);

    return (
        <div className="flex items-center gap-2">
            {tokens && tokens.length > 1 && (
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Button size="sm" variant="secondary" iconRight={<ChevronDown className="opacity-60" />}>
                            {selectedToken ? tokenLabel(selectedToken) : 'Token'}
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="w-44">
                        {tokens.map(token => (
                            <DropdownMenuItem
                                key={token.mint}
                                onClick={() => setSelectedMint(token.mint)}
                                className={token.mint === selectedMint ? 'font-medium' : undefined}
                            >
                                {tokenLabel(token)}
                            </DropdownMenuItem>
                        ))}
                    </DropdownMenuContent>
                </DropdownMenu>
            )}
            <Button
                variant="secondary"
                size="sm"
                iconOnly
                iconLeft={<Plus />}
                aria-label="Add token"
                onClick={() => setAddOpen(true)}
            />
            <AddTokenDialog open={addOpen} onOpenChange={setAddOpen} />
        </div>
    );
}
