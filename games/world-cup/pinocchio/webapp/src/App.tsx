import { Route, Routes, Navigate } from 'react-router';
import { AppProviders } from '@/components/app-providers';
import { AppLayout } from '@/components/app-layout';
import { Home } from '@/routes/home';
import { BracketView } from '@/routes/bracket-view';
import { Leaderboard } from '@/routes/leaderboard';

export default function App() {
    return (
        <AppProviders>
            <Routes>
                <Route
                    element={
                        <AppLayout>
                            <Home />
                        </AppLayout>
                    }
                    path="/"
                />
                <Route
                    element={
                        <AppLayout>
                            <Leaderboard />
                        </AppLayout>
                    }
                    path="/leaderboard"
                />
                <Route
                    element={
                        <AppLayout>
                            <BracketView />
                        </AppLayout>
                    }
                    path="/b/:address"
                />
                <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
        </AppProviders>
    );
}
