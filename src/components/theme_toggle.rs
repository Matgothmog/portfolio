use leptos::prelude::*;

use crate::app::use_theme;
use crate::theme::{Theme, apply_theme};

const SUN_PATH: &str = "M12 4V2m0 20v-2m8-8h2M2 12h2m13.66-5.66 1.41-1.41M4.93 19.07l1.41-1.41m0-11.32L4.93 4.93m14.14 14.14-1.41-1.41M12 7a5 5 0 1 0 0 10 5 5 0 0 0 0-10Z";
const MOON_PATH: &str = "M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8Z";

/// Switches between the light and dark theme; `aria-pressed` is true while dark is on.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_theme();
    let is_dark = move || theme.get() == Theme::Dark;
    let toggle = move |_| {
        let next = theme.get_untracked().toggled();
        apply_theme(next);
        theme.set(next);
    };

    view! {
        <button
            type="button"
            class="theme-toggle"
            aria-label="Dark theme"
            aria-pressed=move || is_dark().to_string()
            on:click=toggle
        >
            <svg
                viewBox="0 0 24 24"
                width="18"
                height="18"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
            >
                <path d=move || if is_dark() { SUN_PATH } else { MOON_PATH } />
            </svg>
        </button>
    }
}
