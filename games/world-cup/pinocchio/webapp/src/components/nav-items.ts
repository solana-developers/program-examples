import { type LucideIcon, Trophy } from 'lucide-react';

export interface NavItem {
    children?: NavItem[];
    clusterFilter?: string[];
    icon: LucideIcon;
    label: string;
    path: string;
}

export const NAV_ITEMS: NavItem[] = [{ icon: Trophy, label: 'Leaderboard', path: '/leaderboard' }];
