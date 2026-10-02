use leptos::prelude::*;

use super::InlineText;
use crate::data::Pr;

/// One merged PR. `heading_level` is 2 on the all-PRs page (its title is the h1)
/// and 3 inside the home page sections.
#[component]
pub fn PrCard(pr: Pr, heading_level: u8) -> impl IntoView {
    let title = view! { <InlineText text=pr.title.clone() /> };
    let heading = match heading_level {
        2 => view! { <h2>{title}</h2> }.into_any(),
        _ => view! { <h3>{title}</h3> }.into_any(),
    };

    view! {
        <article class="card pr-card">
            <div class="card-top">
                <a class="badge" href=pr.url.clone() target="_blank" rel="noopener noreferrer">
                    {format!("#{}", pr.number)}
                </a>
                <span class="tag">
                    <InlineText text=pr.tag.clone() />
                </span>
            </div>
            {heading}
            <p class="card-meta">
                <span class="diff">
                    <span class="add">{format!("+{}", pr.diff.add)}</span>
                    " "
                    <span class="del">{format!("-{}", pr.diff.del)}</span>
                </span>
                <span class="sep">"·"</span>
                <span>{format!("Merged {}", pr.merged.display_long())}</span>
            </p>
            <p class="card-body">
                <InlineText text=pr.description.clone() />
            </p>
        </article>
    }
}
