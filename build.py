#!/usr/bin/env python3
"""Render the portfolio site from src/*.html and data/prs.json into dist/.

Standard library only. Usage: python3 build.py
"""

from __future__ import annotations

import html
import json
import re
import shutil
import sys
from datetime import date
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent
TEMPLATE_PATH = ROOT / "src" / "index.html"
PRS_TEMPLATE_PATH = ROOT / "src" / "prs.html"
DATA_PATH = ROOT / "data" / "prs.json"
DIST_DIR = ROOT / "dist"
ASSETS_DIR = ROOT / "assets"
NOJEKYLL_PATH = ROOT / ".nojekyll"

MARKER_LINE = "        <!-- PR_CARDS -->\n"
LINK_MARKER_LINE = "      <!-- ALL_PRS_LINK -->\n"

# How many top-ranked PRs the main page shows; the rest live on prs.html.
FEATURED_COUNT = 15

MONTH_ABBR = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
]

REQUIRED_PR_FIELDS = {"number", "rank", "url", "tag", "title", "diff", "merged", "description"}
REQUIRED_DIFF_FIELDS = {"add", "del"}

CODE_SPAN = re.compile(r"`([^`]+)`")

ARTICLE_TEMPLATE = """        <article class="card pr-card">
          <div class="card-top">
            <a class="badge" href="{url}" target="_blank" rel="noopener">#{number}</a>
            <span class="tag">{tag}</span>
          </div>
          <h{level}>{title}</h{level}>
          <p class="card-meta"><span class="diff"><span class="add">+{add}</span> <span class="del">-{del_}</span></span><span class="sep">·</span><span>Merged {date}</span></p>
          <p class="card-body">{description}</p>
        </article>"""


class BuildError(Exception):
    """Raised when the site cannot be built, with context on what went wrong."""


# --- pure rendering helpers -------------------------------------------------

def escape_attr(value: str) -> str:
    """Escape a string for use inside a double-quoted HTML attribute."""
    return html.escape(value, quote=True)


def render_inline(text: str) -> str:
    """HTML-escape text, then turn `backtick spans` into <code> spans.

    The escape runs first so any &, < or > in the source text is neutralised;
    the backtick markup is applied afterwards on the escaped string, so code
    spans can never introduce unescaped HTML.
    """
    escaped = html.escape(text, quote=False)
    return CODE_SPAN.sub(lambda m: f"<code>{m.group(1)}</code>", escaped)


def format_merged_date(iso_date: str) -> str:
    """Format an ISO date ("2026-09-25") as "Sep 25, 2026"."""
    try:
        parsed = date.fromisoformat(iso_date)
    except ValueError as exc:
        raise BuildError(f"invalid merged date {iso_date!r}: {exc}") from exc
    return f"{MONTH_ABBR[parsed.month - 1]} {parsed.day}, {parsed.year}"


