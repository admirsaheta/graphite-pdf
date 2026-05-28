use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

#[component]
pub fn About() -> Html {
    let section_ref = use_node_ref();
    let visible = use_state(|| false);
    let observer_handle = use_mut_ref(|| None::<IntersectionObserver>);
    let has_revealed = use_mut_ref(|| false);

    {
        let section_ref = section_ref.clone();
        let visible = visible.clone();
        let observer_handle = observer_handle.clone();
        let has_revealed = has_revealed.clone();
        use_effect_with((), move |_| {
            if let Some(window) = window() {
                if window.document().is_some() {
                    if let Some(element) = section_ref.cast::<web_sys::Element>() {
                        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, observer: IntersectionObserver| {
                            for entry in entries.iter() {
                                let entry: IntersectionObserverEntry = entry.unchecked_into();
                                if entry.is_intersecting() {
                                    if !*has_revealed.borrow() {
                                        *has_revealed.borrow_mut() = true;
                                        visible.set(true);
                                    }
                                    observer.unobserve(&entry.target());
                                }
                            }
                        }) as Box<dyn FnMut(_, _)>);

                        let options = IntersectionObserverInit::new();
                        options.set_threshold(&JsValue::from_f64(0.2));

                        if let Ok(observer) = IntersectionObserver::new_with_options(
                            callback.as_ref().unchecked_ref(),
                            &options,
                        ) {
                            observer.observe(&element);
                            *observer_handle.borrow_mut() = Some(observer);
                        }

                        callback.forget();
                    }
                }
            }
            move || {
                if let Some(observer) = observer_handle.borrow_mut().take() {
                    observer.disconnect();
                }
            }
        });
    }

    let dot_classes = classes!(
        "mx-auto",
        "h-2",
        "w-2",
        "rounded-full",
        "bg-white/60",
        "transition-all",
        "duration-[700ms]",
        "ease-[cubic-bezier(0.16,1,0.3,1)]",
        if *visible {
            "scale-100 opacity-100"
        } else {
            "scale-0 opacity-0"
        }
    );

    let line_classes = classes!(
        "mx-auto",
        "mt-3",
        "h-20",
        "w-[2px]",
        "rounded-full",
        "bg-white/25",
        "origin-top",
        "transition-all",
        "duration-[1200ms]",
        "ease-[cubic-bezier(0.16,1,0.3,1)]",
        if *visible {
            "scale-y-100 opacity-100"
        } else {
            "scale-y-0 opacity-0"
        }
    );

    let content_classes = classes!(
        "mt-6",
        "text-center",
        "transition-all",
        "duration-[1200ms]",
        "ease-[cubic-bezier(0.16,1,0.3,1)]",
        if *visible {
            "opacity-100 translate-y-0"
        } else {
            "opacity-0 translate-y-4"
        }
    );

    html! {
        <section id="about-section" ref={section_ref} class="relative w-full bg-black pb-24 sm:pb-32">
            <div class="mx-auto flex w-full max-w-4xl flex-col items-center px-6">
                <div class={dot_classes} />
                <div class={line_classes} />
                <div class={content_classes}>
                    <p class="text-xs sm:text-sm uppercase tracking-[0.42em] text-white/65">
                        { "BIOGRAPHY" }
                    </p>
                    <h2 class="mt-6 text-2xl sm:text-3xl md:text-4xl font-admir-bold tracking-[0.06em] text-white">
                        { "Trusted by startups and tech giants with " }
                        <span class="text-[#C3976F]">{ "$40B+ " }</span>
                        { "in funding" }
                    </h2>
                    <p class="mt-6 max-w-4xl text-sm sm:text-base leading-relaxed text-white/70">
                        { "Software engineer with 8+ years of experience, including at Shopify, building products used by millions. I have collaborated with ambitious startups and Forbes-accredited companies across the US, developing robust, scalable solutions for web and mobile applications in both B2B and B2C. I focus on clean, efficient code and precise execution, and I thrive on challenges that push the boundaries of what is possible in software." }
                    </p>
                </div>
            </div>
        </section>
    }
}