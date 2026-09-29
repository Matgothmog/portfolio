# portfolio

Personal portfolio for Matgothmog — a static site covering open-source
contributions to Nethermind, Ethereum Foundation bug-bounty work, and the
Postage project from ETHOnline 2026.

## Stack

Plain HTML, CSS, and a small vanilla JS file for the mobile nav toggle. No
framework, no runtime dependencies. The pages are generated from templates
and a data file by a small build script that uses only the Python standard
library. All asset references are relative, so the built site works unchanged
from a project-pages URL (`matgothmog.github.io/portfolio`) or from a
user-pages root.

## Structure

```
src/index.html        page template (hero + three sections + footer), with
                      a marker comment where the featured PR cards get
                      spliced in and one for the "see all" link
src/prs.html          template for the full merged-PR listing (prs.html)
data/prs.json         merged-PR entries rendered into cards
build.py              stdlib Python script that renders the templates and
                      data into dist/
Makefile              build, clean, serve targets
tests/                unittest suite for build.py
assets/css/style.css  shared stylesheet
assets/js/main.js     mobile nav toggle
assets/img/           Postage screenshots used in the project section
.nojekyll             disables Jekyll processing on GitHub Pages
dist/                 build output (generated, gitignored)
```

## Build

```bash
make build   # renders src/*.html + data/prs.json into dist/
make clean   # removes dist/
make serve   # builds, then serves dist/ at http://localhost:8000
```

`make build` runs `python3 build.py`, which loads and validates
`data/prs.json` and writes two pages into `dist/`:

- `index.html` shows the top `FEATURED_COUNT` PRs (15, set at the top of
  `build.py`) in rank order, followed by a "See all N merged PRs" link.
- `prs.html` lists every PR as the same cards, newest merge first.

It also copies `assets/` and `.nojekyll` into `dist/`.

## Tests

```bash
python3 -m unittest
```

Covers card rendering, date formatting, sorting, ranking and featured
selection, PR data validation, and the "see all" link, including a full build
run against a temporary project layout.

## Adding a PR entry

Add an entry to the array in `data/prs.json` with these fields:

| Field | Type | Notes |
|---|---|---|
| `number` | int | PR number |
| `rank` | int | 1 to N across N entries, no gaps or repeats; ranks 1 to `FEATURED_COUNT` appear on the main page |
| `url` | str | full PR URL |
| `tag` | str | free-text category (e.g. `JSON-RPC`, `Network / TxPool`) |
| `title` | str | sentence, may contain `` `backtick` `` code spans |
| `diff` | object | `{"add": int, "del": int}` |
| `merged` | str | ISO date, `YYYY-MM-DD` |
| `description` | str | prose, may contain `` `backtick` `` code spans |

Backtick-delimited text in `title` and `description` is rendered as
`<code>…</code>`. The main page shows the entries ranked 1 to 15, best rank
first; `prs.html` shows all of them, sorted automatically by newest merge
date first and by ascending PR number within the same date. Entries can go
anywhere in the array, but ranks must run 1 to N with no gaps or repeats: a
new entry either goes last as rank N, or takes an earlier rank and moves
every entry from that rank onward down by one. `build.py` validates every
field on build and fails loudly on a missing or malformed one, a duplicate
rank, or a gap in the ranks, so run `make build` after editing the file to
catch mistakes.

## Deploy

The site is published with GitHub Pages at
https://matgothmog.github.io/portfolio/. The workflow in
`.github/workflows/deploy.yml` runs on every push to `main` (and on manual
dispatch): it builds the site with `python3 build.py` and deploys the
resulting `dist/` directory through the Pages Actions source. `dist/` stays
gitignored; only the workflow publishes it.
