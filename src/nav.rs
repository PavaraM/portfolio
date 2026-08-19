use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, IntersectionObserver, IntersectionObserverInit};

const SECTIONS: &[(&str, &str)] = &[
    ("projects", "Projects"),
    ("about", "About"),
    ("contact", "Contact"),
];

#[component]
pub fn Nav() -> impl IntoView {
    let (scrolled, set_scrolled) = signal(false);
    let (active_section, set_active_section) = signal(String::from("projects"));
    let (is_dark, set_is_dark) = signal(initial_theme());

    let toggle_theme = move |_| {
        let next = if is_dark.get() { "light" } else { "dark" };
        set_is_dark.set(next == "dark");
        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if let Some(root) = doc.document_element() {
                    root.set_attribute("data-theme", next).ok();
                }
            }
            if let Ok(Some(storage)) = win.local_storage() {
                storage.set_item("pv-theme", next).ok();
            }
        }
    };

    // Sync theme
    let _ = Effect::watch(
        move || is_dark.get(),
        move |dark, _, _| {
            let theme = if *dark { "dark" } else { "light" };
            if let Some(win) = window() {
                if let Some(doc) = win.document() {
                    if let Some(root) = doc.document_element() {
                        root.set_attribute("data-theme", theme).ok();
                    }
                }
            }
        },
        false,
    );

    // Scroll listener
    let scroll_closure = Closure::wrap(Box::new(move || {
        let y = window().map(|w| w.scroll_y().unwrap_or(0.0)).unwrap_or(0.0);
        set_scrolled.set(y > 4.0);
    }) as Box<dyn Fn()>);

    if let Some(win) = window() {
        win.add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref())
            .ok();
        scroll_closure.forget();
        set_scrolled.set(win.scroll_y().unwrap_or(0.0) > 4.0);
    }

    // Section spy
    let observer_closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
        for entry in entries.iter() {
            if let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                if entry.is_intersecting() {
                    if let Ok(el) = entry.target().dyn_into::<Element>() {
                        if let Some(id) = el.get_attribute("id") {
                            set_active_section.set(id);
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array)>);

    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            let opts = IntersectionObserverInit::new();
            opts.set_threshold(&0.3.into());
            if let Ok(observer) =
                IntersectionObserver::new_with_options(observer_closure.as_ref().unchecked_ref(), &opts)
            {
                for &(section_id, _) in SECTIONS {
                    if let Some(el) = doc.get_element_by_id(section_id) {
                        observer.observe(&el);
                    }
                }
                std::mem::forget(observer);
            }
        }
    }
    observer_closure.forget();

    view! {
        <nav id="nav" class:scrolled=move || scrolled.get()>
            <div class="container nav-inner">
                <a class="nav-logo" href="#">"pavara"<span>"."</span>"dev"</a>
                <div class="nav-right">
                    <ul class="nav-links">
                        {SECTIONS.iter().map(|&(id, label)| {
                            let section = id.to_string();
                            view! {
                                <li>
                                    <a
                                        href=format!("#{}", id)
                                        class:active=move || active_section.get() == section
                                    >
                                        {label}
                                    </a>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                    <button
                        id="theme-toggle"
                        aria-label="Toggle dark mode"
                        on:click=toggle_theme
                    >
                        <svg class="icon-moon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
                        </svg>
                        <svg class="icon-sun" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="4"/>
                            <path d="M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32 1.41 1.41M2 12h2m16 0h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/>
                        </svg>
                    </button>
                </div>
            </div>
        </nav>
    }
}

fn initial_theme() -> bool {
    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(t)) = storage.get_item("pv-theme") {
                return t == "dark";
            }
        }
        if let Ok(Some(mql)) = win.match_media("(prefers-color-scheme: dark)") {
            return mql.matches();
        }
    }
    false
}
