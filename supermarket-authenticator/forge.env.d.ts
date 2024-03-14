export {}; // Make this a module

declare global {
    // This allows TypeScript to pick up the magic constants that's auto-generated by Forge's Vite
    // plugin that tells the Electron app where to look for the Vite-bundled app code (depending on
    // whether you're running in development or production).
    const MAIN_WINDOW_VITE_DEV_SERVER_URL: string;
    const MAIN_WINDOW_VITE_NAME: string;

    namespace NodeJS {
        interface Process {
            // Used for hot reload after preload scripts.
            viteDevServers: Record<string, import('vite').ViteDevServer>;
        }
    }

    type VitePluginConfig = ConstructorParameters<typeof import('@electron-forge/plugin-vite').VitePlugin>[0];

    interface VitePluginRuntimeKeys {
        VITE_DEV_SERVER_URL: `${string}_VITE_DEV_SERVER_URL`;
        VITE_NAME: `${string}_VITE_NAME`;
    }
}

declare module 'vite' {
    interface ConfigEnv<K extends keyof VitePluginConfig = keyof VitePluginConfig> {
        root: string;
        forgeConfig: VitePluginConfig;
        forgeConfigSelf: VitePluginConfig[K][number];
    }
}
