use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::markdown::Markdown;
use crate::components::toc::{Toc, TocEntry, extract_toc};
use crate::content::{get_static_content, github_edit_url, github_raw_url};

#[derive(Properties, PartialEq)]
pub struct DocPageProps {
    pub section: String,
    pub page: String,
}

#[derive(Clone, PartialEq)]
enum LoadState {
    Loading,
    Loaded(String),
    Error(String),
}

#[component]
pub fn DocPage(props: &DocPageProps) -> Html {
    let state = use_state(|| LoadState::Loading);
    let toc = use_state(Vec::<TocEntry>::new);

    {
        let state = state.clone();
        let toc = toc.clone();
        let section = props.section.clone();
        let page = props.page.clone();

        use_effect_with((section.clone(), page.clone()), move |_| {
            state.set(LoadState::Loading);
            toc.set(vec![]);

            if let Some(static_content) = get_static_content(&section, &page) {
                let entries = extract_toc(static_content);
                state.set(LoadState::Loaded(static_content.to_string()));
                toc.set(entries);
            } else if let Some(url) = github_raw_url(&section, &page) {
                let state = state.clone();
                let toc = toc.clone();
                spawn_local(async move {
                    match Request::get(&url).send().await {
                        Ok(resp) => match resp.text().await {
                            Ok(text) => {
                                let entries = extract_toc(&text);
                                state.set(LoadState::Loaded(text));
                                toc.set(entries);
                            }
                            Err(e) => state.set(LoadState::Error(e.to_string())),
                        },
                        Err(e) => state.set(LoadState::Error(e.to_string())),
                    }
                });
            } else {
                state.set(LoadState::Error("Page not found.".into()));
            }

            || ()
        });
    }

    let edit_url = github_edit_url(&props.section, &props.page);

    html! {
        <div class="flex min-h-full">
            // Main content
            <div class="flex-1 min-w-0 px-8 py-10 max-w-4xl">
                {
                    match (*state).clone() {
                        LoadState::Loading => html! { <DocSkeleton /> },
                        LoadState::Error(msg) => html! { <DocError message={msg} /> },
                        LoadState::Loaded(content) => html! {
                            <>
                                <Markdown content={content} />
                                <DocFooter edit_url={edit_url} />
                            </>
                        },
                    }
                }
            </div>

            // Right TOC rail
            <Toc entries={(*toc).clone()} />
        </div>
    }
}

// ── Skeleton ──────────────────────────────────────────────────────────────────

#[component]
fn DocSkeleton() -> Html {
    html! {
        <div class="animate-pulse space-y-6 pt-2">
            <div class="h-8 w-2/3 rounded-md bg-[#2E2E2C]" />
            <div class="h-px w-full bg-[#2E2E2C]" />
            <div class="space-y-3">
                <div class="h-4 w-full rounded bg-[#1E1E1C]" />
                <div class="h-4 w-5/6 rounded bg-[#1E1E1C]" />
                <div class="h-4 w-4/6 rounded bg-[#1E1E1C]" />
            </div>
            <div class="h-6 w-1/3 rounded-md bg-[#2E2E2C]" />
            <div class="space-y-3">
                <div class="h-4 w-full rounded bg-[#1E1E1C]" />
                <div class="h-4 w-3/4 rounded bg-[#1E1E1C]" />
            </div>
            <div class="h-32 w-full rounded-lg bg-[#1E1E1C]" />
        </div>
    }
}

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct DocErrorProps {
    message: String,
}

#[component]
fn DocError(props: &DocErrorProps) -> Html {
    html! {
        <div class="flex flex-col items-start gap-4 rounded-lg border border-[#3A3A38] bg-[#1A1A18] p-8">
            <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-[#2E2E2C] text-[#D4581A]">
                <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0zm-9 3.75h.008v.008H12v-.008z" />
                </svg>
            </div>
            <div>
                <p class="text-sm font-semibold text-[#C4C4C0]">{"Could not load content"}</p>
                <p class="mt-1 text-[13px] text-[#6E6E6C]">{ &props.message }</p>
            </div>
            <a
                href="https://github.com/admirsaheta/graphite-pdf"
                target="_blank"
                rel="noopener noreferrer"
                class="text-[13px] text-[#D4581A] hover:text-[#F58040] transition-colors"
            >
                {"View source on GitHub →"}
            </a>
        </div>
    }
}

// ── Footer ────────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct DocFooterProps {
    edit_url: String,
}

#[component]
fn DocFooter(props: &DocFooterProps) -> Html {
    html! {
        <footer class="mt-16 flex items-center justify-between border-t border-[#2E2E2C] pt-6">
            <div class="flex items-center gap-2">
                <img src="https://tvnzqqaaq45nrfqc.public.blob.vercel-storage.com/graphite_pdf/graphitepdf-favicon.svg" alt="" class="h-5 w-5 opacity-30" />
                <span class="text-[12px] text-[#585856]">{"GraphitePDF Docs"}</span>
            </div>
            <a
                href={props.edit_url.clone()}
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-1.5 text-[12px] text-[#585856] hover:text-[#D4581A] transition-colors"
            >
                <svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L6.832 19.82a4.5 4.5 0 0 1-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 0 1 1.13-1.897L16.863 4.487z" />
                </svg>
                {"Edit on GitHub"}
            </a>
        </footer>
    }
}
