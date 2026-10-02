use leptos::prelude::*;

use super::Section;

#[component]
pub fn BugBountySection() -> impl IntoView {
    view! {
        <Section
            id="bug-bounty"
            title="Ethereum Foundation — Bug Bounty"
            intro=|| "Findings are under coordinated disclosure / NDA. Details will be added once cleared."
            alt=true
        >
            <div class="project-body">
                <p>
                    "One high-severity and three medium-severity findings are currently being examined by the Ethereum Foundation."
                </p>
            </div>
        </Section>
    }
}
