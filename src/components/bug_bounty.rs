use leptos::prelude::*;

use super::Section;

const LEADERBOARD_URL: &str = "https://ethereum.org/bug-bounty/#leaderboard";

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
                    <h3>"Consensus & Execution Layer Research"</h3>
                    <p>
                        "Recognized on both official Ethereum Foundation bug-bounty leaderboards for protocol and client vulnerability research."
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
                        <span class="metric-value">"60th / 62"</span>
                        <span class="metric-label">"Consensus Layer · 250 pts"</span>
                    </div>
                    <div class="credential-stat">
                        <span class="metric-value">"85th / 100"</span>
                        <span class="metric-label">"Execution Layer · 1,000 pts"</span>
                    </div>
                    <div class="credential-stat">
                        <span class="metric-value">"1,250"</span>
                        <span class="metric-label">"Combined leaderboard points"</span>
                    </div>
                </div>
            </div>
        </Section>
    }
}
