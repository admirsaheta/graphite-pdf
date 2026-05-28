use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[component]
pub fn HomePage() -> Html {
    html! {
        <main class="relative overflow-hidden">
            // Ambient glow
            <div class="pointer-events-none absolute -top-40 left-1/2 -translate-x-1/2 h-[600px] w-[900px] rounded-full opacity-[0.06]"
                style="background: radial-gradient(circle at 50% 50%, #F58040, transparent 70%);" />

            // Hero
            <section class="relative mx-auto flex min-h-[calc(100vh-56px)] max-w-screen-xl flex-col items-center justify-center px-6 py-24 text-center">

                // Status pill
                <div class="mb-8 inline-flex items-center gap-2 rounded-full border border-[#3A3A38] bg-[#1E1E1C] px-4 py-1.5">
                    <span class="h-1.5 w-1.5 rounded-full bg-[#D4581A] animate-[rust-pulse_3s_ease-in-out_infinite]" />
                    <span class="text-[12px] font-medium tracking-wide text-[#6E6E6C]">{"Alpha — actively developed"}</span>
                </div>

                // Wordmark
                <h1 class="mb-6 text-5xl sm:text-6xl md:text-7xl font-bold tracking-[-0.03em] leading-[1.05]"
                    style="font-family: 'Space Grotesk', sans-serif;">
                    <span class="text-[#C4C4C0]">{"graphite"}</span>
                    <span class="text-[#D4581A]">{"pdf"}</span>
                </h1>

                <p class="mb-4 text-lg sm:text-xl font-medium text-[#C4C4C0] tracking-tight">
                    {"PDF generation for Rust."}
                </p>
                <p class="mb-12 max-w-xl text-base leading-relaxed text-[#6E6E6C]">
                    {"A modular, Rust-native engine for layout, composition, and rendering pipelines. Build precisely. Output reliably."}
                </p>

                // CTA buttons
                <div class="flex flex-wrap items-center justify-center gap-3">
                    <Link<Route>
                        to={Route::Doc { section: "getting-started".into(), page: "introduction".into() }}
                        classes="inline-flex items-center gap-2 rounded-lg bg-[#D4581A] px-5 py-2.5 text-sm font-semibold text-white shadow-lg shadow-[#D4581A]/20 transition-all hover:bg-[#F58040] hover:shadow-[#F58040]/25 hover:scale-[1.02]"
                    >
                        {"Get started"}
                        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                        </svg>
                    </Link<Route>>
                    <a
                        href="https://github.com/admirsaheta/graphitepdf"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center gap-2 rounded-lg border border-[#3A3A38] bg-[#1E1E1C] px-5 py-2.5 text-sm font-semibold text-[#C4C4C0] transition-all hover:border-[#585856] hover:bg-[#2E2E2C]"
                    >
                        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0 1 12 6.844a9.59 9.59 0 0 1 2.504.337c1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.02 10.02 0 0 0 22 12.017C22 6.484 17.522 2 12 2z" />
                        </svg>
                        {"GitHub"}
                    </a>
                </div>

                // Install snippet
                <div class="mt-14 w-full max-w-sm rounded-lg border border-[#3A3A38] bg-[#1E1E1C] p-4 text-left shadow-xl">
                    <div class="mb-2.5 flex items-center gap-2">
                        <span class="h-2.5 w-2.5 rounded-full bg-[#3A3A38]" />
                        <span class="h-2.5 w-2.5 rounded-full bg-[#3A3A38]" />
                        <span class="h-2.5 w-2.5 rounded-full bg-[#3A3A38]" />
                        <span class="ml-2 text-[11px] text-[#585856] font-mono">{"Cargo.toml"}</span>
                    </div>
                    <pre class="overflow-x-auto text-[13px] font-mono leading-relaxed m-0 bg-transparent">
                        <span class="text-[#585856]">{"[dependencies]"}</span>
                        {"\n"}
                        <span class="text-[#C4C4C0]">{"graphitepdf"}</span>
                        <span class="text-[#6E6E6C]">{" = "}</span>
                        <span class="text-[#D4581A]">{r#""0.1""#}</span>
                    </pre>
                </div>
            </section>

            // Feature grid
            <section class="mx-auto max-w-screen-xl px-6 pb-32">
                <div class="mb-12 text-center">
                    <h2 class="text-2xl font-semibold text-[#C4C4C0] tracking-tight">{"Built for the Rust ecosystem"}</h2>
                    <p class="mt-3 text-[#6E6E6C]">{"Modular crates. Zero-C-wrapper. Production-ready."}</p>
                </div>

                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                    { for FEATURES.iter().map(|(icon, title, desc)| html! {
                        <div class="rounded-xl border border-[#2E2E2C] bg-[#1A1A18] p-5 hover:border-[#3A3A38] transition-colors group">
                            <div class="mb-3 flex h-9 w-9 items-center justify-center rounded-lg bg-[#2E2E2C] text-[#D4581A] text-lg group-hover:bg-[#3A3A38] transition-colors">
                                { icon }
                            </div>
                            <h3 class="mb-1.5 text-sm font-semibold text-[#C4C4C0]">{ title }</h3>
                            <p class="text-[13px] leading-relaxed text-[#585856]">{ desc }</p>
                        </div>
                    })}
                </div>
            </section>

            // Crate list
            <section class="border-t border-[#1E1E1C] bg-[#0D0D0C] px-6 py-20">
                <div class="mx-auto max-w-screen-xl">
                    <h2 class="mb-2 text-xl font-semibold text-[#C4C4C0]">{"16 focused crates"}</h2>
                    <p class="mb-8 text-sm text-[#6E6E6C]">{"Take what you need, or use the facade for everything."}</p>
                    <div class="flex flex-wrap gap-2">
                        { for CRATES.iter().map(|name| html! {
                            <Link<Route>
                                to={Route::Doc { section: "crates".into(), page: name.to_string() }}
                                classes="inline-flex items-center gap-1.5 rounded-md border border-[#2E2E2C] bg-[#1A1A18] px-3 py-1.5 text-[13px] font-mono text-[#6E6E6C] hover:border-[#D4581A]/40 hover:text-[#D4581A] transition-all"
                            >
                                <span class="text-[#3A3A38]">{"graphitepdf-"}</span>
                                { name }
                            </Link<Route>>
                        })}
                    </div>
                </div>
            </section>
        </main>
    }
}

const FEATURES: &[(&str, &str, &str)] = &[
    (
        "⬡",
        "Pure Rust",
        "No C bindings. No unsafe FFI. Built ground-up for safety and portability.",
    ),
    (
        "◈",
        "Modular crates",
        "Use only what you need. Each layer of the stack is independently versioned.",
    ),
    (
        "◎",
        "Layout engine",
        "Box model layout with flex-like flow, precise block/inline composition.",
    ),
    (
        "◇",
        "Text pipeline",
        "Font loading, shaping, glyph metrics, and full text layout via textkit.",
    ),
    (
        "▣",
        "Style system",
        "CSS-influenced cascade with strict specificity and stylesheet resolution.",
    ),
    (
        "◐",
        "Render pipeline",
        "Page rendering and PDF assembly with zero-copy output where possible.",
    ),
];

const CRATES: &[&str] = &[
    "document",
    "errors",
    "font",
    "image",
    "kit",
    "layout",
    "math",
    "primitives",
    "render",
    "renderer",
    "style",
    "stylesheet",
    "svg",
    "textkit",
    "utils",
];
