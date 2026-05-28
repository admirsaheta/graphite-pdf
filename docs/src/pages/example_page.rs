use wasm_bindgen::JsCast;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct HollerProps {
    #[allow(dead_code)]
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[component]
pub fn Holler(props: &HollerProps) -> Html {
    let container_classes = classes!(
        "fixed",
        "inset-0",
        "z-50",
        "transition-all",
        "duration-[1400ms]",
        "ease-[cubic-bezier(0.16,1,0.3,1)]",
        if props.open { "opacity-100" } else { "opacity-0 pointer-events-none" }
    );

    let panel_classes = classes!(
        "absolute",
        "top-0",
        "right-0",
        "h-full",
        "w-full",
        "md:w-[68%]",
        "bg-black/80",
        "backdrop-blur-xl",
        "border-l",
        "border-white/10",
        "transition-all",
        "duration-[1600ms]",
        "ease-[cubic-bezier(0.16,1,0.3,1)]",
        if props.open { "translate-x-0" } else { "translate-x-full" }
    );

    let form_classes = classes!(
        "mt-10",
        "grid",
        "gap-4",
        "text-left",
        "text-sm",
        "w-full",
        "text-white/80"
    );

    let on_close = props.on_close.clone();
    let backdrop_click = Callback::from(move |event: MouseEvent| {
        if let Some(target) = event.target() {
            if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                if element.id() == "holler-backdrop" {
                    on_close.emit(event);
                }
            }
        }
    });

    html! {
        <div
            id="holler-backdrop"
            class={container_classes}
            style="background-image: url('/public/burberry-texture.webp'); background-repeat: repeat; background-size: 320px;"
            onclick={backdrop_click}
        >
            <div class="absolute inset-0 bg-black/75" />
            <div class={panel_classes}>
                <div class="relative h-full overflow-y-auto px-8 py-16 sm:px-12">
                    <button
                        class="absolute right-6 top-6 text-xs uppercase tracking-[0.32em] text-white/60 transition hover:text-white"
                        onclick={props.on_close.clone()}
                    >
                        { "CLOSE" }
                    </button>
                    <div class="mx-auto flex w-full max-w-2xl flex-col items-start">
                        <p class="text-xs uppercase tracking-[0.42em] text-white/60">{ "CONTACT" }</p>
                        <h2 class="mt-4 text-3xl sm:text-4xl font-admir-bold tracking-[0.08em] text-white">
                            { "Let’s talk" }
                        </h2>
                        <p class="mt-4 text-sm sm:text-base leading-relaxed text-white/70">
                            { "Share a few details and I’ll get back within 48 hours. Let’s build something crisp, bold, and unforgettable." }
                        </p>
                        <form class={form_classes} action="https://api.web3forms.com/submit" method="POST">
                            <input type="hidden" name="access_key" value="4cf5edca-2916-4658-bac4-0cbb7f8ea94a" />
                            <div class="grid gap-2">
                                <label class="text-xs uppercase tracking-[0.32em] text-white/50">{ "Name" }</label>
                                <input class="w-full rounded-none border border-white/15 bg-black/40 px-5 py-4 text-white placeholder:text-white/30 focus:border-[#C3976F] focus:outline-none" type="text" name="name" required={true} placeholder="Your name" />
                            </div>
                            <div class="grid gap-2">
                                <label class="text-xs uppercase tracking-[0.32em] text-white/50">{ "Email" }</label>
                                <input class="w-full rounded-none border border-white/15 bg-black/40 px-5 py-4 text-white placeholder:text-white/30 focus:border-[#C3976F] focus:outline-none" type="email" name="email" required={true} placeholder="you@domain.com" />
                            </div>
                            <div class="grid gap-2">
                                <label class="text-xs uppercase tracking-[0.32em] text-white/50">{ "Message" }</label>
                                <textarea class="min-h-[180px] w-full rounded-none border border-white/15 bg-black/40 px-5 py-4 text-white placeholder:text-white/30 focus:border-[#C3976F] focus:outline-none" name="message" required={true} placeholder="Tell me about your project." />
                            </div>
                            <button type="submit" class="mt-4 w-fit rounded-full border border-[#C3976F] px-6 py-2 text-[11px] font-admir-bold tracking-[0.32em] text-white/90 transition-all duration-500 ease-out hover:border-white hover:text-white hover:scale-[1.03]">
                                { "SEND" }
                            </button>
                        </form>
                        <div class="mt-12 flex items-center gap-4">
                            <img class="h-12 w-12 invert brightness-200" src="/public/sahi2.svg" alt="Admir logo" />
                            <span class="text-xs uppercase tracking-[0.32em] text-white/50">{ "Admir Šaheta" }</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}