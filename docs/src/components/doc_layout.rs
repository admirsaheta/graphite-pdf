use crate::components::sidebar::Sidebar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DocLayoutProps {
    pub children: Children,
    pub section: String,
    pub page: String,
}

#[component]
pub fn DocLayout(props: &DocLayoutProps) -> Html {
    html! {
        <div class="flex h-[calc(100vh-56px)]">
            <Sidebar
                current_section={props.section.clone()}
                current_page={props.page.clone()}
            />
            <div class="flex-1 overflow-y-auto min-w-0">
                { for props.children.iter() }
            </div>
        </div>
    }
}
