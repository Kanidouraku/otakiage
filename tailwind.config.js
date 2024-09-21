/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./assets/**/*.{hbs,js,html}"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ['bumblebee']
  }
}
