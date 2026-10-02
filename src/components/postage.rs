use leptos::prelude::*;

use super::Section;

const LIVE_DEMO_URL: &str = "https://postage-seven.vercel.app";
const SOURCE_URL: &str = "https://github.com/Matgothmog/postage";

const STACK: [&str; 15] = [
    "Rust",
    "WebAssembly",
    "Leptos 0.8",
    "Trunk",
    "Tailwind 4",
    "Axum",
    "alloy",
    "Turso (libSQL)",
    "Cloudflare Worker (workers-rs)",
    "Privy",
    "World ID (IDKit)",
    "Solidity 0.8.28 / Foundry (Arc testnet)",
    "The Graph subgraph",
    "Mailgun",
    "Vercel",
];

struct Screenshot {
    src: &'static str,
    alt: &'static str,
    caption: &'static str,
}

// Relative to the page's <base href>, so the images resolve under any public URL.
const SCREENSHOTS: [Screenshot; 4] = [
    Screenshot {
        src: "assets/img/postage-01-landing.png",
        alt: "Postage landing page",
        caption: "Landing page",
    },
    Screenshot {
        src: "assets/img/postage-02-network.png",
        alt: "Postage network / mail flow view",
        caption: "Network view",
    },
    Screenshot {
        src: "assets/img/postage-03-challenge.png",
        alt: "Postage World ID personhood challenge screen",
        caption: "Personhood challenge",
    },
    Screenshot {
        src: "assets/img/postage-05-attestation-tx.png",
        alt: "Postage onchain attestation transaction",
        caption: "Onchain attestation",
    },
];

#[component]
pub fn PostageSection() -> impl IntoView {
    view! {
        <Section
            id="postage"
            title="Postage — ETHOnline 2026"
            intro=|| "An email gateway that puts a price on strangers' mail."
            alt=true
        >
            <figure class="project-banner">
                <img
                    src="assets/img/postage-cover.png"
                    alt="Postage project cover banner"
                    loading="lazy"
                />
            </figure>
            <div class="project-body">
                <PostageDescription />
                <PostageLinks />
                <ul class="stack-tags">
                    {STACK.iter().map(|tag| view! { <li>{*tag}</li> }).collect_view()}
                </ul>
            </div>
            <Gallery />
        </Section>
    }
}

#[component]
fn PostageDescription() -> impl IntoView {
    view! {
        <p>
            "Postage is an email gateway that puts a price on strangers' mail. You hand out a "
            <code>"you@usepostage.com"</code>
            " address; incoming mail is held and classified — expected mail (login codes, receipts) and mail proven human-written pass free, while unproven automated mail (marketing, newsletters) must pay a small USDC fee to reach your real inbox, and deceptive mail is never delivered. Personhood is proven with a World ID Selfie Check bound onchain to a wallet; escrow, pricing signatures, and settlement run on the Arc testnet, with The Graph indexing payments and verdicts. Rewritten in Rust end to end — a Leptos/WebAssembly front end, an Axum API, and a Rust Cloudflare email worker. Built during ETHOnline 2026 as a proof of concept."
        </p>
    }
}

#[component]
fn PostageLinks() -> impl IntoView {
    view! {
        <div class="project-links">
            <a class="btn btn-primary" href=LIVE_DEMO_URL target="_blank" rel="noopener noreferrer">
                "Live demo"
            </a>
            <a class="btn btn-outline" href=SOURCE_URL target="_blank" rel="noopener noreferrer">
                "Source code"
            </a>
        </div>
    }
}

#[component]
fn Gallery() -> impl IntoView {
    let items = SCREENSHOTS
        .iter()
        .map(|shot| {
            view! {
                <figure class="gallery-item">
                    <img src=shot.src alt=shot.alt loading="lazy" />
                    <figcaption>{shot.caption}</figcaption>
                </figure>
            }
        })
        .collect_view();

    view! { <div class="gallery">{items}</div> }
}