def sort_prs(prs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Sort newest-merged-first; ascending PR number within the same date.

    Two stable passes: sort by number first, then by date descending. The
    date pass preserves the ascending-number order already established
    for entries that share a date.
    """
    by_number = sorted(prs, key=lambda pr: pr["number"])
    return sorted(by_number, key=lambda pr: pr["merged"], reverse=True)


def render_card(pr: dict[str, Any], heading_level: int = 3) -> str:
    return ARTICLE_TEMPLATE.format(
        level=heading_level,
        url=escape_attr(pr["url"]),
        number=pr["number"],
        tag=render_inline(pr["tag"]),
        title=render_inline(pr["title"]),
        add=pr["diff"]["add"],
        del_=pr["diff"]["del"],
        date=format_merged_date(pr["merged"]),
        description=render_inline(pr["description"]),
    )


def select_featured(prs: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Entries ranked 1..FEATURED_COUNT, best rank first."""
    featured = [pr for pr in prs if pr["rank"] <= FEATURED_COUNT]
    return sorted(featured, key=lambda pr: pr["rank"])


def render_cards(prs: list[dict[str, Any]], heading_level: int = 3) -> str:
    """Render cards in the order given."""
    return "\n\n".join(render_card(pr, heading_level) for pr in prs) + "\n"


def render_all_prs_link(total: int) -> str:
    noun = "PR" if total == 1 else "PRs"
    return (
        '      <p class="section-cta">'
        f'<a class="btn btn-outline" href="prs.html">See all {total} merged {noun} &rarr;</a>'
        "</p>\n"
    )


def build_page(template: str, prs: list[dict[str, Any]], heading_level: int = 3) -> str:
    """Splice the rendered PR cards, in the order given, into the template at the marker line."""
    occurrences = template.count(MARKER_LINE)
    if occurrences != 1:
        raise BuildError(
            f"expected exactly one marker line {MARKER_LINE!r} in template, "
            f"found {occurrences}"
        )
    return template.replace(MARKER_LINE, render_cards(prs, heading_level))


def build_index_page(template: str, prs: list[dict[str, Any]]) -> str:
    """Main page: featured cards in rank order, then a link to the full listing."""
    occurrences = template.count(LINK_MARKER_LINE)
    if occurrences != 1:
        raise BuildError(
            f"expected exactly one marker line {LINK_MARKER_LINE!r} in template, "
            f"found {occurrences}"
        )
    with_link = template.replace(LINK_MARKER_LINE, render_all_prs_link(len(prs)))
    return build_page(with_link, select_featured(prs))


def build_prs_page(template: str, prs: list[dict[str, Any]]) -> str:
    """Full listing: every entry, newest merge first.

    The page title is an h1, so its cards sit one level higher than on the index.
    """
    if LINK_MARKER_LINE in template:
        raise BuildError(
            f"marker line {LINK_MARKER_LINE!r} belongs in the index template, "
            "not the full listing template"
        )
    return build_page(template, sort_prs(prs), heading_level=2)


# --- validation --------------------------------------------------------------

def validate_pr(pr: Any, index: int) -> dict[str, Any]:
    if not isinstance(pr, dict):
        raise BuildError(f"data/prs.json entry {index} is not an object: {pr!r}")

    missing = REQUIRED_PR_FIELDS - pr.keys()
    if missing:
        raise BuildError(
            f"data/prs.json entry {index} (number={pr.get('number', '?')}) "
            f"is missing field(s): {', '.join(sorted(missing))}"
        )

    diff = pr["diff"]
    if not isinstance(diff, dict) or REQUIRED_DIFF_FIELDS - diff.keys():
        raise BuildError(
            f"data/prs.json entry {index} (number={pr['number']}) has a malformed "
            f"'diff' field, expected an object with 'add' and 'del'"
        )
    for diff_field in REQUIRED_DIFF_FIELDS:
        if not isinstance(diff[diff_field], int) or diff[diff_field] < 0:
            raise BuildError(
                f"data/prs.json entry {index} (number={pr['number']}) field "
                f"'diff.{diff_field}' must be a non-negative integer, "
                f"got {diff[diff_field]!r}"
            )

    if not isinstance(pr["number"], int):
        raise BuildError(
            f"data/prs.json entry {index} field 'number' must be an integer, "
            f"got {pr['number']!r}"
        )

    rank = pr["rank"]
    if not isinstance(rank, int) or isinstance(rank, bool) or rank < 1:
        raise BuildError(
            f"data/prs.json entry {index} (number={pr['number']}) field 'rank' "
            f"must be a positive integer, got {rank!r}"
        )

    for field in ("url", "tag", "title", "description", "merged"):
        if not isinstance(pr[field], str) or not pr[field]:
            raise BuildError(
                f"data/prs.json entry {index} (number={pr['number']}) field "
                f"{field!r} must be a non-empty string"
            )

    return pr


def load_prs(path: Path) -> list[dict[str, Any]]:
    try:
        raw_text = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise BuildError(f"could not read {path}: {exc}") from exc

    try:
        data = json.loads(raw_text)
    except json.JSONDecodeError as exc:
        raise BuildError(f"malformed JSON in {path}: {exc}") from exc

    if not isinstance(data, list):
        raise BuildError(f"{path} must contain a JSON array of PR entries")

    prs = [validate_pr(pr, i) for i, pr in enumerate(data)]
    check_unique_ranks(prs)
    check_contiguous_ranks(prs)
    return prs


def check_unique_ranks(prs: list[dict[str, Any]]) -> None:
    number_by_rank: dict[int, int] = {}
    for pr in prs:
        rank = pr["rank"]
        if rank in number_by_rank:
            raise BuildError(
                f"duplicate rank {rank} in data/prs.json on entries "
                f"#{number_by_rank[rank]} and #{pr['number']}; ranks must be unique"
            )
        number_by_rank[rank] = pr["number"]


def check_contiguous_ranks(prs: list[dict[str, Any]]) -> None:
    """Require ranks to be exactly 1..N so a gap can't silently shrink the featured list."""
    expected = set(range(1, len(prs) + 1))
    actual = {pr["rank"] for pr in prs}
    if actual == expected:
        return
    missing = sorted(expected - actual)
    unexpected = sorted(actual - expected)
    raise BuildError(
        f"ranks in data/prs.json must be exactly 1..{len(prs)} with no gaps; "
        f"missing rank(s): {missing}, unexpected rank(s): {unexpected}"
    )


def load_template(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as exc:
        raise BuildError(f"could not read template {path}: {exc}") from exc


# --- file I/O ------------------------------------------------------------

def copy_static_files(dist_dir: Path) -> None:
    if ASSETS_DIR.is_dir():
        shutil.copytree(ASSETS_DIR, dist_dir / "assets", dirs_exist_ok=True)
    if NOJEKYLL_PATH.is_file():
        shutil.copy2(NOJEKYLL_PATH, dist_dir / ".nojekyll")


def write_output(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def build() -> None:
    index_template = load_template(TEMPLATE_PATH)
    prs_template = load_template(PRS_TEMPLATE_PATH)
    prs = load_prs(DATA_PATH)
    index_page = build_index_page(index_template, prs)
    prs_page = build_prs_page(prs_template, prs)

    DIST_DIR.mkdir(parents=True, exist_ok=True)
    write_output(DIST_DIR / "index.html", index_page)
    write_output(DIST_DIR / "prs.html", prs_page)
    copy_static_files(DIST_DIR)


def main() -> int:
    try:
        build()
    except BuildError as exc:
        print(f"build.py: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
