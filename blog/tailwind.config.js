/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [require("daisyui"), require("@tailwindcss/typography")],


  //
  // @plugin "daisyui/theme" {
  //   name: "solarpunk";
  //   default: true;
  //   prefersdark: false;
  //   color-scheme: "light";
  //   --color-base-100: oklch(84% 0.143 164.978);
  //   --color-base-200: oklch(79% 0.209 151.711);
  //   --color-base-300: oklch(62% 0.194 149.214);
  //   --color-base-content: oklch(0% 0 0);
  //   --color-primary: oklch(92% 0.12 95.746);
  //   --color-primary-content: oklch(14.844% 0.041 6.35);
  //   --color-secondary: oklch(83.33% 0.184 204.72);
  //   --color-secondary-content: oklch(16.666% 0.036 204.72);
  //   --color-accent: oklch(71.86% 0.217 310.43);
  //   --color-accent-content: oklch(14.372% 0.043 310.43);
  //   --color-neutral: oklch(23.04% 0.065 269.31);
  //   --color-neutral-content: oklch(94.51% 0.179 104.32);
  //   --color-info: oklch(72.06% 0.191 231.6);
  //   --color-info-content: oklch(0% 0 0);
  //   --color-success: oklch(64.8% 0.15 160);
  //   --color-success-content: oklch(0% 0 0);
  //   --color-warning: oklch(75% 0.183 55.934);
  //   --color-warning-content: oklch(0% 0 0);
  //   --color-error: oklch(71.76% 0.221 22.18);
  //   --color-error-content: oklch(0% 0 0);
  //   --radius-selector: 0rem;
  //   --radius-field: 0rem;
  //   --radius-box: 0rem;
  //   --size-selector: 0.25rem;
  //   --size-field: 0.25rem;
  //   --border: 1px;
  //   --depth: 0;
  //   --noise: 0;
  // }

  daisyui: {
    themes: ["solarpunk", "light", "dark"],
    darkTheme: "solarpunk",
    base: true,
    styled: true,
    utils: true,
    prefix: "",
    logs: false,
    themeRoot: ":root",
    classPrefix: "",
    themes: {
      solarpunk: {
        primary: "#56AB2F",
        secondary: "#FFD700",
        accent: "#FFA500",
        neutral: "#4A5568",
        "base-100": "#F0F6EC",
        "base-200": "#D9E9D5",
        "base-300": "#C2DCC1",
        "base-content": "#2A333F",
        info: "#2E9AFF",
        success: "#38A169",
        warning: "#FFB02E",
        error: "#FF5E5B",
        "code_bg": "#E5E9F0",
        "code_fg": "#243B53"
      }
    }
  },
};
