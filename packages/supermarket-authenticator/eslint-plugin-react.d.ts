declare module 'eslint-plugin-react' {
    import type { TSESLint } from '@typescript-eslint/utils';

    const plugin: {
        configs: {
            flat: {
                all: TSESLint.FlatConfig.Config;
                recommended: TSESLint.FlatConfig.Config;
                'jsx-runtime': TSESLint.FlatConfig.Config;
            };
        };
    };

    export default plugin;
}
