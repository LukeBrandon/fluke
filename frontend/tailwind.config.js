/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}"
  ],
  theme: {
    extend: {
      colors: {
        'fluke-bg-default': '#0d1111',
        'fluke-bg-surface': '#e5e7eb',
        'fluke-bg-overlay': '#161b22',
        'fluke-text-primary': '#5dacbd',
        'fluke-text-hover': '#1565c',
        'fluke-text-secondary': '#2363eb',
        'fluke-text-active': '#0d47a1',
        'fluke-text-warning': '#ef4444',
      },
      fontFamily: {
        Poppins: ['Poppins, sans-serif'],
        sans: ['Graphik', 'sans-serif'],
        serif: ['Merriweather', 'serif'],
      },
      container: {
        center: true,
        padding: '1rem',
        // these are default, customize at some point
        screens: {
          'sm': '640px',
          'md': '768px',
          'lg': '1024px',
          'xl': '1280px',
          '2xl': '1536px',
        },
      },
    },
  },
  plugins: [],
};

