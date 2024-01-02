/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      animation: {
        blob: "blob 7s infinite",
        blob2: "blob2 8s infinite",
      },
      keyframes: {
        blob: {
          "0%": {
            transform: "translate(0px, 0px) scale(1)",
          },
          "33%": {
            transform: "translate(30px, -50px) scale(1.1)",
          },
          "66%": {
            transform: "translate(-20px, 20px) scale(0.9)",
          },
          "100%": {
            transform: "translate(0px, 0px) scale(1)",
          },
        },
        blob2: {
          "0%": {
            transform: "translate(0px, 0px) scale(1)",
          },
          "33%": {
            transform: "translate(-51px, 30px) scale(1.1)",
          },
          "66%": {
            transform: "translate(20px, -20px) scale(0.9)",
          },
          "100%": {
            transform: "translate(0px, 0px) scale(1)",
          },
        },
      },
      boxShadow: {
        'white-md': '0 4px 6px -1px rgba(255, 255, 255, 0.1), 0 2px 4px -1px rgba(255, 255, 255, 0.06)',
      },
      backgroundImage: theme => ({
        'custom-gradient': "radial-gradient(at 56% 0%, rgb(22, 163, 74) 0, transparent 33%), radial-gradient(at 62% 23%, rgb(234, 179, 8) 0, transparent 49%), radial-gradient(at 42% 0%, rgb(16, 185, 129) 0, transparent 47%), radial-gradient(at 68% 26%, rgb(249, 115, 22) 0, transparent 65%), radial-gradient(at 0% 43%, rgb(249, 115, 22) 0, transparent 60%), radial-gradient(at 29% 21%, rgb(132, 204, 22) 0, transparent 69%), radial-gradient(at 4% 12%, rgb(245, 158, 11) 0, transparent 64%), radial-gradient(at 36% 40%, rgb(52, 211, 153) 0, transparent 100%)"
      }),
      colors: {
        'text': {
          50: 'var(--text-50)',
          100: 'var(--text-100)',
          200: 'var(--text-200)',
          300: 'var(--text-300)',
          400: 'var(--text-400)',
          500: 'var(--text-500)',
          600: 'var(--text-600)',
          700: 'var(--text-700)',
          800: 'var(--text-800)',
          900: 'var(--text-900)',
          950: 'var(--text-950)',
        },
        'background': {
          50: 'var(--background-50)',
          100: 'var(--background-100)',
          200: 'var(--background-200)',
          300: 'var(--background-300)',
          400: 'var(--background-400)',
          500: 'var(--background-500)',
          600: 'var(--background-600)',
          700: 'var(--background-700)',
          800: 'var(--background-800)',
          900: 'var(--background-900)',
          950: 'var(--background-950)',
        },
        'primary': {
          50: 'var(--primary-50)',
          100: 'var(--primary-100)',
          200: 'var(--primary-200)',
          300: 'var(--primary-300)',
          400: 'var(--primary-400)',
          500: 'var(--primary-500)',
          600: 'var(--primary-600)',
          700: 'var(--primary-700)',
          800: 'var(--primary-800)',
          900: 'var(--primary-900)',
          950: 'var(--primary-950)',
        },
        'secondary': {
          50: 'var(--secondary-50)',
          100: 'var(--secondary-100)',
          200: 'var(--secondary-200)',
          300: 'var(--secondary-300)',
          400: 'var(--secondary-400)',
          500: 'var(--secondary-500)',
          600: 'var(--secondary-600)',
          700: 'var(--secondary-700)',
          800: 'var(--secondary-800)',
          900: 'var(--secondary-900)',
          950: 'var(--secondary-950)',
        },
        'accent': {
          50: 'var(--accent-50)',
          100: 'var(--accent-100)',
          200: 'var(--accent-200)',
          300: 'var(--accent-300)',
          400: 'var(--accent-400)',
          500: 'var(--accent-500)',
          600: 'var(--accent-600)',
          700: 'var(--accent-700)',
          800: 'var(--accent-800)',
          900: 'var(--accent-900)',
          950: 'var(--accent-950)',
        },
      },

      fontFamily: {
        kanit: ["Kanit", ...defaultTheme.fontFamily.sans],
      },
    },
  },
  plugins: [
    require('tailwindcss'),
    require('autoprefixer'),
    function ({ addUtilities }) {
      const newUtilities = {
        '.input-dark': {
          'color-scheme': 'dark',
        },
      }
      addUtilities(newUtilities, ['dark'])
    },
  ],
};