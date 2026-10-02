use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_router::hooks::use_location;
use web_sys::HtmlElement;

use super::{Footer, Header};

/// Skip link, header, `<main>` and footer shared by every page.
/// `nav` supplies the page-specific header links.
#[component]
pub fn PageShell(#[prop(into)] nav: ViewFn, children: Children) -> impl IntoView {
    scroll_to_hash_after_mount();

    view! {
        <a
            class="skip-link"
            href="#main"
            on:click=|event| {
                // A bare #main would resolve against <base href> and leave the current route.
                event.prevent_default();
                focus_main();
            }
        >
            "Skip to content"
        </a>
        <Header>{move || nav.run()}</Header>
        <main id="main" tabindex="-1">
            {children()}
        </main>
        <Footer />
    }
}

fn focus_main() {
    let main = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("main"))
        .and_then(|element| element.dyn_into::<HtmlElement>().ok());
    if let Some(main) = main {
        // Focus can only fail on a detached element; nothing useful to do then.
        let _ = main.focus();
    }
}

/// The router scrolls to a `#hash` before a freshly navigated page has rendered, and a
/// direct `/#section` load has no target yet either, so scroll once the page is mounted.
fn scroll_to_hash_after_mount() {
    let hash = use_location().hash;
    Effect::new(move |_| {
        let hash = hash.get();
        let id = hash.trim_start_matches('#').to_string();
        if id.is_empty() {
            return;
        }
        // The effect can run before the new page is attached to the document.
        request_animation_frame(move || {
            let target = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id(&id));
            if let Some(target) = target {
                target.scroll_into_view();
            }
        });
    });
}
