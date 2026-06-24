import { useState } from 'react';
import { useNavigate } from 'react-router';
import { isAddress } from '@solana/kit';
import { Button, TextInput } from '@solana/design-system';
import { Search } from 'lucide-react';

import { cn } from '@/lib/utils';

interface BracketSearchProps {
    className?: string;
}

/** Look up any wallet's bracket by address; navigates to the shareable view. */
export function BracketSearch({ className }: BracketSearchProps) {
    const [value, setValue] = useState('');
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    function handleSubmit(event: React.FormEvent) {
        event.preventDefault();
        const trimmed = value.trim();
        if (!isAddress(trimmed)) {
            setError('Enter a valid Solana wallet address.');
            return;
        }
        setError(null);
        navigate(`/b/${trimmed}`);
    }

    return (
        <form onSubmit={handleSubmit} className={cn('w-full', className)}>
            <div className="flex items-stretch gap-2">
                <TextInput
                    value={value}
                    onChange={e => {
                        setValue(e.currentTarget.value);
                        if (error) setError(null);
                    }}
                    placeholder="Look up a bracket by wallet address"
                    inputClassName="font-mono text-sm"
                    aria-label="Wallet address"
                    aria-invalid={error != null}
                />
                <Button type="submit" iconLeft={<Search />} variant="secondary">
                    View
                </Button>
            </div>
            {error && <p className="mt-1.5 text-xs text-destructive">{error}</p>}
        </form>
    );
}
