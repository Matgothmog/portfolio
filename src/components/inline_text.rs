use leptos::prelude::*;

use crate::data::{Segment, parse_inline};

/// Text whose `backtick spans` render as `<code>` elements.
#[component]
pub fn InlineText(#[prop(into)] text: String) -> impl IntoView {
    parse_inline(&text)
        .into_iter()
        .map(|segment| match segment {
            Segment::Text(plain) => plain.into_any(),
            Segment::Code(code) => view! { <code>{code}</code> }.into_any(),
        })
        .collect_view()
}
