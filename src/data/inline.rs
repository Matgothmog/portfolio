//! Splits PR prose into plain text and `backtick code` spans.
//!
//! The UI renders each segment as a text node or a `<code>` element, so Leptos
//! escapes the content and no HTML string is ever built here.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    Text(String),
    Code(String),
}

/// Pairs backticks left to right; an unpaired or empty span (`` `` ``) stays literal text.
pub fn parse_inline(text: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut plain = String::new();
    let mut rest = text;

    while let Some(open) = rest.find('`') {
        let after_open = &rest[open + 1..];
        let code_end = after_open.find('`').filter(|&end| end > 0);
        let Some(end) = code_end else {
            // Not a code span: keep the backtick and keep scanning after it.
            plain.push_str(&rest[..=open]);
            rest = after_open;
            continue;
        };
        plain.push_str(&rest[..open]);
        if !plain.is_empty() {
            segments.push(Segment::Text(std::mem::take(&mut plain)));
        }
        segments.push(Segment::Code(after_open[..end].to_string()));
        rest = &after_open[end + 1..];
    }

    plain.push_str(rest);
    if !plain.is_empty() {
        segments.push(Segment::Text(plain));
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &str) -> Segment {
        Segment::Text(value.to_string())
    }

    fn code(value: &str) -> Segment {
        Segment::Code(value.to_string())
    }

    #[test]
    fn plain_text_is_one_segment() {
        assert_eq!(parse_inline("just words"), vec![text("just words")]);
    }

    #[test]
    fn empty_input_gives_no_segments() {
        assert!(parse_inline("").is_empty());
    }

    #[test]
    fn converts_backtick_span_to_code_segment() {
        assert_eq!(
            parse_inline("call `foo()` now"),
            vec![text("call "), code("foo()"), text(" now")]
        );
    }

    #[test]
    fn keeps_special_characters_untouched_for_the_view_to_escape() {
        assert_eq!(
            parse_inline("`A & B` < C"),
            vec![code("A & B"), text(" < C")]
        );
    }

    #[test]
    fn handles_multiple_code_spans() {
        assert_eq!(
            parse_inline("`a` and `b`"),
            vec![code("a"), text(" and "), code("b")]
        );
    }

    #[test]
    fn span_at_the_very_start_and_end_has_no_empty_text() {
        assert_eq!(parse_inline("`only`"), vec![code("only")]);
    }

    #[test]
    fn unpaired_backtick_stays_literal() {
        assert_eq!(parse_inline("a ` b"), vec![text("a ` b")]);
    }

    #[test]
    fn empty_backtick_pair_stays_literal() {
        assert_eq!(parse_inline("a `` b"), vec![text("a `` b")]);
    }
}
