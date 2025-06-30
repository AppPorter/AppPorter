import pluginJs from '@eslint/js'
import eslintConfigPrettier from 'eslint-config-prettier/flat'
import tailwind from 'eslint-plugin-tailwindcss'
import pluginVue from 'eslint-plugin-vue'
import globals from 'globals'
import tseslint from 'typescript-eslint'

export default [
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/base'],
  ...tailwind.configs['flat/recommended'],
  eslintConfigPrettier,
  {
    languageOptions: {
      globals: {
        ...globals.browser,
      },
    },
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: pluginVue.parser,
      parserOptions: {
        parser: tseslint.parser,
        ecmaVersion: 'latest',
        sourceType: 'module',
        extraFileExtensions: ['.vue'],
        ecmaFeatures: { jsx: true },
      },
    },
  },
  {
    files: ['**/*.ts', '**/*.tsx'],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: { jsx: true },
      },
    },
  },
  {
    rules: {
      'vue/multi-word-component-names': 'off',
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      'tailwindcss/no-custom-classname': [
        'warn',
        {
          whitelist: ['^mir-.*'],
        },
      ],
      'vue/first-attribute-linebreak': 'off',
    },
  },
  {
    ignores: ['dist/', 'node_modules/', '.output/', '*.config.js', '*.cjs', '*.mjs'],
  },
]
