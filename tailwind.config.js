/** @type {import('tailwindcss').Config} */
const primeui = require('tailwindcss-primeui')
export const darkMode = ['media']
export const content = [
  './index.html',
  './pages/**/*.{js,ts,tsx,vue}',
  './components/**/*.{js,ts,tsx,vue}',
  './app/**/*.{js,ts,tsx,vue}',
  './src/**/*.{js,ts,tsx,vue}',
]
export const prefix = ''
export const theme = {
  container: {
    center: true,
    padding: '2rem',
    screens: {
      '2xl': '1400px',
    },
  },
  extend: {},
}
export const plugins = [primeui]
