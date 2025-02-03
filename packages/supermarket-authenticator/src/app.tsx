import { Button } from 'src/components/ui/button';

export const App = () => {
    return (
        <div className="space-y-4 p-4">
            <h1 className="text-xl font-bold">Supermarket Authenticator</h1>

            <div className="grid gap-2">
                <Button>Albert Heijn</Button>
                <Button>Jumbo</Button>
            </div>
        </div>
    );
};
