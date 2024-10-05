module.exports = {
  mode: "all",
  content: [
      "./src/**/*.rs",
      "./dist/**/*.html",
      "./index.html",
  ],
  theme: {
      extend: {},
  },
  safelist: [
   "px-4", "py-2", "bg-blue-500", "text-white", "rounded", "hover:bg-blue-600"
  ],
  plugins: [],
}
