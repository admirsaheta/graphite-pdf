use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[component]
pub fn Navbar() -> Html {
    let mobile_open = use_state(|| false);
    let toggle = {
        let mobile_open = mobile_open.clone();
        Callback::from(move |_: MouseEvent| mobile_open.set(!*mobile_open))
    };

    html! {
        <header class="sticky top-0 z-40 w-full border-b border-white/[0.07] bg-[#111110]/95 backdrop-blur-sm">
            <div class="mx-auto flex h-14 max-w-screen-2xl items-center gap-6 px-4 sm:px-6">

                // Logo
                <Link<Route> to={Route::Home} classes="flex items-center gap-2.5 shrink-0 group">
                    <img
                        src="/public/graphitepdf-mark.svg"
                        alt="GraphitePDF mark"
                        class="h-7 w-7 transition-opacity group-hover:opacity-80"
                    />
                    <span class="hidden sm:flex items-baseline gap-[2px]">
                        <span class="text-[15px] font-semibold tracking-tight text-[#C4C4C0]">{"graphite"}</span>
                        <span class="text-[15px] font-semibold tracking-tight text-[#D4581A]">{"pdf"}</span>
                    </span>
                </Link<Route>>

                // Version badge
                <span class="hidden sm:inline-flex items-center rounded-full border border-[#3A3A38] bg-[#2E2E2C] px-2.5 py-0.5 text-[11px] font-medium text-[#6E6E6C] tracking-wide">
                    {"v0.1.0-alpha"}
                </span>

                <div class="flex-1" />

                // Desktop nav links
                <nav class="hidden md:flex items-center gap-1">
                    <Link<Route>
                        to={Route::Doc { section: "getting-started".into(), page: "introduction".into() }}
                        classes="px-3 py-1.5 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors rounded-md hover:bg-white/5"
                    >
                        {"Docs"}
                    </Link<Route>>
                    <a
                        href="https://docs.rs/graphitepdf"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="px-3 py-1.5 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors rounded-md hover:bg-white/5"
                    >
                        {"API"}
                    </a>
                    <a
                        href="https://crates.io/crates/graphitepdf"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="px-3 py-1.5 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors rounded-md hover:bg-white/5"
                    >
                        {"crates.io"}
                    </a>
                </nav>

                // GitHub link
                <a
                    href="https://github.com/admirsaheta/graphitepdf"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center gap-1.5 text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors"
                    aria-label="GitHub repository"
                >
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0 1 12 6.844a9.59 9.59 0 0 1 2.504.337c1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.02 10.02 0 0 0 22 12.017C22 6.484 17.522 2 12 2z" />
                    </svg>
                </a>

                // Mobile hamburger
                <button
                    onclick={toggle}
                    class="md:hidden flex items-center justify-center h-8 w-8 text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors"
                    aria-label="Toggle menu"
                >
                    if *mobile_open {
                        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    } else {
                        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
                        </svg>
                    }
                </button>
            </div>

            // Mobile menu
            if *mobile_open {
                <div class="md:hidden border-t border-white/[0.07] bg-[#111110] px-4 pb-4 pt-2">
                    <nav class="flex flex-col gap-1">
                        <Link<Route>
                            to={Route::Doc { section: "getting-started".into(), page: "introduction".into() }}
                            classes="px-3 py-2 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] rounded-md hover:bg-white/5"
                        >
                            {"Docs"}
                        </Link<Route>>
                        <a href="https://docs.rs/graphitepdf" target="_blank" rel="noopener noreferrer"
                            class="px-3 py-2 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] rounded-md hover:bg-white/5">
                            {"API Reference"}
                        </a>
                        <a href="https://github.com/admirsaheta/graphitepdf" target="_blank" rel="noopener noreferrer"
                            class="px-3 py-2 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] rounded-md hover:bg-white/5">
                            {"GitHub"}
                        </a>
                    </nav>
                </div>
            }
        </header>
    }
}
