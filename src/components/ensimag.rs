use leptos::prelude::*;

use super::Section;

struct Principle {
    title: &'static str,
    body: &'static str,
}

const PRINCIPLES: [Principle; 3] = [
    Principle {
        title: "Separation of duties",
        body: "Reconnaissance, attack ideation, and exploitation are distinct agents with non-overlapping tool permissions. An exploit can only run after an explicit human approval gate, never because a prompt asked for it.",
    },
    Principle {
        title: "Evidence integrity",
        body: "The reporting agent is not trusted. Findings are corroborated by an independently written audit trail and reproduced by a second, context-isolated agent.",
    },
    Principle {
        title: "Scope and kill-switch guard",
        body: "A guard sits beneath every command path, including the arbitrary shell escape hatch, enforcing scope and an emergency kill switch that halts all activity immediately, so contacting an out-of-scope target cannot be routed around by a cleverly ordered command.",
    },
];

#[component]
pub fn EnsimagSection() -> impl IntoView {
    let cards = PRINCIPLES
        .iter()
        .map(|principle| {
            view! {
                <article class="card">
                    <h3>{principle.title}</h3>
                    <p class="card-body">{principle.body}</p>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="ensimag-pentest"
            title="Ensimag — AI pentesting"
            intro=|| "A self-hosted, locally deployed system that orchestrates specialized AI agents over real reconnaissance and security tooling, built for Ensimag (Grenoble INP)."
        >
            <div class="card-grid">{cards}</div>
        </Section>
    }
}
