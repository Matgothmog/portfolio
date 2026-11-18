use leptos::prelude::*;

use super::Section;

const LEADERBOARD_URL: &str = "https://ethereum.org/en/bug-bounty/#leaderboard";

#[component]
pub fn BugBountySection() -> impl IntoView {
    view! {
        <Section
            id="bug-bounty"
            eyebrow="05 // Security Research"
            title="Ethereum Foundation — Bug Bounty"
            intro=|| "Findings are under coordinated disclosure / NDA. Details will be added once cleared."
        >
            <div class="bento-credential">
                <div class="credential-main">
                    <span class="credential-badge">"Coordinated Disclosure · Active"</span>
                    <h3>"Execution-Layer Security Research"</h3>
                    <p>
                        "Recognized on the official Ethereum Foundation bug-bounty leaderboard for execution-layer vulnerability research."
                    </p>
                    <a
                        class="btn btn-outline"
                        href=LEADERBOARD_URL
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <span>"View EF Leaderboard ↗"</span>
                    </a>
                </div>
                <div class="credential-stats">
                    <div class="credential-stat">
                        <span class="metric-value">"82nd"</span>
                        <span class="metric-label">"of 99 researchers"</span>
                    </div>
                    <div class="credential-stat">
                        <span class="metric-value">"1,000"</span>
                        <span class="metric-label">"Leaderboard points"</span>
                    </div>
                    <div class="credential-stat">
                        <span class="metric-value">"EL"</span>
                        <span class="metric-label">"Execution layer scope"</span>
                    </div>
                </div>
            </div>
        </Section>
    }
}
