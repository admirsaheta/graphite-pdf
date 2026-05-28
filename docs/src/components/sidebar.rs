use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::content::{NAV, NavSection};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub current_section: String,
    pub current_page: String,
}

#[component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    html! {
        <aside class="hidden lg:flex flex-col w-60 shrink-0 border-r border-white/[0.07] bg-[#111110] overflow-y-auto">
            <div class="px-4 py-6 space-y-6">
                { for NAV.iter().map(|section| html! {
                    <SidebarSection
                        section={section}
                        current_section={props.current_section.clone()}
                        current_page={props.current_page.clone()}
                    />
                })}
            </div>
        </aside>
    }
}

#[derive(Properties, PartialEq)]
struct SidebarSectionProps {
    section: &'static NavSection,
    current_section: String,
    current_page: String,
}

#[component]
fn SidebarSection(props: &SidebarSectionProps) -> Html {
    html! {
        <div>
            <p class="mb-2 px-2 text-[11px] font-semibold uppercase tracking-[0.1em] text-[#585856]">
                { props.section.label }
            </p>
            <ul class="space-y-0.5">
                { for props.section.items.iter().map(|item| {
                    let is_active = props.current_section == item.section && props.current_page == item.page;
                    let link_class = if is_active {
                        "sidebar-link-active flex items-center gap-2 px-2 py-1.5 rounded-md text-sm transition-colors"
                    } else {
                        "flex items-center gap-2 px-2 py-1.5 rounded-md text-sm text-[#6E6E6C] hover:text-[#C4C4C0] hover:bg-white/5 transition-colors"
                    };
                    html! {
                        <li>
                            <Link<Route>
                                to={Route::Doc { section: item.section.to_string(), page: item.page.to_string() }}
                                classes={link_class}
                            >
                                if is_active {
                                    <span class="h-1 w-1 rounded-full bg-[#D4581A] shrink-0" />
                                } else {
                                    <span class="h-1 w-1 rounded-full bg-transparent shrink-0" />
                                }
                                { item.label }
                            </Link<Route>>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
