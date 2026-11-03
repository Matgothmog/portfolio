use leptos::prelude::*;

use super::Section;

const LEADERBOARD_URL: &str = "https://ethereum.org/en/bug-bounty/#leaderboard";

#[component]
pub fn BugBountySection() -> impl IntoView {
    view! {
        <Section
            id="bug-bounty"
            title="Ethereum Foundation — Bug Bounty"
            intro=|| "Findings are under coordinated disclosure / NDA. Details will be added once cleared."
        >
            <div class="project-body">
                <p>
                    "Ranked 82nd of 99 researchers on the "
                    <a href=LEADERBOARD_URL target="_blank" rel="noopener noreferrer">
                        "Ethereum Foundation bug-bounty leaderboard"
                    </a>
                    " (execution layer), with 1,000 points."
                </p>
            </div>
        </Section>
    }
}
