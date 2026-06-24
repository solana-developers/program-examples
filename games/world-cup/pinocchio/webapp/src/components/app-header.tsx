import { useEffect, useState } from 'react';
import { Link, useLocation } from 'react-router';
import { useCluster } from '@solana/connector/react';
import { Button, TextInput } from '@solana/design-system';
import { ChevronDown, Menu, Plus, Settings2, Trash2 } from 'lucide-react';
import { toast } from 'sonner';

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import solanaLogo from '@/assets/solana-logo.svg';
import { clearCustomRpc, detectNetwork, isValidRpcUrl, readCustomRpc, saveCustomRpc } from '@/lib/custom-rpc';
import { cn } from '@/lib/utils';

import { NAV_ITEMS, type NavItem } from './nav-items';
import { WalletButton } from './solana/solana-provider';
import { TimeTravelButton } from './time-travel/time-travel-button';

function ClusterButton() {
    const { cluster, clusters, setCluster } = useCluster();
    const [dialogOpen, setDialogOpen] = useState(false);
    const [url, setUrl] = useState('');
    const [saving, setSaving] = useState(false);

    const hasCustom = readCustomRpc() !== null;

    function openDialog() {
        setUrl(readCustomRpc()?.url ?? '');
        setDialogOpen(true);
    }

    async function handleSave() {
        const trimmed = url.trim();
        if (!isValidRpcUrl(trimmed)) {
            toast.error('Enter a valid http(s) RPC URL');
            return;
        }
        setSaving(true);
        try {
            const network = await detectNetwork(trimmed);
            if (!network) {
                toast.error('Could not detect mainnet, devnet, or testnet from this RPC');
                return;
            }
            saveCustomRpc(trimmed, network);
            window.location.reload();
        } catch {
            toast.error('Could not reach RPC URL');
        } finally {
            setSaving(false);
        }
    }

    function handleRemove() {
        clearCustomRpc();
        window.location.reload();
    }

    return (
        <>
            <DropdownMenu>
                <DropdownMenuTrigger asChild>
                    <Button
                        iconLeft={<Settings2 />}
                        iconRight={<ChevronDown className="opacity-60" />}
                        size="sm"
                        variant="secondary"
                    >
                        {cluster?.label ?? 'Network'}
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" className="w-44">
                    <DropdownMenuLabel>Network</DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    {clusters.map(c => (
                        <DropdownMenuItem
                            key={c.id}
                            onClick={() => {
                                void setCluster(c.id);
                            }}
                        >
                            {c.label}
                        </DropdownMenuItem>
                    ))}
                    <DropdownMenuSeparator />
                    <DropdownMenuItem onClick={openDialog}>
                        <Plus className="mr-2 h-4 w-4" />
                        {hasCustom ? 'Edit custom RPC' : 'Add custom RPC'}
                    </DropdownMenuItem>
                    {hasCustom && (
                        <DropdownMenuItem className="text-destructive focus:text-destructive" onClick={handleRemove}>
                            <Trash2 className="mr-2 h-4 w-4" />
                            Remove custom RPC
                        </DropdownMenuItem>
                    )}
                </DropdownMenuContent>
            </DropdownMenu>

            <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Custom RPC endpoint</DialogTitle>
                        <DialogDescription>
                            Point the app at your own Solana RPC URL. The network is detected from the endpoint; saving
                            reloads the page and selects it.
                        </DialogDescription>
                    </DialogHeader>
                    <TextInput
                        value={url}
                        onChange={e => setUrl(e.currentTarget.value)}
                        placeholder="https://my-rpc.example.com"
                        inputClassName="font-mono"
                    />
                    <DialogFooter>
                        <Button variant="secondary" disabled={saving} onClick={() => setDialogOpen(false)}>
                            Cancel
                        </Button>
                        <Button loading={saving} onClick={() => void handleSave()}>
                            Save
                        </Button>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
        </>
    );
}

function isActive(pathname: string, path: string): boolean {
    return path === '/' ? pathname === '/' : pathname.startsWith(path);
}

