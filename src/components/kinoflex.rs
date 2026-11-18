use leptos::prelude::*;

use super::Section;

const REPORT_URL: &str = "https://github.com/Matgothmog/kinoflex/blob/main/report.pdf";

const STACK: [&str; 12] = [
    "Python",
    "PySide6 / Qt",
    "Raspberry Pi 5",
    "Raspberry Pi Pico (RP2040)",
    "MQTT / Mosquitto",
    "Ansible",
    "SQLite",
    "Zstandard",
    "libcamera",
    "systemd",
    "pytest",
    "chrony",
];

struct Pillar {
    title: &'static str,
    body: &'static str,
}

const PILLARS: [Pillar; 4] = [
    Pillar {
        title: "Hardware synchronisation",
        body: "A Raspberry Pi Pico generates a 60 Hz trigger using a PIO state machine and DMA at 100 ns resolution, star-wired to every IMX296 global-shutter sensor so all cameras expose together.",
    },
    Pillar {
        title: "Real-time lossless compression",
        body: "A multi-process Python capture pipeline writes frame-delta + Zstandard blocks straight to NVMe, reaching 3x-6x compression and keeping each node within its write budget (~50 MB/s in the worst case, from a ~280 MB/s raw stream).",
    },
    Pillar {
        title: "Tested against failure",
        body: "Around 190 tests — unit, live-fleet and chaos — exercise the system, including crashes mid-acquisition, rapid start/stop and node reboots; they surfaced a recurring NVMe write-stall and an upstream libcamera crash that its maintainers reproduced and fixed.",
    },
    Pillar {
        title: "LED-probe evaluation methodology",
        body: "A second Pico drives a 23-LED panel as an independent optical ground truth, comparing the hardware trigger against free-run, NTP and libcamera master/slave; the trigger is expected to hold cameras within sub-microsecond of each other versus about 1 ms for software-only methods.",
    },
];

#[component]
pub fn KinoflexSection() -> impl IntoView {
    let specs = PILLARS
        .iter()
        .enumerate()
        .map(|(index, pillar)| {
            let num = format!("{:02}", index + 1);
            view! {
                <article class="spec-cell">
                    <span class="spec-index">{num}</span>
                    <h3>{pillar.title}</h3>
                    <p>{pillar.body}</p>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="inria-kinoflex"
            eyebrow="04 // Embedded & Systems"
            title="Inria — Kinoflex: synchronised multi-camera capture"
            intro=|| "During my M1 MoSIG research internship in Inria Grenoble's Morpheo team, I built the first stage of Kinoflex — a low-cost, open-source rig of Raspberry Pi 5 cameras for synchronised 4D (volumetric) video capture, and an affordable, portable alternative to large fixed studios. A Raspberry Pi Pico broadcasts a hardware trigger to every camera; each node compresses its stream losslessly in real time; and a single operator drives the whole fleet from one GUI. This report covers a 10-node fleet, the foundation for a planned 80-camera system."
            alt=true
        >
            <figure class="window-frame figure-strip">
                <div class="window-chrome" aria-hidden="true">
                    <span class="window-dots">
                        <span></span><span></span><span></span>
                    </span>
                    <span class="window-title">"kinoflex-rig · 10-node RPi5 + IMX296 cluster"</span>
                </div>
                <img
                    src="assets/img/kinoflex-rig.png"
                    alt="Row of ten Raspberry Pi 5 cameras mounted on tripods, wired with Ethernet and power cables, forming the Kinoflex acquisition rig"
                    loading="lazy"
                />
            </figure>
            <div class="spec-matrix spec-matrix-2">{specs}</div>
            <figure class="window-frame figure-strip figure-sync">
                <div class="window-chrome" aria-hidden="true">
                    <span class="window-dots">
                        <span></span><span></span><span></span>
                    </span>
                    <span class="window-title">"led-probe · optical ground truth"</span>
                </div>
                <img
                    src="assets/img/kinoflex-sync.png"
                    alt="Kinoflex LED panel next to nine cameras' simultaneous captures of the same LED pattern under the hardware trigger"
                    loading="lazy"
                />
                <figcaption class="window-caption">
                    "Nine cameras capturing the same LED frame under the hardware trigger."
                </figcaption>
            </figure>
            <div class="project-footer project-footer-inline">
                <KinoflexLinks />
                <ul class="stack-tags">
                    {STACK.iter().map(|tag| view! { <li>{*tag}</li> }).collect_view()}
                </ul>
            </div>
        </Section>
    }
}

#[component]
fn KinoflexLinks() -> impl IntoView {
    view! {
        <div class="project-links">
            <a class="btn btn-green" href=REPORT_URL target="_blank" rel="noopener noreferrer">
                "Report (PDF)"
            </a>
        </div>
    }
}
