# portfolio

Personal portfolio for Matgothmog — a static site covering open-source
contributions to Nethermind, Ethereum Foundation bug-bounty work, and the
Postage project from ETHOnline 2026.

Published at https://matgothmog.github.io/portfolio/.

## Stack

Built with Rust using Leptos 0.8 (client-side rendered), bundled by Trunk 0.21
into a WebAssembly app. No JavaScript dependencies: the only script outside the
wasm bundle is a small inline snippet in `index.html` that sets the theme before
first paint. The app renders two routes (`/` and `/prs`, plus the `/index.html`
and `/prs.html` aliases for old URLs). PR data is embedded from `data/prs.json`
with `include_str!` and parsed and validated at runtime by `load_prs` in `src/data.rs`. A
`<noscript>` fallback in `index.html` covers browsers without JavaScript.

## Prerequisites

Rust 1.88 or newer (required by Leptos 0.8; the crate uses edition 2024) and Trunk:

```bash
cargo install --locked trunk --version ~0.21
```

`rust-toolchain.toml` pins the stable channel and pulls in `clippy`, `rustfmt`
and the `wasm32-unknown-unknown` target, so no separate `rustup target add` is
needed.

## Structure

```
src/main.rs                    app entry point
src/lib.rs                     lib root
src/app.rs                     root component and router
src/pages/                     routable pages (home.rs, all_prs.rs)
src/components/                shared UI components, including theme_toggle.rs
src/theme.rs                   theme state and persistence
src/data.rs                    PR loading, ranking and sorting (with unit tests)
src/data/                      date parsing, backtick code spans, field
                               validation, and the data error type
data/prs.json                  PR data, embedded into the wasm bundle
index.html                     app shell, theme-init snippet, noscript fallback
style/main.css                 shared stylesheet
assets/img/                    project screenshots
Cargo.toml                     crate manifest and dependencies
rust-toolchain.toml            toolchain, components and wasm32 target
Trunk.toml                     Trunk build config and post-build hook
Makefile                       serve, build, test, lint, clean targets
.nojekyll                      tracked marker for GitHub Pages; Trunk does not
                               copy it into dist/
dist/                          build output (generated, gitignored)
```

## Build

```bash
make serve       # trunk serve --open=false
make build       # cargo test, then trunk build --release
make build-pages # cargo test, then trunk build --release --public-url /portfolio/
make test        # cargo test
make lint        # cargo fmt --check, then clippy (native and wasm32) with -D warnings
make clean       # rm -rf dist
```

`make build` and `make build-pages` run `cargo test` first, so a malformed
`data/prs.json` fails the build. `include_str!` only embeds the file; the data
is validated at runtime by `load_prs`, which the `embedded_data_is_valid` test
exercises.

## Tests

```bash
make test
```

Unit tests cover PR data validation and ranking, date parsing, backtick code
spans, the router base path, theme selection, and the shipped `data/prs.json`.

## Adding a PR entry

Add an entry to the array in `data/prs.json` with these fields. Every field is
required, and strings must not be empty.

| Field | Type | Notes |
|---|---|---|
| `number` | int | PR number |
| `rank` | int | 1 to N across N entries, no gaps or repeats; ranks 1 to 15 appear on the main page |
| `url` | str | full PR URL |
| `tag` | str | free-text category (e.g. `JSON-RPC`, `Network / TxPool`) |
| `title` | str | sentence, may contain `` `backtick` `` code spans |
| `diff` | object | `{"add": int, "del": int}` |
| `merged` | str | ISO date, `YYYY-MM-DD` |
| `description` | str | prose, may contain `` `backtick` `` code spans |

Backtick-delimited text in `title` and `description` is rendered as
`<code>…</code>`. The main page shows the entries ranked 1 to 15, best rank
first; `/prs` shows all of them, sorted automatically by newest merge date
first and by ascending PR number within the same date. Entries can go anywhere
in the array, but ranks must run 1 to N with no gaps or repeats: a new entry
either goes last as rank N, or takes an earlier rank and moves every entry from
that rank onward down by one. Run `make test` after editing the file to catch a
missing or malformed field, a duplicate rank, or a gap in the ranks.

## Deploy

GitHub Pages serves the site from `/portfolio/`. The workflow in
`.github/workflows/deploy.yml` runs on every push to `main` (and on manual
dispatch): it runs `cargo test`, builds the site with
`trunk build --release --public-url /portfolio/` and deploys the resulting
`dist/` directory through the Pages Actions source. To build the same output
locally:

```bash
make build-pages
```

The built output is in `dist/`. The Trunk post-build hook in `Trunk.toml`
writes copies of the app shell to `404.html`, `prs.html` and `prs/index.html`,
so deep links such as `/portfolio/prs` and the old `/portfolio/prs.html` return
the app instead of a Pages 404, and unknown paths render the router's Not Found
page.
