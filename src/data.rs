//! PR data: typed entries, validation, and the orderings the pages need.
//!
//! `data/prs.json` is the single source of truth and is embedded at compile time.
//! Everything here is pure, so it runs under a native `cargo test`.

mod date;
mod error;
mod inline;
mod validate;

use std::collections::BTreeMap;

use serde_json::Value;

pub use date::Date;
pub use error::DataError;
pub use inline::{Segment, parse_inline};

/// How many top-ranked PRs the home page shows; the rest live on the all-PRs page.
pub const FEATURED_COUNT: usize = 15;

const EMBEDDED_PRS_JSON: &str = include_str!("../data/prs.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diff {
    pub add: u32,
    pub del: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pr {
    pub number: u64,
    pub rank: u32,
    pub url: String,
    pub tag: String,
    pub title: String,
    pub diff: Diff,
    pub merged: Date,
    pub description: String,
}

/// Parses and validates PR entries. Ranks must be unique and exactly 1..=N.
pub fn load_prs(json: &str) -> Result<Vec<Pr>, DataError> {
    let root: Value =
        serde_json::from_str(json).map_err(|error| DataError::MalformedJson(error.to_string()))?;
    let Value::Array(entries) = root else {
        return Err(DataError::NotAnArray);
    };

    let prs = entries
        .iter()
        .enumerate()
        .map(|(index, raw)| validate::validate_pr(raw, index))
        .collect::<Result<Vec<Pr>, DataError>>()?;
    check_unique_ranks(&prs)?;
    check_contiguous_ranks(&prs)?;
    Ok(prs)
}

/// The PRs shipped inside the binary, validated.
pub fn embedded_prs() -> Result<Vec<Pr>, DataError> {
    load_prs(EMBEDDED_PRS_JSON)
}

/// Newest merge first; ascending PR number within the same date.
pub fn sorted_newest_first(prs: &[Pr]) -> Vec<Pr> {
    let mut sorted = prs.to_vec();
    sorted.sort_by(|a, b| b.merged.cmp(&a.merged).then(a.number.cmp(&b.number)));
    sorted
}

/// Entries ranked 1..=[`FEATURED_COUNT`], best rank first.
pub fn featured(prs: &[Pr]) -> Vec<Pr> {
    let mut top: Vec<Pr> = prs
        .iter()
        .filter(|pr| usize::try_from(pr.rank).is_ok_and(|rank| rank <= FEATURED_COUNT))
        .cloned()
        .collect();
    top.sort_by_key(|pr| pr.rank);
    top
}

/// Label for the home page's link to the full listing, e.g. `See all 17 merged PRs`.
pub fn all_prs_link_label(total: usize) -> String {
    let noun = if total == 1 { "PR" } else { "PRs" };
    format!("See all {total} merged {noun}")
}

fn check_unique_ranks(prs: &[Pr]) -> Result<(), DataError> {
    let mut number_by_rank: BTreeMap<u32, u64> = BTreeMap::new();
    for pr in prs {
        if let Some(&first) = number_by_rank.get(&pr.rank) {
            return Err(DataError::DuplicateRank {
                rank: pr.rank,
                first,
                second: pr.number,
            });
        }
        number_by_rank.insert(pr.rank, pr.number);
    }
    Ok(())
}

/// Requires ranks to be exactly 1..=N so a gap cannot silently shrink the featured list.
fn check_contiguous_ranks(prs: &[Pr]) -> Result<(), DataError> {
    let entry_count = prs.len();
    let in_range = |rank: u32| usize::try_from(rank).is_ok_and(|rank| rank <= entry_count);
    let mut present = vec![false; entry_count + 1];
    let mut unexpected: Vec<u32> = Vec::new();
    for pr in prs {
        if in_range(pr.rank) {
            present[pr.rank as usize] = true;
        } else {
            unexpected.push(pr.rank);
        }
    }
    let missing: Vec<u32> = (1..=entry_count)
        .filter(|&rank| !present[rank])
        .map(|rank| u32::try_from(rank).unwrap_or(u32::MAX))
        .collect();
    if missing.is_empty() && unexpected.is_empty() {
        return Ok(());
    }
    unexpected.sort_unstable();
    Err(DataError::RankSequence {
        entry_count,
        missing,
        unexpected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Value {
        json!({
            "number": 100,
            "rank": 1,
            "url": "https://github.com/NethermindEth/nethermind/pull/100",
            "tag": "Core",
            "title": "Sample title",
            "diff": { "add": 10, "del": 2 },
            "merged": "2026-01-02",
            "description": "Sample `Code` description with an & ampersand and a \"quote\"."
        })
    }

    fn with(overrides: Value) -> Value {
        let mut pr = sample();
        for (key, value) in overrides.as_object().unwrap() {
            pr[key] = value.clone();
        }
        pr
    }

    fn without(field: &str) -> Value {
        let mut pr = sample();
        pr.as_object_mut().unwrap().remove(field);
        pr
    }

    fn load(entries: Vec<Value>) -> Result<Vec<Pr>, DataError> {
        load_prs(&Value::Array(entries).to_string())
    }

    fn load_one(entry: Value) -> Result<Vec<Pr>, DataError> {
        load(vec![entry])
    }

    fn error_text(result: Result<Vec<Pr>, DataError>) -> String {
        result.expect_err("expected a validation error").to_string()
    }

    fn pr(number: u64, rank: u32, merged: &str) -> Pr {
        Pr {
            number,
            rank,
            url: format!("https://github.com/o/r/pull/{number}"),
            tag: "Core".into(),
            title: "Title".into(),
            diff: Diff { add: 1, del: 0 },
            merged: Date::parse_iso(merged).unwrap(),
            description: "Description".into(),
        }
    }

    /// `count` PRs whose rank is the reverse of their merge-date order.
    fn ranked_prs(count: u32) -> Vec<Pr> {
        (0..count)
            .map(|i| {
                pr(
                    1000 + u64::from(i),
                    count - i,
                    &format!("2026-01-{:02}", i + 1),
                )
            })
            .collect()
    }

    fn ranks(prs: &[Pr]) -> Vec<u32> {
        prs.iter().map(|pr| pr.rank).collect()
    }

    fn numbers(prs: &[Pr]) -> Vec<u64> {
        prs.iter().map(|pr| pr.number).collect()
    }

    // --- load_prs: shape ---

    #[test]
    fn loads_valid_entries() {
        let prs = load_one(sample()).unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].number, 100);
        assert_eq!(prs[0].diff, Diff { add: 10, del: 2 });
        assert_eq!(prs[0].merged, Date::parse_iso("2026-01-02").unwrap());
    }

    #[test]
    fn empty_array_loads_as_no_entries() {
        assert_eq!(load(vec![]).unwrap(), vec![]);
    }

    #[test]
    fn malformed_json_is_reported() {
        assert!(matches!(
            load_prs("{not valid json"),
            Err(DataError::MalformedJson(_))
        ));
    }

    #[test]
    fn non_array_root_is_reported() {
        assert_eq!(load_prs(r#"{"not": "a list"}"#), Err(DataError::NotAnArray));
    }

    #[test]
    fn non_object_entry_is_reported_with_its_index() {
        let result = load(vec![sample(), json!("nope")]);
        assert_eq!(result, Err(DataError::EntryNotObject { index: 1 }));
    }

    // --- load_prs: field validation ---

    #[test]
    fn missing_field_names_the_entry_and_field() {
        let result = load_one(without("title"));
        assert_eq!(
            result,
            Err(DataError::MissingFields {
                index: 0,
                number: Some(100),
                fields: vec!["title".into()]
            })
        );
    }

    #[test]
    fn missing_rank_mentions_rank() {
        assert!(error_text(load_one(without("rank"))).contains("rank"));
    }

    #[test]
    fn malformed_diff_is_rejected() {
        for bad in [
            json!({ "add": 1 }),
            json!({ "del": 1 }),
            json!("1/2"),
            json!(null),
            json!([1, 2]),
        ] {
            let text = error_text(load_one(with(json!({ "diff": bad.clone() }))));
            assert!(text.contains("'diff'"), "diff {bad} gave: {text}");
        }
    }

    #[test]
    fn negative_diff_value_is_rejected() {
        assert!(
            error_text(load_one(with(json!({ "diff": { "add": -1, "del": 2 } }))))
                .contains("diff.add")
        );
    }

    #[test]
    fn non_integer_diff_value_is_rejected() {
        for bad in [json!("10"), json!(1.5), json!(null)] {
            let text = error_text(load_one(with(json!({ "diff": { "add": bad, "del": 2 } }))));
            assert!(text.contains("diff.add"), "{text}");
        }
    }

    #[test]
    fn boolean_diff_values_are_rejected() {
        let text = error_text(load_one(with(json!({ "diff": { "add": true, "del": 2 } }))));
        assert!(text.contains("diff.add"), "{text}");
        let text = error_text(load_one(with(
            json!({ "diff": { "add": 1, "del": false } }),
        )));
        assert!(text.contains("diff.del"), "{text}");
    }

    #[test]
    fn non_integer_number_is_rejected() {
        for bad in [json!("100"), json!(1.5), json!(-1), json!(null)] {
            let text = error_text(load_one(with(json!({ "number": bad.clone() }))));
            assert!(text.contains("'number'"), "number {bad} gave: {text}");
        }
    }

    #[test]
    fn boolean_number_is_rejected() {
        for bad in [true, false] {
            let text = error_text(load_one(with(json!({ "number": bad }))));
            assert!(text.contains("'number'"), "{text}");
        }
    }

    #[test]
    fn empty_string_field_is_rejected() {
        for field in ["url", "tag", "title", "description", "merged"] {
            let text = error_text(load_one(with(json!({ field: "" }))));
            assert!(text.contains(&format!("'{field}'")), "{field}: {text}");
        }
    }

    #[test]
    fn non_string_text_field_is_rejected() {
        let text = error_text(load_one(with(json!({ "tag": 5 }))));
        assert!(text.contains("'tag'"), "{text}");
    }

    #[test]
    fn non_integer_rank_is_rejected() {
        for bad in [json!("1"), json!(1.5), json!(null), json!(true)] {
            let text = error_text(load_one(with(json!({ "rank": bad.clone() }))));
            assert!(text.contains("'rank'"), "rank {bad} gave: {text}");
        }
    }

    #[test]
    fn zero_or_negative_rank_is_rejected() {
        for bad in [0, -3] {
            let text = error_text(load_one(with(json!({ "rank": bad }))));
            assert!(text.contains("'rank'"), "rank {bad} gave: {text}");
        }
    }

    #[test]
    fn oversized_rank_is_rejected_instead_of_wrapping() {
        let text = error_text(load_one(with(json!({ "rank": 4_294_967_297_u64 }))));
        assert!(text.contains("'rank'"), "{text}");
    }

    #[test]
    fn invalid_merged_date_is_rejected() {
        for bad in ["not-a-date", "2026-13-01", "2026-02-30"] {
            let text = error_text(load_one(with(json!({ "merged": bad }))));
            assert!(text.contains("'merged'"), "{bad}: {text}");
        }
    }

    // --- load_prs: url scheme ---

    #[test]
    fn http_and_https_urls_are_accepted() {
        for url in ["https://github.com/o/r/pull/1", "http://example.com/x"] {
            assert!(load_one(with(json!({ "url": url }))).is_ok(), "{url}");
        }
    }

    #[test]
    fn non_web_url_schemes_are_rejected() {
        for url in [
            "javascript:alert(1)",
            "data:text/html,hi",
            "ftp://example.com/x",
            "//example.com/x",
            "github.com/o/r/pull/1",
            "https://",
            "https://exa mple.com",
            "HTTPS-not://x",
        ] {
            let text = error_text(load_one(with(json!({ "url": url }))));
            assert!(text.contains("'url'"), "{url}: {text}");
        }
    }

    // --- load_prs: ranks ---

    #[test]
    fn duplicate_rank_names_both_entries() {
        let first = with(json!({ "number": 1, "rank": 2 }));
        let second = with(json!({ "number": 2, "rank": 2 }));
        assert_eq!(
            load(vec![first, second]),
            Err(DataError::DuplicateRank {
                rank: 2,
                first: 1,
                second: 2
            })
        );
    }

    #[test]
    fn rank_gap_names_the_missing_rank() {
        let entries = vec![
            with(json!({ "number": 1, "rank": 1 })),
            with(json!({ "number": 2, "rank": 3 })),
        ];
        assert_eq!(
            load(entries),
            Err(DataError::RankSequence {
                entry_count: 2,
                missing: vec![2],
                unexpected: vec![3]
            })
        );
    }

    #[test]
    fn ranks_not_starting_at_one_name_the_missing_rank() {
        let entries = vec![
            with(json!({ "number": 1, "rank": 2 })),
            with(json!({ "number": 2, "rank": 3 })),
        ];
        assert_eq!(
            load(entries),
            Err(DataError::RankSequence {
                entry_count: 2,
                missing: vec![1],
                unexpected: vec![3]
            })
        );
    }

    #[test]
    fn rank_beyond_entry_count_is_named() {
        let entries = vec![
            with(json!({ "number": 1, "rank": 1 })),
            with(json!({ "number": 2, "rank": 5 })),
        ];
        let text = error_text(load(entries));
        assert!(text.contains("unexpected") && text.contains('5'), "{text}");
    }

    #[test]
    fn rank_order_in_the_file_does_not_matter() {
        let entries = vec![
            with(json!({ "number": 1, "rank": 2 })),
            with(json!({ "number": 2, "rank": 1 })),
        ];
        assert!(load(entries).is_ok());
    }

    // --- orderings ---

    #[test]
    fn sorts_newest_date_first() {
        let sorted = sorted_newest_first(&[pr(1, 1, "2026-01-01"), pr(2, 2, "2026-02-01")]);
        assert_eq!(numbers(&sorted), vec![2, 1]);
    }

    #[test]
    fn sorts_ascending_number_within_same_date() {
        let sorted = sorted_newest_first(&[pr(200, 1, "2026-01-01"), pr(100, 2, "2026-01-01")]);
        assert_eq!(numbers(&sorted), vec![100, 200]);
    }

    #[test]
    fn sorts_mixed_dates_and_numbers() {
        let prs = [
            pr(13833, 1, "2026-09-25"),
            pr(13506, 2, "2026-09-25"),
            pr(13485, 3, "2026-09-23"),
            pr(13366, 4, "2026-09-23"),
        ];
        assert_eq!(
            numbers(&sorted_newest_first(&prs)),
            vec![13506, 13833, 13366, 13485]
        );
    }

    #[test]
    fn featured_orders_by_rank_ascending() {
        let prs = [
            pr(1, 3, "2026-01-01"),
            pr(2, 1, "2026-01-02"),
            pr(3, 2, "2026-01-03"),
        ];
        assert_eq!(ranks(&featured(&prs)), vec![1, 2, 3]);
    }

    #[test]
    fn featured_excludes_entries_ranked_beyond_the_count() {
        let top = featured(&ranked_prs(FEATURED_COUNT as u32 + 2));
        assert_eq!(ranks(&top), (1..=FEATURED_COUNT as u32).collect::<Vec<_>>());
    }

    #[test]
    fn featured_keeps_exactly_the_count() {
        assert_eq!(
            featured(&ranked_prs(FEATURED_COUNT as u32)).len(),
            FEATURED_COUNT
        );
    }

    #[test]
    fn featured_keeps_everything_when_there_are_fewer() {
        assert_eq!(featured(&ranked_prs(4)).len(), 4);
    }

    #[test]
    fn featured_of_nothing_is_nothing() {
        assert!(featured(&[]).is_empty());
    }

    #[test]
    fn link_label_shows_the_total() {
        assert_eq!(all_prs_link_label(17), "See all 17 merged PRs");
    }

    #[test]
    fn link_label_is_singular_for_one_pr() {
        assert_eq!(all_prs_link_label(1), "See all 1 merged PR");
    }

    // --- embedded data ---

    #[test]
    fn embedded_data_is_valid() {
        let prs = embedded_prs().expect("data/prs.json must pass validation");
        assert_eq!(prs.len(), 17);
    }

    #[test]
    fn embedded_data_features_ranks_one_to_fifteen() {
        let prs = embedded_prs().unwrap();
        assert_eq!(ranks(&featured(&prs)), (1..=15).collect::<Vec<u32>>());
    }
}
