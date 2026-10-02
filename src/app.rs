//! Root component: theme context, router and the routes.

use std::borrow::Cow;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::NotFound;
use crate::pages::{AllPrsPage, HomePage};
use crate::theme::{Theme, initial_theme};

#[component]
pub fn App() -> impl IntoView {
    provide_context(RwSignal::new(initial_theme()));
    let base = router_base(document_base_href().as_deref());
    provide_context(BasePath(base.clone()));
    let router_base: Cow<'static, str> = Cow::Owned(base);

    view! {
        <Router base=router_base>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/prs") view=AllPrsPage />
                // The pre-Leptos site served these files; old bookmarks still point at them.
                <Route path=path!("/index.html") view=HomePage />
                <Route path=path!("/prs.html") view=AllPrsPage />
            </Routes>
        </Router>
    }
}

/// The shared theme signal provided by [`App`].
pub fn use_theme() -> RwSignal<Theme> {
    expect_context::<RwSignal<Theme>>()
}

/// The path prefix the app is served under, e.g. `/portfolio`, or empty at the root.
#[derive(Debug, Clone)]
struct BasePath(String);

/// An in-app link target. The router matches routes under its base but leaves `href`
/// values untouched, so every link has to carry the prefix itself.
pub fn app_href(path: &str) -> String {
    let base = use_context::<BasePath>()
        .map(|base| base.0)
        .unwrap_or_default();
    format!("{base}{path}")
}

/// Turns the `<base href>` Trunk writes (`/`, `/portfolio/` or a full URL) into the
/// router base: no trailing slash, empty when served from the root.
pub fn router_base(base_href: Option<&str>) -> String {
    let Some(href) = base_href else {
        return String::new();
    };
    let path = match href.split_once("://") {
        Some((_, rest)) => rest.find('/').map_or("", |slash| &rest[slash..]),
        None => href,
    };
    path.trim_end_matches('/').to_string()
}

fn document_base_href() -> Option<String> {
    let document = web_sys::window()?.document()?;
    let base = document.query_selector("base").ok().flatten()?;
    base.get_attribute("href")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_public_url_gives_empty_base() {
        assert_eq!(router_base(Some("/")), "");
    }

    #[test]
    fn subpath_public_url_drops_the_trailing_slash() {
        assert_eq!(router_base(Some("/portfolio/")), "/portfolio");
    }

    #[test]
    fn full_url_keeps_only_its_path() {
        assert_eq!(
            router_base(Some("https://example.com/portfolio/")),
            "/portfolio"
        );
        assert_eq!(router_base(Some("https://example.com/")), "");
        assert_eq!(router_base(Some("https://example.com")), "");
    }

    #[test]
    fn missing_base_element_gives_empty_base() {
        assert_eq!(router_base(None), "");
    }
}
