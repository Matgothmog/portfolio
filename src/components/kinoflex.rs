use leptos::prelude::*;

use super::Section;

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

const PILLARS: [Pillar; 6] = [
    Pillar {
        title: "Hardware synchronisation",
        body: "A Raspberry Pi Pico generates a 60 Hz trigger using a PIO state machine and DMA at 100 ns resolution, star-wired to every IMX296 global-shutter sensor so all cameras expose together.",
    },
    Pillar {
        title: "Real-time lossless compression",
        body: "A multi-process Python capture pipeline writes frame-delta + Zstandard blocks straight to NVMe, reaching 3x-6x compression and keeping each node within its write budget (~50 MB/s in the worst case, from a ~280 MB/s raw stream).",
    },
    Pillar {
        title: "Single-operator fleet supervisor",
        body: "A PySide6 GUI driven by MQTT telemetry, with 14 auto-discovered pages for live previews, health metrics and recordings, backed by a SQLite session catalogue and parallel SCP export with SHA-256 verification.",
    },
    Pillar {
        title: "One-command fleet provisioning",
        body: "An idempotent Ansible playbook brings every Raspberry Pi to the same known-good state — installing and pinning the camera stack, mounting the NVMe, applying the kernel and PCIe tweaks that eliminate write stalls, and installing the capture service — so the whole fleet can be rebuilt or extended from one command.",
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
    let cards = PILLARS
        .iter()
        .map(|pillar| {
            view! {
                <article class="card">
                    <h3>{pillar.title}</h3>
                    <p class="card-body">{pillar.body}</p>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="inria-kinoflex"
            title="Inria — Kinoflex: synchronised multi-camera capture"
            intro=|| "During my M1 MoSIG research internship in Inria Grenoble's Morpheo team, I built the first stage of Kinoflex — a low-cost, open-source rig of Raspberry Pi 5 cameras for synchronised 4D (volumetric) video capture, and an affordable, portable alternative to large fixed studios. A Raspberry Pi Pico broadcasts a hardware trigger to every camera; each node compresses its stream losslessly in real time; and a single operator drives the whole fleet from one GUI. This report covers a 10-node fleet, the foundation for a planned 80-camera system."
            alt=true
        >
            <figure class="project-banner">
                <img
                    src="assets/img/kinoflex-rig.png"
                    alt="Row of ten Raspberry Pi 5 cameras mounted on tripods, wired with Ethernet and power cables, forming the Kinoflex acquisition rig"
                    loading="lazy"
                />
            </figure>
            <div class="card-grid">{cards}</div>
            <figure class="figure-contain">
                <img
                    src="assets/img/kinoflex-sync.png"
                    alt="Kinoflex LED panel next to nine cameras' simultaneous captures of the same LED pattern under the hardware trigger"
                    loading="lazy"
                />
                <figcaption>"Nine cameras capturing the same LED frame under the hardware trigger."</figcaption>
            </figure>
            <ul class="stack-tags">
                {STACK.iter().map(|tag| view! { <li>{*tag}</li> }).collect_view()}
            </ul>
        </Section>
    }
}
