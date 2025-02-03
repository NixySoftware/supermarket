// @ts-check
import eslint from '@eslint/js';
import eslintImport from 'eslint-plugin-import';
import jsxA11y from 'eslint-plugin-jsx-a11y';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import tseslint from 'typescript-eslint';

export default tseslint.config(
    {
        ignores: ['.vite/', 'build/', 'coverage/'],
    },

    // ESLint
    eslint.configs.recommended,

    // TypeScript ESLint
    /* eslint-disable import/no-named-as-default-member */
    tseslint.configs.strictTypeChecked,
    tseslint.configs.stylisticTypeChecked,
    /* eslint-enable import/no-named-as-default-member */
    {
        languageOptions: {
            parserOptions: {
                projectService: true,
            },
        },
        rules: {
            '@typescript-eslint/consistent-type-definitions': ['error', 'type'],
            '@typescript-eslint/consistent-type-exports': [
                'error',
                {
                    fixMixedExportsWithInlineTypeSpecifier: true,
                },
            ],
            '@typescript-eslint/consistent-type-imports': [
                'error',
                {
                    fixStyle: 'inline-type-imports',
                },
            ],
            '@typescript-eslint/no-unused-vars': [
                'error',
                {
                    argsIgnorePattern: '^_',
                    caughtErrorsIgnorePattern: '^_',
                    destructuredArrayIgnorePattern: '^_',
                    varsIgnorePattern: '^_',
                },
            ],
            '@typescript-eslint/non-nullable-type-assertion-style': 'off',
        },
    },

    // Import
    eslintImport.flatConfigs.recommended,
    eslintImport.flatConfigs.typescript,
    {
        settings: {
            'import/internal-regex': '^~/',
            'import/resolver': {
                node: {
                    extensions: ['.ts', '.tsx'],
                },
                typescript: {
                    alwaysTryTypes: true,
                },
            },
        },
        rules: {
            'import/namespace': ['error', { allowComputed: true }],
            // TypeScript alreadys checks imports.
            'import/no-unresolved': 'off',
        },
    },

    // React
    react.configs.flat.recommended,
    react.configs.flat['jsx-runtime'],
    {
        settings: {
            react: {
                version: 'detect',
            },
            formComponents: ['Form'],
            linkComponents: [
                { name: 'Link', linkAttribute: 'to' },
                { name: 'NavLink', linkAttribute: 'to' },
            ],
        },
    },

    // React Hooks
    {
        plugins: {
            'react-hooks': reactHooks,
        },
        rules: reactHooks.configs.recommended.rules,
    },

    // JSX Accessibility
    jsxA11y.flatConfigs.recommended,
    {
        rules: {
            // TODO: Consider introducing an accessibility setting for autofocus.
            'jsx-a11y/no-autofocus': 'off',
        },
    },
);
