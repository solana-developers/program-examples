import { cn } from '@/lib/utils';

interface BlurredOverlayProps {
    /** Whether the blur effect is active */
    active?: boolean;
    /** Content to render (will be blurred when active) */
    children: React.ReactNode;
    /** Optional message to display over the blurred content */
    message?: string;
    /** Additional class names for the container */
    className?: string;
}

/**
 * A component that applies a blur and opacity effect to its children.
 * Used for progressive disclosure - showing disabled/unavailable UI sections.
 */
export function BlurredOverlay({ active = true, children, message, className }: BlurredOverlayProps) {
    return (
        <div className={cn('relative', className)}>
            <div
                className={cn(
                    'transition-all duration-300',
                    active && 'blur-sm opacity-40 pointer-events-none select-none',
                )}
            >
                {children}
            </div>
            {active && message && (
                <div className="absolute inset-0 flex items-center justify-center">
                    <p className="text-muted-foreground text-sm font-medium bg-background/80 px-4 py-2 rounded-md">
                        {message}
                    </p>
                </div>
            )}
        </div>
    );
}
