import { defineConfig } from 'vite';

export default defineConfig((env) => {
    const forgeEnv = env as unknown as { forgeConfigSelf: { entry: string } };
    const { forgeConfigSelf } = forgeEnv;

    return {
        build: {
            lib: {
                entry: forgeConfigSelf.entry,
                fileName: () => '[name].js',
                formats: ['es'],
            },
        },
        resolve: {
            tsconfigPaths: true,
        },
    };
});
