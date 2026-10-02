use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::app_href;

use crate::components::{PageShell, PrCard, PrsError};
use crate::data::{self, Pr};

const TITLE: &str = "Merged PRs — Matgothmog";
const DESCRIPTION: &str =
    "Every merged pull request by Matgothmog to NethermindEth/nethermind, newest first.";
const NETHERMIND_REPO_URL: &str = "https://github.com/NethermindEth/nethermind";

#[component]
pub fn AllPrsPage() -> impl IntoView {
    super::set_document_title(TITLE);
    use_meta_description(DESCRIPTION);
    let prs = data::embedded_prs().map_err(|error| error.to_string());
    let body = match prs {
        Ok(prs) => view! { <PrGrid prs=prs /> }.into_any(),
        Err(message) => view! { <PrsError message=message /> }.into_any(),
    };

    view! {
        <PageShell nav=back_nav>
            <section class="section" id="all-prs">
                <div class="wrap">
                    <header class="section-head">
                        <h1>"All merged PRs"</h1>
                        <p class="section-intro">
                            "Every merged contribution to "
                            <a href=NETHERMIND_REPO_URL target="_blank" rel="noopener noreferrer">
                                "NethermindEth/nethermind"
                            </a>
                            ", newest first. "
                            <A href=app_href("/#nethermind")>"Back to the portfolio"</A>
                            "."
                        </p>
                    </header>
                    {body}
                </div>
            </section>
        </PageShell>
    }
}

/// Sets the meta description for as long as the page is mounted, then puts back the one
/// that was there (the home description from `index.html`).
fn use_meta_description(description: &str) {
    let Some(meta) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| {
            document
                .query_selector(r#"meta[name="description"]"#)
                .ok()
                .flatten()
        })
    else {
        return;
    };
    let previous = meta.get_attribute("content");
    // Setting an attribute on a live element cannot fail for a valid name.
    let _ = meta.set_attribute("content", description);
    on_cleanup(move || {
        if let Some(previous) = previous {
            let _ = meta.set_attribute("content", &previous);
        }
    });
}

fn back_nav() -> impl IntoView {
    view! { <A href=app_href("/#nethermind")>"← Back to portfolio"</A> }
}

#[component]
fn PrGrid(prs: Vec<Pr>) -> impl IntoView {
    let cards = data::sorted_newest_first(&prs)
        .into_iter()
        .map(|pr| view! { <PrCard pr=pr heading_level=2 /> })
        .collect_view();

    view! { <div class="card-grid">{cards}</div> }
}
