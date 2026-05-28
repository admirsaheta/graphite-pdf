const config = {
    content: ["./src/**/*.rs", "./index.html"],
    mode: "jit",
    darkMode: "class",
    theme: {
        extend: {
            colors: {
                graphite: {
                    950: "#060604",
                    900: "#1E1E1C",
                    850: "#2E2E2C",
                    700: "#3A3A38",
                    600: "#606060",
                    500: "#6E6E6C",
                },
                mist: {
                    300: "#C4C4C0",
                },
                rust: {
                    400: "#F58040",
                    500: "#D4581A",
                    600: "#C84C14",
                    700: "#7C2806",
                },
                steel: {
                    400: "#585856",
                    500: "#666664",
                },
                base: {
                    bg: "#111110",
                    raised: "#1E1E1C",
                    surface: "#2E2E2C",
                    elevated: "#3A3A38",
                },
            },
            fontFamily: {
                sans: ["Space Grotesk", "Inter", "ui-sans-serif", "system-ui", "-apple-system", "Segoe UI", "sans-serif"],
                mono: ["JetBrains Mono", "SFMono-Regular", "Menlo", "Monaco", "Consolas", "Liberation Mono", "monospace"],
            },
            keyframes: {
                fadeOpacity: {
                    "0%, 100%": { opacity: "1" },
                    "50%": { opacity: "0.6" },
                },
                "rust-pulse": {
                    "0%, 100%": { boxShadow: "0 0 0 0 rgba(213,88,26,0)" },
                    "50%": { boxShadow: "0 0 20px 4px rgba(213,88,26,0.3)" },
                },
                "slide-in-left": {
                    "0%": { opacity: "0", transform: "translateX(-12px)" },
                    "100%": { opacity: "1", transform: "translateX(0)" },
                },
                "fade-up": {
                    "0%": { opacity: "0", transform: "translateY(16px)" },
                    "100%": { opacity: "1", transform: "translateY(0)" },
                },
                "spin-slow": {
                    "0%": { transform: "rotate(0deg)" },
                    "100%": { transform: "rotate(360deg)" },
                },
            },
            animation: {
                fadeOpacity: "fadeOpacity 8s infinite ease-in-out",
                "rust-pulse": "rust-pulse 3s ease-in-out infinite",
                "slide-in-left": "slide-in-left 0.3s cubic-bezier(0.16,1,0.3,1) forwards",
                "fade-up": "fade-up 0.5s cubic-bezier(0.16,1,0.3,1) forwards",
                "spin-slow": "spin-slow 12s linear infinite",
            },
        },
    },
    plugins: [],
};

if (typeof window !== "undefined") {
    window.tailwind = window.tailwind || {};
    window.tailwind.config = config;
}

if (typeof module !== "undefined" && module.exports) {
    module.exports = config;
}
