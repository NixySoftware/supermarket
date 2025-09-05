import { type ConfigEnv, defineConfig } from 'vite';

export default defineConfig((env) => {
    const forgeEnv = env as ConfigEnv<'build'>;
    const { forgeConfigSelf } = forgeEnv;

    return {
        build: {
            lib: {
                entry: forgeConfigSelf.entry,
                fileName: () => '[name].js',
                formats: ['es'],
            },
        },
    };
});
