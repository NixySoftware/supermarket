import { Button } from '~/components/ui/button';
import { SUPERMARKETS } from '~/constants/supermarkets';

export const App = () => {
    const startUrl = `${window.location.origin}${window.location.pathname}`;
    const params = new URLSearchParams(window.location.search);
    const supermarketId = params.get('supermarket-id');
    const supermarketCode = params.get('supermarket-code');
    const supermarket = SUPERMARKETS.find((supermarket) => supermarket.id === supermarketId);

    return (
        <div className="flex justify-center">
            <div className="flex max-w-xl flex-col items-center gap-4 p-8">
                <h1 className="text-xl font-bold">Supermarket Authenticator</h1>

                {supermarket && supermarketCode ? (
                    <div className="flex flex-col items-center gap-4">
                        <h2 className="font-bold">{supermarket.name}</h2>
                        <code className="bg-muted rounded-md px-4 py-2 font-mono">{supermarketCode}</code>

                        <Button asChild>
                            <a href={startUrl}>Back to start</a>
                        </Button>
                    </div>
                ) : (
                    <div className="flex w-full flex-col gap-2">
                        {SUPERMARKETS.map((supermarket) => (
                            <Button key={supermarket.id} className="flex-grow" asChild>
                                <a href={supermarket.authorizationUrl}>{supermarket.name}</a>
                            </Button>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
};
