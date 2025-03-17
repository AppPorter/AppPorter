import pluginJs from '@eslint/js'
import tailwind from 'eslint-plugin-tailwindcss'
import pluginVue from 'eslint-plugin-vue'
import globals from 'globals'
import tseslint from 'typescript-eslint'

/** @type {import('eslint').Linter.Config[]} */
export default [
  { files: ['**/*.{js,mjs,cjs,ts,vue}'] },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/base'],
  ...tailwind.configs['flat/recommended'],
  {
    files: ['**/*.{js,mjs,cjs,ts,vue}'],
    plugins: { tailwindcss: tailwind },
    rules: {
      'tailwindcss/no-custom-classname': 'off',
      '@typescript-eslint/no-unused-vars': 'warn',
    },
  },
  { files: ['**/*.vue'], languageOptions: { parserOptions: { parser: tseslint.parser } } },
]
