import { AlertCircle } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import type { ReactNode } from 'react';

export function AppAlert({
    action,
    children,
    className = '',
}: {
    action?: ReactNode;
    children: ReactNode;
    className?: string;
}) {
    return (
        <Alert variant="warning" className={className}>
            <AlertCircle className="h-4 w-4" />
            <AlertTitle>{children}</AlertTitle>
            {action && <AlertDescription className="flex justify-end">{action}</AlertDescription>}
        </Alert>
    );
}
