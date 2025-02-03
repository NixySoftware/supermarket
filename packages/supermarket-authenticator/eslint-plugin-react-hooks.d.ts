declare module 'eslint-plugin-react-hooks' {
    import type { ESLint } from 'eslint';

    const plugin: Omit<ESLint.Plugin, 'configs'> & {
        configs: {
            recommended: ESLint.ConfigData;
        };
    };

    export default plugin;
}