function NavLinks({ items, pathname }: { items: NavItem[]; pathname: string }) {
    return (
        <>
            {items.map(item => {
                if (item.children?.length) {
                    return <NavParent key={item.path} item={item} pathname={pathname} />;
                }
                const active = isActive(pathname, item.path);
                return (
                    <Link
                        key={item.path}
                        to={item.path}
                        className={cn(
                            'rounded-full px-3 py-2 text-sm font-medium transition-colors',
                            active
                                ? 'text-foreground bg-sand-200'
                                : 'text-sand-1100 hover:text-foreground hover:bg-sand-100',
                        )}
                    >
                        {item.label}
                    </Link>
                );
            })}
        </>
    );
}

function NavParent({ item, pathname }: { item: NavItem; pathname: string }) {
    const active = isActive(pathname, item.path) || item.children?.some(c => isActive(pathname, c.path));
    return (
        <DropdownMenu>
            <DropdownMenuTrigger
                className={cn(
                    'inline-flex items-center gap-1 rounded-full px-3 py-2 text-sm font-medium transition-colors outline-none',
                    active ? 'text-foreground bg-sand-200' : 'text-sand-1100 hover:text-foreground hover:bg-sand-100',
                )}
            >
                {item.label}
                <ChevronDown className="h-3.5 w-3.5 opacity-60" />
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start" className="w-48">
                <DropdownMenuItem asChild>
                    <Link to={item.path}>{item.label}</Link>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                {item.children?.map(child => (
                    <DropdownMenuItem key={child.path} asChild>
                        <Link to={child.path} className="flex items-center gap-2">
                            <child.icon className="h-4 w-4" />
                            {child.label}
                        </Link>
                    </DropdownMenuItem>
                ))}
            </DropdownMenuContent>
        </DropdownMenu>
    );
}

export function AppHeader() {
    const { pathname } = useLocation();
    const { cluster } = useCluster();
    const [hasScrolled, setHasScrolled] = useState(false);

    useEffect(() => {
        function handleScroll() {
            const next = window.scrollY > 0;
            setHasScrolled(prev => (prev === next ? prev : next));
        }
        handleScroll();
        window.addEventListener('scroll', handleScroll, { passive: true });
        return () => window.removeEventListener('scroll', handleScroll);
    }, []);

    const filteredItems = NAV_ITEMS.filter(
        item => !item.clusterFilter || item.clusterFilter.includes(cluster?.id ?? ''),
    );

    return (
        <header
            className={cn(
                'fixed inset-x-0 top-0 z-40 border-b transition-colors duration-200',
                hasScrolled
                    ? 'bg-background/70 backdrop-blur-sm border-border-low/70'
                    : 'bg-transparent border-transparent',
            )}
        >
            <div className="mx-auto flex max-w-7xl items-center justify-between gap-4 px-6 py-4">
                <Link to="/" className="flex items-center gap-2 group">
                    <img src={solanaLogo} alt="Solana" className="h-6 w-6 shrink-0" />
                    <span className="text-foreground font-semibold text-lg tracking-tight">World Cup</span>
                </Link>

                <nav className="hidden md:flex items-center gap-1">
                    <NavLinks items={filteredItems} pathname={pathname} />
                </nav>

                <div className="hidden md:flex items-center gap-2">
                    <TimeTravelButton />
                    <WalletButton />
                    <ClusterButton />
                </div>

                <div className="md:hidden flex items-center gap-2">
                    <ClusterButton />
                    <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                            <Button
                                aria-label="Open navigation menu"
                                iconLeft={<Menu />}
                                iconOnly
                                size="sm"
                                variant="secondary"
                            />
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end" className="w-56">
                            {filteredItems.map(item => (
                                <DropdownMenuItem key={item.path} asChild>
                                    <Link to={item.path} className="flex items-center gap-2">
                                        <item.icon className="h-4 w-4" />
                                        {item.label}
                                    </Link>
                                </DropdownMenuItem>
                            ))}
                            <DropdownMenuSeparator />
                            <div className="p-2 flex flex-col gap-2">
                                <TimeTravelButton />
                                <WalletButton />
                            </div>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            </div>
        </header>
    );
}
