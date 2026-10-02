//! Light/dark theme: the pure decision logic plus the thin browser glue.
//!
//! An inline script in `index.html` applies the initial theme before the wasm loads;
//! this module reads and writes the same `data-theme` attribute and `theme` storage key.

use web_sys::{Document, Storage, Window};

const STORAGE_KEY: &str = "theme";
const ATTRIBUTE: &str = "data-theme";
const DARK_QUERY: &str = "(prefers-color-scheme: dark)";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn parse(value: &str) -> Option<Theme> {
        match value {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            _ => None,
        }
    }

    pub fn toggled(self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

/// A valid stored choice wins; otherwise follow the system preference.
pub fn resolve_initial(stored: Option<&str>, prefers_dark: bool) -> Theme {
    if let Some(theme) = stored.and_then(Theme::parse) {
        return theme;
    }
    if prefers_dark {
        Theme::Dark
    } else {
        Theme::Light
    }
}

/// A valid attribute set by the inline script wins; when it is absent or invalid
/// (the script did not run) the stored choice, then the system preference, decide.
pub fn resolve_applied(applied: Option<&str>, stored: Option<&str>, prefers_dark: bool) -> Theme {
    if let Some(theme) = applied.and_then(Theme::parse) {
        return theme;
    }
    resolve_initial(stored, prefers_dark)
}

/// The theme the page is showing now. If the inline script did not set the attribute,
/// the fallback is resolved here and written to the page (not to storage) so the
/// stylesheet and the toggle agree.
pub fn initial_theme() -> Theme {
    let root = document().and_then(|document| document.document_element());
    let applied = root.as_ref().and_then(|root| root.get_attribute(ATTRIBUTE));
    let theme = resolve_applied(
        applied.as_deref(),
        stored_theme().as_deref(),
        system_prefers_dark(),
    );
    if applied.as_deref() != Some(theme.as_str())
        && let Some(root) = root
    {
        // Setting an attribute on a live element cannot fail for a valid name.
        let _ = root.set_attribute(ATTRIBUTE, theme.as_str());
    }
    theme
}

/// Shows `theme` and remembers it. Storage failures are ignored: the choice then
/// only lasts for this visit.
pub fn apply_theme(theme: Theme) {
    if let Some(root) = document().and_then(|document| document.document_element()) {
        // Setting an attribute on a live element cannot fail for a valid name.
        let _ = root.set_attribute(ATTRIBUTE, theme.as_str());
    }
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(STORAGE_KEY, theme.as_str());
    }
}

fn stored_theme() -> Option<String> {
    local_storage()?.get_item(STORAGE_KEY).ok().flatten()
}

fn system_prefers_dark() -> bool {
    window()
        .and_then(|window| window.match_media(DARK_QUERY).ok().flatten())
        .is_some_and(|query| query.matches())
}

fn window() -> Option<Window> {
    web_sys::window()
}

fn document() -> Option<Document> {
    window()?.document()
}

/// `localStorage` access itself can throw (blocked site data), so every use goes through here.
fn local_storage() -> Option<Storage> {
    window()?.local_storage().ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_choice_beats_system_preference() {
        assert_eq!(resolve_initial(Some("light"), true), Theme::Light);
        assert_eq!(resolve_initial(Some("dark"), false), Theme::Dark);
    }

    #[test]
    fn missing_storage_follows_system_preference() {
        assert_eq!(resolve_initial(None, true), Theme::Dark);
        assert_eq!(resolve_initial(None, false), Theme::Light);
    }

    #[test]
    fn invalid_stored_value_follows_system_preference() {
        assert_eq!(resolve_initial(Some("sepia"), true), Theme::Dark);
        assert_eq!(resolve_initial(Some(""), false), Theme::Light);
    }

    #[test]
    fn applied_attribute_beats_storage_and_system_preference() {
        assert_eq!(
            resolve_applied(Some("dark"), Some("light"), false),
            Theme::Dark
        );
        assert_eq!(
            resolve_applied(Some("light"), Some("dark"), true),
            Theme::Light
        );
    }

    #[test]
    fn absent_attribute_falls_back_to_stored_then_system_then_light() {
        assert_eq!(resolve_applied(None, Some("dark"), false), Theme::Dark);
        assert_eq!(resolve_applied(None, None, true), Theme::Dark);
        assert_eq!(resolve_applied(None, None, false), Theme::Light);
    }

    #[test]
    fn invalid_attribute_is_treated_as_absent() {
        assert_eq!(
            resolve_applied(Some("sepia"), Some("dark"), false),
            Theme::Dark
        );
    }

    #[test]
    fn toggled_flips_between_the_two_themes() {
        assert_eq!(Theme::Light.toggled(), Theme::Dark);
        assert_eq!(Theme::Dark.toggled(), Theme::Light);
    }

    #[test]
    fn names_round_trip_through_parse() {
        for theme in [Theme::Light, Theme::Dark] {
            assert_eq!(Theme::parse(theme.as_str()), Some(theme));
        }
    }
}
