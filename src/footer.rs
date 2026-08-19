use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn Footer() -> impl IntoView {
    let year = js_sys::Date::new_0().get_full_year();
    view! {
        <footer>
            <div class="container footer-inner">
                <span>"© " {year} " Pavara Mirihagalla"</span>
                <span>"Built with Leptos · Rust"</span>
            </div>
        </footer>
    }
}

#[component]
pub fn ScrollToTop() -> impl IntoView {
    let (visible, set_visible) = signal(false);

    let closure = Closure::wrap(Box::new(move || {
        let y = window().map(|w| w.scroll_y().unwrap_or(0.0)).unwrap_or(0.0);
        set_visible.set(y > 600.0);
    }) as Box<dyn Fn()>);

    if let Some(win) = window() {
        win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
        set_visible.set(win.scroll_y().unwrap_or(0.0) > 600.0);
    }

    let scroll_to_top = move |_: web_sys::MouseEvent| {
        if let Some(win) = window() {
            let opts = web_sys::ScrollToOptions::new();
            win.scroll_to_with_scroll_to_options(&opts);
        }
    };

    view! {
        <button
            id="to-top"
            aria-label="Scroll to top"
            class:visible=move || visible.get()
            on:click=scroll_to_top
        >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
                <path d="m18 15-6-6-6 6"/>
            </svg>
        </button>
    }
}
