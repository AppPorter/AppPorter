/** @type {import('tailwindcss').Config} */
import primeui from 'tailwindcss-primeui'

export default {
  content: [
    './index.html',
    './pages/**/*.{js,ts,tsx,vue}',
    './components/**/*.{js,ts,tsx,vue}',
    './app/**/*.{js,ts,tsx,vue}',
    './src/**/*.{js,ts,tsx,vue}',
  ],
  darkMode: ['media'],
  plugins: [primeui],
}
