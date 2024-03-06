/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        'comic': ['Comic Relief', 'cursive'],
      },
      boxShadow: {
        'comic': '4px 4px #1c1917',
      },
    },
  },
}
