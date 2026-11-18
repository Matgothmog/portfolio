use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::app_href;
use crate::components::{
    BugBountySection, EnsimagSection, Footer, KinoflexSection, NethermindSection, PostageSection,
    SplitAboutSection, SplitExperienceSection, ThemeToggle, scroll_to_hash_after_mount,
};
use crate::data;

const TITLE: &str = "Matteo Pecher (Matgothmog) — Student Software & Security Engineer";
const GITHUB_URL: &str = "https://github.com/Matgothmog";
const EMAIL: &str = "matteo.pecher@grenoble-inp.fr";
const MAILTO_URL: &str = "mailto:matteo.pecher@grenoble-inp.fr";

const RAIL_LINKS: [(&str, &str, &str); 7] = [
    ("01", "About & Skills", "/#about"),
    ("02", "Experience", "/#experience"),
    ("03", "Nethermind PRs", "/#nethermind"),
    ("04", "Postage", "/#postage"),
    ("05", "Ensimag Pentest", "/#ensimag-pentest"),
    ("06", "Inria Kinoflex", "/#inria-kinoflex"),
    ("07", "Bug Bounty", "/#bug-bounty"),
];

/// Split-rail portfolio layout: sticky identity & navigation on the left,
/// experience and interactive engineering evidence on the right.
#[component]
pub fn HomePage() -> impl IntoView {
    super::set_document_title(TITLE);
    scroll_to_hash_after_mount();
    let prs = data::embedded_prs().map_err(|error| error.to_string());

    let nav_items = RAIL_LINKS
        .iter()
        .map(|(num, label, href)| {
            view! {
                <A href=app_href(href) attr:class="rail-nav-link">
                    <span class="rail-nav-line" aria-hidden="true"></span>
                    <span class="rail-nav-num">{*num}</span>
                    <span>{*label}</span>
                </A>
            }
        })
        .collect_view();

    view! {
        <div class="split-shell">
            <aside class="split-rail">
                <div class="rail-topbar">
                    <A href=app_href("/#about") attr:class="brand">
                        "M"
                        <span class="brand-dot">"."</span>
                    </A>
                    <div class="rail-controls">
                        <ThemeToggle />
                    </div>
                </div>

                <div class="rail-identity">
                    <div class="rail-profile">
                        <img
                            class="rail-avatar"
                            src="assets/img/avatar.jpg"
                            alt="Matteo Pecher"
                            width="68"
                            height="68"
                        />
                        <div>
                            <h1 class="rail-name">"Matteo Pecher"</h1>
                            <p class="rail-handle">"@Matgothmog · Grenoble, FR"</p>
                        </div>
                    </div>
                    <p class="rail-role">
                        "Final-year engineering student in Cybersecurity & Blockchain Security"
                    </p>
                    <p class="rail-bio">
                        "Ethereum Foundation bug-bounty researcher & Nethermind execution-client contributor. From low-level C/Rust & assembly to protocol security."
                    </p>
                </div>

                <nav class="rail-nav" aria-label="Section navigation">
                    {nav_items}
                </nav>

                <div class="rail-footer">
                    <div class="rail-actions">
                        <a
                            class="btn btn-primary"
                            href=GITHUB_URL
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "GitHub"
                        </a>
                        <a class="btn btn-outline" href=MAILTO_URL>
                            {EMAIL}
                        </a>
                    </div>
                    <p class="rail-meta">
                        <span class="status-dot" aria-hidden="true"></span>
                        <span>"ENSIMAG M2 Cysec · TOEFL 101"</span>
                    </p>
                </div>
            </aside>

            <main class="split-content" id="main" tabindex="-1">
                <SplitAboutSection />
                <SplitExperienceSection />
                <NethermindSection prs=prs />
                <PostageSection />
                <EnsimagSection />
                <KinoflexSection />
                <BugBountySection />
                <Footer />
            </main>
        </div>
    }
}
