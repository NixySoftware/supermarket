declare module 'eslint-plugin-react' {
    import type { Linter } from 'eslint';

    const plugin: {
        configs: {
            flat: {
                all: Linter.Config;
                recommended: Linter.Config;
                'jsx-runtime': Linter.Config;
            };
        };
    };

    export default plugin;
}
