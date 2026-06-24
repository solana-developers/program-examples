import { Button as SolanaButton } from '@solana/design-system';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import React from 'react';

export function AppModal({
    children,
    title,
    submit,
    submitDisabled,
    submitLabel,
}: {
    children: React.ReactNode;
    title: string;
    submit?: () => void;
    submitDisabled?: boolean;
    submitLabel?: string;
}) {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button variant="outline">{title}</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[525px]">
                <DialogHeader>
                    <DialogTitle>{title}</DialogTitle>
                </DialogHeader>
                <div className="grid gap-4 py-4">{children}</div>
                <DialogFooter>
                    {submit ? (
                        <SolanaButton type="submit" onClick={submit} disabled={submitDisabled}>
                            {submitLabel || 'Save'}
                        </SolanaButton>
                    ) : null}
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
}
