use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    DocsRoot,
    #[at("/docs/:section/:page")]
    Doc { section: String, page: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
