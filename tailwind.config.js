/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "src/*/**.rs",
    "*.html",
    "src/components/*.rs",
    "src/*.rs",
    "src/components/**/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
