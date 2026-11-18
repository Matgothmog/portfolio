use leptos::prelude::*;

use super::header::GITHUB_URL;

const EMAIL: &str = "matteo.pecher@grenoble-inp.fr";
const MAILTO_URL: &str = "mailto:matteo.pecher@grenoble-inp.fr";

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="wrap footer-inner">
                <p>"Matgothmog"</p>
                <a href=GITHUB_URL target="_blank" rel="noopener noreferrer">
                    "github.com/Matgothmog"
                </a>
                <a href=MAILTO_URL>{EMAIL}</a>
                <p class="footer-note">"Built with Rust + WebAssembly (Leptos)."</p>
            </div>
        </footer>
    }
}
