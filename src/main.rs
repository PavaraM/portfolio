mod nav;
mod hero;
mod projects;
mod about;
mod contact;
mod footer;

use leptos::prelude::*;
use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, IntersectionObserver, IntersectionObserverInit};

use nav::Nav;
use hero::Hero;
use projects::Projects;
use about::About;
use contact::Contact;
use footer::{Footer, ScrollToTop};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let _ = setup_reveal_observer();

    view! {
        <Nav/>
        <Hero/>
        <Projects/>
        <About/>
        <Contact/>
        <Footer/>
        <ScrollToTop/>
    }
}

fn setup_reveal_observer() -> bool {
    let win = match window() {
        Some(w) => w,
        None => return false,
    };
    let doc = match win.document() {
        Some(d) => d,
        None => return false,
    };

    let prefers_reduced = win
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|mql| mql.matches())
        .unwrap_or(false);

    let reveals: Vec<Element> = doc
        .query_selector_all(".reveal")
        .ok()
        .map(|list| {
            let mut els = Vec::new();
            for i in 0..list.length() {
                if let Some(node) = list.get(i) {
                    if let Ok(el) = node.dyn_into::<Element>() {
                        els.push(el);
                    }
                }
            }
            els
        })
        .unwrap_or_default();

    if prefers_reduced || reveals.is_empty() {
        for el in &reveals {
            el.class_list().add_1("is-visible").ok();
        }
        return true;
    }

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
        for entry in entries.iter() {
            if let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                if entry.is_intersecting() {
                    if let Ok(el) = entry.target().dyn_into::<Element>() {
                        el.class_list().add_1("is-visible").ok();
                    }
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array)>);

    let opts = IntersectionObserverInit::new();
    opts.set_threshold(&0.12.into());

    if let Ok(observer) =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &opts)
    {
        for el in &reveals {
            observer.observe(el);
        }
        std::mem::forget(observer);
    }
    callback.forget();
    true
}
