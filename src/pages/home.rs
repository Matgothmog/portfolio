use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::app_href;

use crate::components::{
    BugBountySection, EnsimagSection, Hero, KinoflexSection, NethermindSection, PageShell,
    PostageSection,
};
use crate::data;

const TITLE: &str = "Matgothmog — Student Software & Security Engineer";

#[component]
pub fn HomePage() -> impl IntoView {
    super::set_document_title(TITLE);
    let prs = data::embedded_prs().map_err(|error| error.to_string());

    view! {
        <PageShell nav=home_nav>
            <Hero />
            <NethermindSection prs=prs />
            <PostageSection />
            <EnsimagSection />
            <KinoflexSection />
            <BugBountySection />
        </PageShell>
    }
}

fn home_nav() -> impl IntoView {
    view! {
        <A href=app_href("/#nethermind")>"Nethermind"</A>
        <A href=app_href("/#postage")>"Postage"</A>
        <A href=app_href("/#ensimag-pentest")>"Ensimag"</A>
        <A href=app_href("/#inria-kinoflex")>"Kinoflex"</A>
        <A href=app_href("/#bug-bounty")>"Bug Bounty"</A>
    }
}
