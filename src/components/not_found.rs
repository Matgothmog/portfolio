use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::app_href;

use super::PageShell;

const TITLE: &str = "Page not found — Matgothmog";

/// Shown for any path that is neither `/` nor `/prs`.
#[component]
pub fn NotFound() -> impl IntoView {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        document.set_title(TITLE);
    }
    view! {
        <PageShell nav=|| ()>
            <section class="section">
                <div class="wrap">
                    <header class="section-head">
                        <h1>"Page not found"</h1>
                        <p class="section-intro">
                            <A href=app_href("/")>"Back to the portfolio"</A>
                            "."
                        </p>
                    </header>
                </div>
            </section>
        </PageShell>
    }
}
