use crate::router::Route;
use web_sys::Element;
use yew::prelude::*;
use yew_router::prelude::*;

const MARK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="100%" height="100%" fill="none"><defs><linearGradient id="b1" x1="18" y1="16" x2="182" y2="184" gradientUnits="userSpaceOnUse"><stop offset="0%" stop-color="#3a3a38"/><stop offset="42%" stop-color="#6e6e6c"/><stop offset="100%" stop-color="#3a3a38"/></linearGradient><linearGradient id="b2" x1="52" y1="184" x2="148" y2="16" gradientUnits="userSpaceOnUse"><stop offset="0%" stop-color="#343432"/><stop offset="48%" stop-color="#666664"/><stop offset="100%" stop-color="#343432"/></linearGradient><linearGradient id="b3" x1="10" y1="100" x2="190" y2="100" gradientUnits="userSpaceOnUse"><stop offset="0%" stop-color="#2e2e2c"/><stop offset="50%" stop-color="#606060"/><stop offset="100%" stop-color="#2e2e2c"/></linearGradient><linearGradient id="hx" x1="74" y1="62" x2="126" y2="138" gradientUnits="userSpaceOnUse"><stop offset="0%" stop-color="#6a6a68"/><stop offset="42%" stop-color="#3c3c3a"/><stop offset="100%" stop-color="#1e1e1c"/></linearGradient><radialGradient id="dt" cx="36%" cy="30%" r="68%"><stop offset="0%" stop-color="#f58040"/><stop offset="50%" stop-color="#c84c14"/><stop offset="100%" stop-color="#7c2806"/></radialGradient><filter id="sh" x="-20%" y="-20%" width="140%" height="140%"><feDropShadow dx="0" dy="1.5" stdDeviation="3.5" flood-color="#000000" flood-opacity="0.72"/></filter></defs><g transform="translate(100,100)"><path transform="rotate(240)" d="M -61 19 C -90 -15 -50 -90 61 -19 C 55 5 -50 50 -61 19 Z" fill="url(#b3)" stroke="#060604" stroke-width="0.7" filter="url(#sh)"/><path transform="rotate(120)" d="M -61 19 C -90 -15 -50 -90 61 -19 C 55 5 -50 50 -61 19 Z" fill="url(#b2)" stroke="#060604" stroke-width="0.7" filter="url(#sh)"/><path transform="rotate(0)" d="M -61 19 C -90 -15 -50 -90 61 -19 C 55 5 -50 50 -61 19 Z" fill="url(#b1)" stroke="#060604" stroke-width="0.7" filter="url(#sh)"/><path transform="rotate(0)" d="M -61 19 C -90 -15 -50 -90 61 -19" fill="none" stroke="rgba(255,255,255,0.18)" stroke-width="1"/><path transform="rotate(120)" d="M -61 19 C -90 -15 -50 -90 61 -19" fill="none" stroke="rgba(255,255,255,0.13)" stroke-width="1"/><path transform="rotate(240)" d="M -61 19 C -90 -15 -50 -90 61 -19" fill="none" stroke="rgba(255,255,255,0.09)" stroke-width="1"/><polygon points="26,0 13,-22.5 -13,-22.5 -26,0 -13,22.5 13,22.5" fill="url(#hx)" stroke="#060604" stroke-width="1"/><polyline points="26,0 13,-22.5 -13,-22.5 -26,0" fill="none" stroke="rgba(255,255,255,0.20)" stroke-width="0.85"/><polyline points="-26,0 -13,22.5 13,22.5 26,0" fill="none" stroke="rgba(0,0,0,0.45)" stroke-width="0.85"/><polygon points="17.5,0 8.75,-15.15 -8.75,-15.15 -17.5,0 -8.75,15.15 8.75,15.15" fill="none" stroke="#585856" stroke-width="0.8"/><circle cx="0" cy="0" r="11.5" fill="url(#dt)"/><ellipse cx="-3" cy="-3.5" rx="4.2" ry="2.6" fill="rgba(255,255,255,0.26)"/><ellipse cx="2.5" cy="4" rx="3.8" ry="2.2" fill="rgba(0,0,0,0.20)"/></g></svg>"##;

#[component]
fn BrandMark() -> Html {
    let el = use_node_ref();
    {
        let el = el.clone();
        use_effect_with((), move |_| {
            if let Some(node) = el.cast::<Element>() {
                node.set_inner_html(MARK_SVG);
            }
        });
    }
    html! { <span ref={el} class="flex h-7 w-7 shrink-0 items-center justify-center" /> }
}

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

                <Link<Route> to={Route::Home} classes="flex items-center gap-2.5 shrink-0 group">
                    <BrandMark />
                    <span class="hidden sm:flex items-baseline gap-[2px]">
                        <span class="text-[15px] font-semibold tracking-tight text-[#C4C4C0] group-hover:text-white transition-colors">{"graphite"}</span>
                        <span class="text-[15px] font-semibold tracking-tight text-[#D4581A] group-hover:text-[#F58040] transition-colors">{"pdf"}</span>
                    </span>
                </Link<Route>>

                <span class="hidden sm:inline-flex items-center rounded-full border border-[#3A3A38] bg-[#2E2E2C] px-2.5 py-0.5 text-[11px] font-medium text-[#6E6E6C] tracking-wide">
                    {"v0.2.0"}
                </span>

                <div class="flex-1" />

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

                <a
                    href="https://github.com/admirsaheta/graphite-pdf"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center gap-1.5 text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors"
                    aria-label="GitHub repository"
                >
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0 1 12 6.844a9.59 9.59 0 0 1 2.504.337c1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.02 10.02 0 0 0 22 12.017C22 6.484 17.522 2 12 2z" />
                    </svg>
                </a>

                <button
                    onclick={toggle}
                    class="md:hidden flex items-center justify-center h-8 w-8 text-[#6E6E6C] hover:text-[#C4C4C0] transition-colors rounded-md hover:bg-white/5"
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
                        <a href="https://github.com/admirsaheta/graphite-pdf" target="_blank" rel="noopener noreferrer"
                            class="px-3 py-2 text-sm text-[#6E6E6C] hover:text-[#C4C4C0] rounded-md hover:bg-white/5">
                            {"GitHub"}
                        </a>
                    </nav>
                </div>
            }
        </header>
    }
}
