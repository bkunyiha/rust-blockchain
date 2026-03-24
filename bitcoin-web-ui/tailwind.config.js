/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'bitcoin-orange': '#f7931a',
        'bitcoin-gold': '#ffb347',
      },
    },
  },
  plugins: [],
}

