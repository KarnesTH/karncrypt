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
    extend: {
      colors: {
        primary: {
          100: "#EE05F2",
          200: "#AF13F2",
          300: "#A01AD9",
          400: "#6805F2",
        },
        background: {
          DEFAULT: "#444444",
          card: "#333333",
        },
      },
      backgroundImage: {
        "gradient-primary": "linear-gradient(120deg, #EE05F2, #6805F2)",
      },
    },
  },
  plugins: [],
};
