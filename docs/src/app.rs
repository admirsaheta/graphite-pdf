use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::doc_layout::DocLayout;
use crate::components::navbar::Navbar;
use crate::pages::doc_page::DocPage;
use crate::pages::home::HomePage;
use crate::router::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },

        Route::DocsRoot => html! {
            <Redirect<Route> to={Route::Doc {
                section: "getting-started".into(),
                page: "introduction".into(),
            }} />
        },

        Route::Doc { section, page } => html! {
            <DocLayout section={section.clone()} page={page.clone()}>
                <DocPage section={section} page={page} />
            </DocLayout>
        },

        Route::NotFound => html! {
            <div class="flex min-h-[80vh] flex-col items-center justify-center gap-4">
                <p class="text-6xl font-bold text-[#3A3A38]">{"404"}</p>
                <p class="text-[#6E6E6C]">{"Page not found."}</p>
                <Link<Route>
                    to={Route::Home}
                    classes="text-sm text-[#D4581A] hover:text-[#F58040] transition-colors"
                >
                    {"← Back to home"}
                </Link<Route>>
            </div>
        },
    }
}

#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-[#111110]">
                <Navbar />
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}
