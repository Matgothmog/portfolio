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

const HIGHLIGHTS: [Highlight; 4] = [
    Highlight {
        title: "Rust workspace at scale",
        body: "Seven library crates and eleven binaries in one Cargo workspace, with unsafe_code forbidden workspace-wide and a pinned toolchain for reproducible builds.",
    },
    Highlight {
        title: "Human-in-the-loop by design",
        body: "Fail-closed validators and approval gates keep an accountable operator in control of every campaign; the system refuses to run until its rules and scope are filled in.",
    },
    Highlight {
        title: "Append-only evidence ledger",
        body: "Every run's results are written to an integrity-checked, append-only record, backed by documented on-disk data contracts.",
    },
    Highlight {
        title: "No finding without a working PoC",
        body: "A potential issue is never treated as a real security finding until a reproducible proof-of-concept actually succeeds; unproven leads are discarded, which keeps the results trustworthy and free of false positives.",
    },
];

#[component]
pub fn EnsimagSection() -> impl IntoView {
    let specs = HIGHLIGHTS
        .iter()
        .enumerate()
        .map(|(index, highlight)| {
            let num = format!("{:02}", index + 1);
            view! {
                <article class="spec-cell">
                    <span class="spec-index">{num}</span>
                    <h3>{highlight.title}</h3>
                    <p>{highlight.body}</p>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="ensimag-pentest"
            eyebrow="03 // Security Automation"
            title="Ensimag — AI pentesting"
            intro=|| "A systems project built at Ensimag: a Rust framework that orchestrates authorized security-testing campaigns under continuous human oversight. It pairs fail-closed safety gates and an append-only evidence ledger with AI agents that plan and execute each nightly round, and surfaces everything through a live operator dashboard. The codebase is a large Rust workspace — roughly 73,000 lines across 18 crates — with around 1,800 tests and a Leptos/WebAssembly front end."
        >
            <figure class="window-frame figure-dashboard">
                <div class="window-chrome" aria-hidden="true">
                    <span class="window-dots">
                        <span></span><span></span><span></span>
                    </span>
                    <span class="window-title">"pentest-dashboard · campaign findings (demo)"</span>
                </div>
                <a href=DASHBOARD_URL target="_blank" rel="noopener noreferrer">
                    <img
                        src="assets/img/pentest-dashboard.png"
                        alt="Campaign Dashboard findings page showing severity counters, filters and a table of eight findings across demo hosts"
                        loading="lazy"
                    />
                </a>
                <figcaption class="window-caption">
                    "The live operator dashboard (public demo with synthetic data)."
                </figcaption>
            </figure>
            <div class="spec-matrix spec-matrix-2">{specs}</div>
            <div class="project-footer project-footer-inline">
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
