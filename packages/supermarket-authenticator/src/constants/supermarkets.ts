export type Supermarket = {
    id: string;
    name: string;
    authorizationUrl: string;
    redirectUrlPrefix: string;
    codeParam: string;
};

export const SUPERMARKETS: Supermarket[] = [
    {
        id: 'albert-heijn',
        name: 'Albert Heijn',
        authorizationUrl:
            'https://login.ah.nl/login?response_type=code&client_id=appie-android&redirect_uri=appie://login-exit',
        redirectUrlPrefix: 'appie://login-exit',
        codeParam: 'code',
    },
    {
        id: 'jumbo',
        name: 'Jumbo',
        authorizationUrl: 'https://loyalty-app.jumbo.com',
        redirectUrlPrefix: 'https://loyalty-app.jumbo.com/home',
        codeParam: 'code',
    },
];

export const getCodeFromRedirectUrl = (redirectUrl: string): [Supermarket, string] | undefined => {
    for (const supermarket of SUPERMARKETS) {
        if (redirectUrl.startsWith(supermarket.redirectUrlPrefix)) {
            const params = new URL(redirectUrl).searchParams;
            const code = params.get(supermarket.codeParam);

            if (code === null) {
                continue;
            }

            return [supermarket, code];
        }
    }

    return undefined;
};
