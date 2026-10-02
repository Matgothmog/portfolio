//! One component per route.

mod all_prs;
mod home;

pub use all_prs::AllPrsPage;
pub use home::HomePage;

/// Routes share one HTML shell, so the tab title is set per page.
fn set_document_title(title: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        document.set_title(title);
    }
}
