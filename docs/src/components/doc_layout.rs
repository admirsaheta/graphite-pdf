use crate::components::sidebar::Sidebar;
use crate::content::NAV;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DocLayoutProps {
    pub children: Children,
    pub section: String,
    pub page: String,
}

#[component]
pub fn DocLayout(props: &DocLayoutProps) -> Html {
    let drawer_open = use_state(|| false);

    let open = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_: MouseEvent| drawer_open.set(true))
    };
    let close = {
        let drawer_open = drawer_open.clone();
        Callback::from(move |_: MouseEvent| drawer_open.set(false))
    };

    // Resolve human-readable breadcrumb labels from the nav tree.
    let (section_label, page_label) = {
        let mut sl = props.section.replace('-', " ");
        let mut pl = props.page.replace('-', " ");
        for nav_section in NAV.iter() {
            for item in nav_section.items.iter() {
                if item.section == props.section && item.page == props.page {
                    sl = nav_section.label.to_string();
                    pl = item.label.to_string();
                }
            }
        }
        (sl, pl)
    };

    html! {
        <div class="flex h-[calc(100vh-56px)]">

            // ── Desktop sidebar ──────────────────────────────────────────────
            <Sidebar
                current_section={props.section.clone()}
                current_page={props.page.clone()}
            />

            // ── Main content column ──────────────────────────────────────────
            <div class="flex-1 min-w-0 flex flex-col overflow-y-auto">

                // Mobile breadcrumb + drawer trigger (hidden on lg+)
                <div class="lg:hidden sticky top-0 z-30 flex items-center gap-3 h-11 shrink-0
                            border-b border-white/[0.07] bg-[#111110]/95 backdrop-blur-sm px-4">
                    <button
                        onclick={open}
                        class="flex items-center justify-center h-7 w-7 rounded-md
                               text-[#6E6E6C] hover:text-[#C4C4C0] hover:bg-white/5 transition-colors"
                        aria-label="Open navigation"
                    >
                        // Hamburger
                        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                             stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M3 12h18M3 6h18M3 18h18" />
                        </svg>
                    </button>

                    // Breadcrumb
                    <div class="flex items-center gap-1.5 text-[13px] min-w-0">
                        <span class="text-[#585856] shrink-0 truncate">{ &section_label }</span>
                        <svg class="h-3 w-3 text-[#3A3A38] shrink-0" fill="none" viewBox="0 0 24 24"
                             stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
                        </svg>
                        <span class="text-[#C4C4C0] truncate">{ &page_label }</span>
                    </div>
                </div>

                // Page children
                <div class="flex-1">
                    { for props.children.iter() }
                </div>
            </div>

            // ── Mobile drawer ────────────────────────────────────────────────
            if *drawer_open {
                <div class="lg:hidden fixed inset-0 z-50 flex">

                    // Backdrop
                    <div
                        class="absolute inset-0 bg-black/60 backdrop-blur-[2px]"
                        onclick={close.clone()}
                    />

                    // Slide-in panel
                    <div class="relative flex flex-col w-72 max-w-[85vw] bg-[#111110]
                                border-r border-white/[0.07] overflow-hidden">

                        // Panel header
                        <div class="flex items-center justify-between px-4 h-12 shrink-0
                                    border-b border-white/[0.07]">
                            <div class="flex items-center gap-2">
                                // Mini mark dot
                                <span class="h-2 w-2 rounded-full"
                                      style="background: radial-gradient(circle at 36% 30%, #f58040, #7c2806)" />
                                <span class="text-[13px] font-medium text-[#C4C4C0]">
                                    {"graphite"}
                                    <span class="text-[#D4581A]">{"pdf"}</span>
                                    {" docs"}
                                </span>
                            </div>
                            <button
                                onclick={close}
                                class="flex items-center justify-center h-7 w-7 rounded-md
                                       text-[#6E6E6C] hover:text-[#C4C4C0] hover:bg-white/5 transition-colors"
                                aria-label="Close navigation"
                            >
                                <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                                     stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                          d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>

                        // Nav tree (reuses sidebar without desktop constraints)
                        <Sidebar
                            current_section={props.section.clone()}
                            current_page={props.page.clone()}
                            mobile={true}
                        />
                    </div>
                </div>
            }
        </div>
    }
}
