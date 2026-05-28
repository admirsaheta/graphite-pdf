use yew::prelude::*;

#[component]
pub fn App() -> Html {
    html! {
        <main class="min-h-screen px-6 py-16 text-slate-50">
            <section class="mx-auto flex max-w-5xl flex-col gap-8 lg:flex-row lg:items-center lg:justify-between">
                <div class="max-w-2xl">
                    <p class="mb-4 inline-flex rounded-full border border-cyan-400/30 bg-cyan-400/10 px-3 py-1 text-xs font-semibold uppercase tracking-[0.3em] text-cyan-200">
                        { "GraphitePDF Docs" }
                    </p>
                    <h1 class="mb-4 text-4xl font-black tracking-tight text-white sm:text-5xl">
                        { "Tailwind is now active in the Yew docs app." }
                    </h1>
                    <p class="max-w-xl text-base leading-7 text-slate-300 sm:text-lg">
                        { "This page is rendered by Yew, served by Trunk, and styled through Tailwind utilities loaded from the docs entrypoint." }
                    </p>
                    <div class="mt-8 flex flex-wrap gap-3">
                        <span class="rounded-full bg-white/10 px-4 py-2 text-sm font-medium text-white shadow-lg shadow-cyan-950/30 ring-1 ring-white/10">
                            { "Wrapper crate docs" }
                        </span>
                        <span class="rounded-full bg-emerald-400/15 px-4 py-2 text-sm font-medium text-emerald-200 ring-1 ring-emerald-400/25">
                            { "Yew + Trunk" }
                        </span>
                        <span class="rounded-full bg-fuchsia-400/15 px-4 py-2 text-sm font-medium text-fuchsia-200 ring-1 ring-fuchsia-400/25">
                            { "Tailwind utilities" }
                        </span>
                    </div>
                </div>

                <div class="w-full max-w-md rounded-3xl border border-white/10 bg-slate-900/70 p-6 shadow-2xl shadow-cyan-950/30 backdrop-blur">
                    <div class="mb-4 flex items-center justify-between">
                        <span class="text-sm font-semibold uppercase tracking-[0.2em] text-slate-400">
                            { "Status" }
                        </span>
                        <span class="rounded-full bg-emerald-400/15 px-3 py-1 text-xs font-bold text-emerald-300 ring-1 ring-emerald-400/30">
                            { "Working" }
                        </span>
                    </div>
                    <div class="space-y-4">
                        <div class="rounded-2xl bg-slate-950/60 p-4 ring-1 ring-white/10">
                            <p class="text-sm text-slate-400">{ "Utility class" }</p>
                            <p class="mt-1 font-mono text-sm text-cyan-300">{ "text-4xl font-black text-white" }</p>
                        </div>
                        <div class="rounded-2xl bg-slate-950/60 p-4 ring-1 ring-white/10">
                            <p class="text-sm text-slate-400">{ "Layout class" }</p>
                            <p class="mt-1 font-mono text-sm text-fuchsia-300">{ "flex max-w-5xl gap-8 lg:flex-row" }</p>
                        </div>
                        <div class="rounded-2xl bg-slate-950/60 p-4 ring-1 ring-white/10">
                            <p class="text-sm text-slate-400">{ "Effect class" }</p>
                            <p class="mt-1 font-mono text-sm text-emerald-300">{ "backdrop-blur shadow-2xl ring-1" }</p>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
