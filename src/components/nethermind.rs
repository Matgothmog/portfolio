use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::app_href;

use super::{PrCard, PrsError, Section};
use crate::data::{self, Pr};

const NETHERMIND_REPO_URL: &str = "https://github.com/NethermindEth/nethermind";

/// Featured PR cards plus the link to the full listing.
#[component]
pub fn NethermindSection(prs: Result<Vec<Pr>, String>) -> impl IntoView {
    let body = match prs {
        Ok(prs) => view! { <FeaturedPrs prs=prs /> }.into_any(),
        Err(message) => view! { <PrsError message=message /> }.into_any(),
    };

    view! {
        <Section id="nethermind" title="Open-source: Nethermind" intro=nethermind_intro>
            {body}
        </Section>
    }
}

fn nethermind_intro() -> impl IntoView {
    view! {
        "Merged contributions to "
        <a href=NETHERMIND_REPO_URL target="_blank" rel="noopener noreferrer">
            "NethermindEth/nethermind"
        </a>
        ", the Ethereum execution client."
    }
}

#[component]
fn FeaturedPrs(prs: Vec<Pr>) -> impl IntoView {
    let label = format!("{} →", data::all_prs_link_label(prs.len()));
    let cards = data::featured(&prs)
        .into_iter()
        .map(|pr| view! { <PrCard pr=pr heading_level=3 /> })
        .collect_view();

    view! {
        <div class="card-grid">{cards}</div>
        <p class="section-cta">
            <A href=app_href("/prs") attr:class="btn btn-outline">
                {label}
            </A>
        </p>
    }
}
