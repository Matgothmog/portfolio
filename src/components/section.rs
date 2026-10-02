use leptos::prelude::*;

/// A titled page section with an intro line; `alt` shades the background.
#[component]
pub fn Section(
    id: &'static str,
    title: &'static str,
    #[prop(into)] intro: ViewFn,
    #[prop(optional)] alt: bool,
    children: Children,
) -> impl IntoView {
    let class = if alt {
        "section section-alt"
    } else {
        "section"
    };

    view! {
        <section class=class id=id>
            <div class="wrap">
                <header class="section-head">
                    <h2>{title}</h2>
                    <p class="section-intro">{move || intro.run()}</p>
                </header>
                {children()}
            </div>
        </section>
    }
}
