use leptos::prelude::*;

/// Visible stand-in for a PR list that failed to load.
#[component]
pub fn PrsError(#[prop(into)] message: String) -> impl IntoView {
    view! {
        <p class="load-error" role="alert">
            "The PR list could not be loaded: " {message}
        </p>
    }
}
