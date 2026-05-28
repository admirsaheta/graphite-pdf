const config = {
    content: ["./src/**/*.rs", "./index.html"],
    darkMode: "media",
    theme: {
        extend: {
            fontFamily: {
                'admir-regular': ['AdmirRegular', 'ui-sans-serif', 'system-ui'],
                'admir-bold': ['AdmirBold', 'ui-sans-serif', 'system-ui'],
            },
            keyframes: {
                fadeOpacity: {
                    '0%, 100%': { opacity: '1' },
                    '50%': { opacity: '0.6' },
                },
                'preloader-logo': {
                    '0%': { opacity: '0', transform: 'scale(0.96)' },
                    '100%': { opacity: '1', transform: 'scale(1)' },
                },
                'preloader-bar': {
                    '0%': { transform: 'translateX(-60%)' },
                    '100%': { transform: 'translateX(260%)' },
                },
                'preloader-fill': {
                    '0%': { transform: 'scaleX(0)' },
                    '100%': { transform: 'scaleX(1)' },
                },
                fall: {
                    '0%': { transform: 'translateY(-10%) rotate(0deg)', opacity: '0' },
                    '10%': { opacity: '0.8' },
                    '100%': { transform: 'translateY(100vh) rotate(360deg)', opacity: '0' },
                },
                'scroll-left': {
                    '0%': { transform: 'translateX(0)' },
                    '100%': { transform: 'translateX(-50%)' },
                },
                'scroll-right': {
                    '0%': { transform: 'translateX(-50%)' },
                    '100%': { transform: 'translateX(0)' },
                },
                'spin-rotate': {
                    '0%': { transform: 'rotate(0deg)' },
                    '100%': { transform: 'rotate(360deg)' },
                },
            },
            animation: {
                fadeOpacity: 'fadeOpacity 8s infinite ease-in-out',
                'preloader-logo': 'preloader-logo 900ms cubic-bezier(0.22, 1, 0.36, 1) forwards',
                'preloader-bar': 'preloader-bar 1.6s cubic-bezier(0.4, 0, 0.2, 1) infinite',
                'preloader-fill': 'preloader-fill 1.8s cubic-bezier(0.4, 0, 0.2, 1) forwards',
                fall: 'fall 3s linear infinite',
                'spin-slow': 'spin-rotate 12s linear infinite',
                'scroll-left': 'scroll-left 40s linear infinite',
                'scroll-right': 'scroll-right 40s linear infinite',
            },
        },
    },
    variants: {
        extend: {},
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
