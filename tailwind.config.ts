import type { Config } from "tailwindcss";

export default {
  content: ["./templates/*.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
} satisfies Config;
