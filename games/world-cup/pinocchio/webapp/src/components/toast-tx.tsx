import { toast } from 'sonner';
import { ExplorerLink } from '@/components/cluster/cluster-ui';

export function toastTx(signature?: string, title = 'Transaction sent') {
    if (!signature) {
        return;
    }
    toast(title, {
        description: <ExplorerLink transaction={signature} label="View Transaction" />,
    });
}
