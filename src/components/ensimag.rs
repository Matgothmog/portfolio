use leptos::prelude::*;

use super::Section;

const DASHBOARD_URL: &str = "https://matgothmog.github.io/pentesting/";
const GITHUB_URL: &str = "https://github.com/Matgothmog/pentesting";

const STACK: [&str; 11] = [
    "Rust",
    "Tokio",
    "axum",
    "Leptos",
    "WebAssembly",
    "Trunk",
    "Serde",
    "clap",
    "GitLab CI",
    "GitHub Actions",
    "AI agents",
];

struct Highlight {
    title: &'static str,
    body: &'static str,
}

const HIGHLIGHTS: [Highlight; 9] = [
    Highlight {
        title: "Rust workspace at scale",
        body: "Seven library crates and eleven binaries in one Cargo workspace, with unsafe_code forbidden workspace-wide and a pinned toolchain for reproducible builds.",
    },
    Highlight {
        title: "Human-in-the-loop by design",
        body: "Fail-closed validators and approval gates keep an accountable operator in control of every campaign; the system refuses to run until its rules and scope are filled in.",
    },
    Highlight {
        title: "AI-driven nightly rounds",
        body: "AI agents, orchestrated through an agent runtime, plan and carry out each round of work, with their decisions recorded for review.",
    },
    Highlight {
        title: "Append-only evidence ledger",
        body: "Every run's results are written to an integrity-checked, append-only record, backed by documented on-disk data contracts.",
    },
    Highlight {
        title: "Live operator dashboard",
        body: "A Leptos/WebAssembly front end with an axum backend and a server-sent-event stream shows campaign state, findings and host coverage in real time.",
    },
    Highlight {
        title: "Tested & documented",
        body: "Around 1,800 unit and integration tests, plus runtime and dashboard data-contract docs, keep the system verifiable and maintainable.",
    },
    Highlight {
        title: "Verified, not assumed",
        body: "Validation hooks cross-check every piece of information the agents rely on before it is acted on, so the system reasons from confirmed facts instead of unverified assumptions.",
    },
    Highlight {
        title: "No finding without a working PoC",
        body: "A potential issue is never treated as a real security finding until a reproducible proof-of-concept actually succeeds; unproven leads are discarded, which keeps the results trustworthy and free of false positives.",
    },
    Highlight {
        title: "Scope refreshed daily by a human",
        body: "The authorized testing scope is reviewed and updated by a human operator every day, so the system only ever works inside an explicitly, freshly approved boundary.",
    },
];

#[component]
pub fn EnsimagSection() -> impl IntoView {
    let cards = HIGHLIGHTS
        .iter()
        .map(|highlight| {
            view! {
                <article class="card">
                    <h3>{highlight.title}</h3>
                    <p class="card-body">{highlight.body}</p>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="ensimag-pentest"
            title="Ensimag — AI pentesting"
            intro=|| "A systems project built at Ensimag: a Rust framework that orchestrates authorized security-testing campaigns under continuous human oversight. It pairs fail-closed safety gates and an append-only evidence ledger with AI agents that plan and execute each nightly round, and surfaces everything through a live operator dashboard. The codebase is a large Rust workspace — roughly 73,000 lines across 18 crates — with around 1,800 tests and a Leptos/WebAssembly front end."
        >
            <figure class="figure-contain">
                <a href=DASHBOARD_URL target="_blank" rel="noopener noreferrer">
                    <img
                        src="assets/img/pentest-dashboard.png"
                        alt="Campaign Dashboard findings page showing severity counters, filters and a table of eight findings across demo hosts"
                        loading="lazy"
                    />
                </a>
                <figcaption>"The live operator dashboard (public demo with synthetic data)."</figcaption>
            </figure>
            <div class="card-grid">{cards}</div>
            <div class="project-body">
                <EnsimagLinks />
                <ul class="stack-tags">
                    {STACK.iter().map(|tag| view! { <li>{*tag}</li> }).collect_view()}
                </ul>
            </div>
        </Section>
    }
}

#[component]
fn EnsimagLinks() -> impl IntoView {
    view! {
        <div class="project-links">
            <a class="btn btn-primary" href=DASHBOARD_URL target="_blank" rel="noopener noreferrer">
                "Live demo"
            </a>
            <a class="btn btn-outline" href=GITHUB_URL target="_blank" rel="noopener noreferrer">
                "GitHub repo"
            </a>
        </div>
    }
}
