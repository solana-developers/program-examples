import { useState, useEffect, useCallback, useRef } from 'react';
import { Clock, RotateCcw } from 'lucide-react';
import { useCluster } from '@solana/connector/react';
import { Select, SelectItem, TextInput } from '@solana/design-system';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { useTimeTravel } from '@/hooks/use-time-travel';
import { useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { fmtDate, fmtDateTime } from '@/lib/utils';

const QUICK_JUMPS = [
    { label: '+1h', seconds: 3600 },
    { label: '+6h', seconds: 21600 },
    { label: '+1d', seconds: 86400 },
    { label: '+7d', seconds: 604800 },
    { label: '+30d', seconds: 2592000 },
] as const;

const TIME_TRAVELED_KEY = 'time-traveled';

function TimeTravelButtonInner() {
    const { timeTravel, getCurrentTimestamp } = useTimeTravel();
    const queryClient = useQueryClient();
    const [open, setOpen] = useState(false);
    const [currentTime, setCurrentTime] = useState<number | null>(null);
    const [loading, setLoading] = useState(false);
    const [date, setDate] = useState('');
    const [hour, setHour] = useState('12');
    const [timeTraveled, setTimeTraveled] = useState(() => sessionStorage.getItem(TIME_TRAVELED_KEY) === 'true');

    const markTimeTraveled = useCallback(() => {
        sessionStorage.setItem(TIME_TRAVELED_KEY, 'true');
        setTimeTraveled(true);
    }, []);

    const fetchTime = useCallback(async () => {
        try {
            const ts = await getCurrentTimestamp();
            setCurrentTime(ts);
        } catch (e) {
            console.warn('[TimeTravel] Failed to fetch clock:', e);
        }
    }, [getCurrentTimestamp]);

    useEffect(() => {
        let cancelled = false;
        queueMicrotask(() => {
            if (!cancelled) void fetchTime();
        });
        return () => {
            cancelled = true;
        };
    }, [fetchTime]);

    useEffect(() => {
        if (!open) return;
        let cancelled = false;
        queueMicrotask(() => {
            if (!cancelled) void fetchTime();
        });
        return () => {
            cancelled = true;
        };
    }, [open, fetchTime]);

    const animRef = useRef<number>(0);
    const dateDisplayRef = useRef<HTMLDivElement>(null);
    const [animating, setAnimating] = useState(false);

    const handleQuickJump = (seconds: number) => {
        if (!currentTime) return;
        if (animRef.current) cancelAnimationFrame(animRef.current);

        const startTs = currentTime;
        const endTs = currentTime + seconds;
        const duration = 1400;
        let startMs = 0;
        setAnimating(true);

        const step = (now: number) => {
            if (startMs === 0) startMs = now;
            const elapsed = now - startMs;
            const t = Math.min(elapsed / duration, 1);
            const eased = 1 - Math.pow(1 - t, 4);
            const ts = startTs + (endTs - startTs) * eased;
            const d = new Date(ts * 1000);
            setDate(d.toLocaleDateString('en-CA'));
            setHour(d.getHours().toString());

            const speed = 4 * Math.pow(1 - t, 3);
            const normalizedSpeed = Math.min(speed / 4, 1);
            const flicker = Math.sin(elapsed * 0.025) * 0.15 * normalizedSpeed;
            const opacity = 0.8 + 0.2 * (1 - normalizedSpeed) + flicker;
            if (dateDisplayRef.current) {
                dateDisplayRef.current.style.opacity = String(Math.max(0.55, Math.min(1, opacity)));
            }

            if (t < 1) {
                animRef.current = requestAnimationFrame(step);
            } else {
                if (dateDisplayRef.current) dateDisplayRef.current.style.opacity = '1';
                setAnimating(false);
                animRef.current = 0;
                setLoading(true);
                timeTravel(endTs)
                    .then(() => {
                        markTimeTraveled();
                        return fetchTime();
                    })
                    .then(() => {
                        queryClient.invalidateQueries();
                        setTimeout(() => queryClient.invalidateQueries(), 500);
                        toast.success('Clock set');
                    })
                    .catch((e: unknown) => {
                        toast.error(e instanceof Error ? e.message : 'Time travel failed');
                    })
                    .finally(() => setLoading(false));
            }
        };
        animRef.current = requestAnimationFrame(step);
    };

    const handleCustomJump = async () => {
        if (!date || !currentTime) return;
        setLoading(true);
        try {
            const ts = Math.floor(new Date(`${date}T${hour.padStart(2, '0')}:00:00`).getTime() / 1000);
            if (ts <= currentTime) {
                toast.error('Cannot travel to the past');
                setLoading(false);
                return;
            }
            await timeTravel(ts);
            markTimeTraveled();
            await fetchTime();
            queryClient.invalidateQueries();
            setTimeout(() => queryClient.invalidateQueries(), 500);
            toast.success('Clock set');
        } catch (e) {
            toast.error(e instanceof Error ? e.message : 'Time travel failed');
        } finally {
            setLoading(false);
        }
    };

    return (
        <Dialog open={open} onOpenChange={setOpen} modal={false}>
            <DialogTrigger asChild>
                <Button
                    variant="outline"
                    size="sm"
                    className={`px-2 py-[6px] h-auto gap-1.5 ${
                        timeTraveled
                            ? 'relative ring-2 ring-green-500/50 shadow-[0_0_12px_rgba(34,197,94,0.4)] animate-[glow_3s_ease-in-out_infinite]'
                            : 'relative'
                    }`}
                    title="Time Travel (Dev)"
                >
                    <Clock className={timeTraveled ? 'h-4 w-4 text-foreground' : 'h-4 w-4 text-muted-foreground'} />
                    {currentTime !== null && (
                        <span
                            className={`text-sm font-semibold ${timeTraveled ? 'text-foreground' : 'text-muted-foreground'}`}
                        >
                            {fmtDate(currentTime)}
                        </span>
                    )}
                </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-md">
                <DialogHeader>
                    <DialogTitle className="flex items-center gap-2">
                        <Clock className="h-5 w-5 text-sand-1100" />
                        Time Travel
                    </DialogTitle>
                </DialogHeader>

                <div className="grid gap-4">
                    <div className="rounded-md bg-sand-1400 p-3 text-sm font-mono text-center">
                        {currentTime !== null ? fmtDateTime(currentTime) : 'Fetching...'}
                    </div>

                    <div className="grid gap-2">
                        <Label className="text-xs font-medium uppercase tracking-wider text-sand-1000">
                            Quick Jump
                        </Label>
                        <div className="flex gap-2 flex-wrap">
                            {QUICK_JUMPS.map(({ label, seconds }) => (
                                <Button
                                    key={label}
                                    variant="outline"
                                    size="sm"
                                    disabled={loading || !currentTime}
                                    onClick={() => handleQuickJump(seconds)}
                                >
                                    {label}
                                </Button>
                            ))}
                        </div>
                    </div>

                    <div className="h-px bg-border" />

                    <div className="grid gap-2">
                        <Label className="text-xs font-medium uppercase tracking-wider text-sand-1000">
                            Jump to Date
                        </Label>
                        <div ref={dateDisplayRef} className="flex gap-2">
                            <TextInput
                                type="date"
                                value={date}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setDate(e.target.value)}
                                min={currentTime ? new Date(currentTime * 1000).toLocaleDateString('en-CA') : undefined}
                                className={`flex-1 transition-shadow duration-200 ${animating ? 'ring-2 ring-foreground/30 border-foreground/25 text-foreground' : ''}`}
                            />
                            <Select
                                value={hour}
                                onValueChange={value => {
                                    if (value) setHour(value);
                                }}
                                className={`w-28 shrink-0 transition-shadow duration-200 ${animating ? 'ring-2 ring-foreground/30 border-foreground/25' : ''}`}
                            >
                                {Array.from({ length: 24 }, (_, i) => (
                                    <SelectItem key={i} value={i.toString()}>
                                        {i.toString().padStart(2, '0')}:00
                                    </SelectItem>
                                ))}
                            </Select>
                        </div>
                        <Button
                            variant="outline"
                            disabled={loading || !date}
                            onClick={handleCustomJump}
                            className={`w-full font-semibold text-base h-11 rounded-full border-foreground/30 text-foreground relative overflow-hidden ${date ? 'bg-transparent' : 'bg-foreground hover:bg-foreground/90 text-background'}`}
                        >
                            {date && (
                                <span className="absolute inset-0 animate-[shimmer_2s_ease-in-out_infinite] bg-[length:200%_100%] bg-gradient-to-r from-green-700 via-green-500 to-green-700" />
                            )}
                            <span className="relative z-10">Jump</span>
                        </Button>
                    </div>

                    {timeTraveled && (
                        <>
                            <div className="h-px bg-border" />
                            <div className="flex items-start gap-2 rounded-md border border-amber-300 bg-amber-50 p-3 text-sm text-amber-700">
                                <RotateCcw className="h-4 w-4 mt-0.5 shrink-0" />
                                <span>
                                    Clock is ahead of system time. Run{' '}
                                    <code className="px-1 py-0.5 rounded bg-sand-1400 text-xs">just webapp-clean</code>{' '}
                                    and restart to reset.
                                </span>
                            </div>
                        </>
                    )}
                </div>
            </DialogContent>
        </Dialog>
    );
}

export function TimeTravelButton() {
    const { cluster } = useCluster();
    if (cluster?.id !== 'solana:localnet') return null;
    return <TimeTravelButtonInner />;
}
