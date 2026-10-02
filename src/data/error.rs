//! The error type for loading and validating `data/prs.json`.

use std::fmt;

/// Everything that can be wrong with the PR data, with enough context to find the entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataError {
    /// The text is not valid JSON.
    MalformedJson(String),
    /// The JSON root is not an array.
    NotAnArray,
    /// An entry in the array is not a JSON object.
    EntryNotObject { index: usize },
    /// Required fields are absent from an entry.
    MissingFields {
        index: usize,
        number: Option<u64>,
        fields: Vec<String>,
    },
    /// A field is present but its value is unacceptable. `field` is a dotted path such as `diff.add`.
    InvalidField {
        index: usize,
        number: Option<u64>,
        field: String,
        reason: String,
    },
    /// Two entries claim the same rank.
    DuplicateRank { rank: u32, first: u64, second: u64 },
    /// Ranks are not exactly 1..=entry_count.
    RankSequence {
        entry_count: usize,
        missing: Vec<u32>,
        unexpected: Vec<u32>,
    },
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedJson(detail) => write!(f, "malformed JSON in data/prs.json: {detail}"),
            Self::NotAnArray => write!(f, "data/prs.json must contain a JSON array of PR entries"),
            Self::EntryNotObject { index } => {
                write!(f, "data/prs.json entry {index} is not an object")
            }
            Self::MissingFields {
                index,
                number,
                fields,
            } => write!(
                f,
                "data/prs.json entry {index}{} is missing field(s): {}",
                number_suffix(*number),
                fields.join(", ")
            ),
            Self::InvalidField {
                index,
                number,
                field,
                reason,
            } => write!(
                f,
                "data/prs.json entry {index}{} field '{field}' is invalid: {reason}",
                number_suffix(*number)
            ),
            Self::DuplicateRank {
                rank,
                first,
                second,
            } => write!(
                f,
                "duplicate rank {rank} in data/prs.json on entries #{first} and #{second}; ranks must be unique"
            ),
            Self::RankSequence {
                entry_count,
                missing,
                unexpected,
            } => write!(
                f,
                "ranks in data/prs.json must be exactly 1..={entry_count} with no gaps; \
                 missing rank(s): {missing:?}, unexpected rank(s): {unexpected:?}"
            ),
        }
    }
}

impl std::error::Error for DataError {}

fn number_suffix(number: Option<u64>) -> String {
    number.map_or_else(String::new, |number| format!(" (number={number})"))
}
