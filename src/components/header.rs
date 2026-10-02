use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::app_href;

use super::ThemeToggle;

pub const GITHUB_URL: &str = "https://github.com/Matgothmog";

/// Sticky top bar. `children` are the nav links, which differ per page; the theme
/// toggle and the mobile menu button are always present.
#[component]
pub fn Header(children: Children) -> impl IntoView {
    let menu_open = RwSignal::new(false);

    view! {
        <header class="site-header">
            <div class="wrap header-inner">
                <A href=app_href("/#top") attr:class="brand">
                    "M"
                    <span class="brand-dot">"."</span>
                </A>
                <div class="header-actions">
                    <ThemeToggle />
                    <button
                        type="button"
                        class="nav-toggle"
                        aria-expanded=move || menu_open.get().to_string()
                        aria-controls="siteNav"
                        aria-label="Toggle navigation"
                        on:click=move |_| menu_open.update(|open| *open = !*open)
                    >
                        <span></span>
                        <span></span>
                        <span></span>
                    </button>
                </div>
                <nav
                    class="site-nav"
                    class:is-open=move || menu_open.get()
                    id="siteNav"
                    on:click=move |_| menu_open.set(false)
                >
                    {children()}
                    <a class="nav-cta" href=GITHUB_URL target="_blank" rel="noopener noreferrer">
                        "GitHub"
                    </a>
                </nav>
            </div>
        </header>
    }
}
