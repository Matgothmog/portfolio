//! Field-level validation of one raw JSON entry, producing a typed [`Pr`].

use serde_json::{Map, Value};

use super::date::Date;
use super::error::DataError;
use super::{Diff, Pr};

const REQUIRED_PR_FIELDS: [&str; 8] = [
    "number",
    "rank",
    "url",
    "tag",
    "title",
    "diff",
    "merged",
    "description",
];
const REQUIRED_DIFF_FIELDS: [&str; 2] = ["add", "del"];

/// Context shared by every error raised for one entry.
struct Entry<'a> {
    index: usize,
    number: Option<u64>,
    fields: &'a Map<String, Value>,
}

impl Entry<'_> {
    fn invalid(&self, field: &str, reason: impl Into<String>) -> DataError {
        DataError::InvalidField {
            index: self.index,
            number: self.number,
            field: field.to_string(),
            reason: reason.into(),
        }
    }

    fn non_empty_string(&self, field: &str) -> Result<String, DataError> {
        match self.fields.get(field) {
            Some(Value::String(text)) if !text.is_empty() => Ok(text.clone()),
            other => Err(self.invalid(
                field,
                format!("must be a non-empty string, got {}", describe(other)),
            )),
        }
    }

    fn unsigned(&self, value: Option<&Value>, field: &str) -> Result<u64, DataError> {
        // `as_u64` is `None` for booleans, floats, negatives and strings alike.
        value.and_then(Value::as_u64).ok_or_else(|| {
            self.invalid(
                field,
                format!("must be a non-negative integer, got {}", describe(value)),
            )
        })
    }
}

fn describe(value: Option<&Value>) -> String {
    value.map_or_else(|| "nothing".to_string(), Value::to_string)
}

pub(super) fn validate_pr(raw: &Value, index: usize) -> Result<Pr, DataError> {
    let Value::Object(fields) = raw else {
        return Err(DataError::EntryNotObject { index });
    };
    let number_hint = fields.get("number").and_then(Value::as_u64);

    let missing: Vec<String> = REQUIRED_PR_FIELDS
        .iter()
        .filter(|field| !fields.contains_key(**field))
        .map(|field| (*field).to_string())
        .collect();
    if !missing.is_empty() {
        return Err(DataError::MissingFields {
            index,
            number: number_hint,
            fields: missing,
        });
    }

    let entry = Entry {
        index,
        number: number_hint,
        fields,
    };
    let number = entry.unsigned(fields.get("number"), "number")?;
    let rank = parse_rank(&entry)?;
    let url = parse_url(&entry)?;
    let merged = parse_merged(&entry)?;
    let diff = parse_diff(&entry)?;

    Ok(Pr {
        number,
        rank,
        url,
        tag: entry.non_empty_string("tag")?,
        title: entry.non_empty_string("title")?,
        diff,
        merged,
        description: entry.non_empty_string("description")?,
    })
}

fn parse_rank(entry: &Entry) -> Result<u32, DataError> {
    let rank = entry.unsigned(entry.fields.get("rank"), "rank")?;
    match u32::try_from(rank) {
        Ok(rank) if rank >= 1 => Ok(rank),
        _ => Err(entry.invalid(
            "rank",
            format!("must be a positive integer that fits in 32 bits, got {rank}"),
        )),
    }
}

/// Only web links may reach an `href`, so `javascript:` and friends are refused.
fn parse_url(entry: &Entry) -> Result<String, DataError> {
    let url = entry.non_empty_string("url")?;
    let host_and_path = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"));
    match host_and_path {
        Some(rest) if !rest.is_empty() && !url.chars().any(char::is_whitespace) => Ok(url),
        _ => Err(entry.invalid(
            "url",
            format!("must be an http(s) URL without whitespace, got {url:?}"),
        )),
    }
}

fn parse_merged(entry: &Entry) -> Result<Date, DataError> {
    let text = entry.non_empty_string("merged")?;
    Date::parse_iso(&text).map_err(|reason| entry.invalid("merged", reason))
}

fn parse_diff(entry: &Entry) -> Result<Diff, DataError> {
    let malformed = || entry.invalid("diff", "expected an object with 'add' and 'del'");
    let Some(Value::Object(diff)) = entry.fields.get("diff") else {
        return Err(malformed());
    };
    if REQUIRED_DIFF_FIELDS
        .iter()
        .any(|field| !diff.contains_key(*field))
    {
        return Err(malformed());
    }
    let line_count = |field: &str| -> Result<u32, DataError> {
        let path = format!("diff.{field}");
        let count = entry.unsigned(diff.get(field), &path)?;
        u32::try_from(count)
            .map_err(|_| entry.invalid(&path, format!("{count} does not fit in 32 bits")))
    };
    Ok(Diff {
        add: line_count("add")?,
        del: line_count("del")?,
    })
}
