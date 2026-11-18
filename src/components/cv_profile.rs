use leptos::prelude::*;

use super::Section;

const NETHERMIND_PRS_URL: &str =
    "https://github.com/NethermindEth/nethermind/pulls?q=author%3AMatgothmog";
const EF_BOUNTY_URL: &str = "https://ethereum.org/bug-bounty/#leaderboard";

struct SkillArea {
    label: &'static str,
    summary: &'static str,
}

const SKILLS: [SkillArea; 4] = [
    SkillArea {
        label: "Blockchain Security",
        summary: "EVM state tests, DoS & input-validation hardening, coordinated disclosure.",
    },
    SkillArea {
        label: "Low-Level & Systems",
        summary: "C, C#/.NET, Rust, x86/ARM assembly, memory management, concurrency.",
    },
    SkillArea {
        label: "Cybersecurity",
        summary: "Penetration testing, vulnerability assessment, reverse engineering.",
    },
    SkillArea {
        label: "Tooling & Languages",
        summary: "Linux, Python, Git, Docker, Ansible, Pentest suite · French (Native), English (TOEFL iBT 101, TOEIC 970).",
    },
];

struct ExperienceItem {
    period: &'static str,
    role: &'static str,
    org: &'static str,
    location: &'static str,
    summary: &'static str,
    url: Option<&'static str>,
    tags: &'static [&'static str],
}

const EXPERIENCES: [ExperienceItem; 6] = [
    ExperienceItem {
        period: "Sep. 2026 — present",
        role: "Open-Source Contributor",
        org: "Nethermind — Ethereum execution client",
        location: "Remote",
        summary: "Merged pull requests to the Nethermind client: Engine API fork-gating spec fix; P2P DoS hardening (oversized transactions dropped pre-decode, repeated devp2p Hello rejected); tracing and JSON-RPC fixes.",
        url: Some(NETHERMIND_PRS_URL),
        tags: &["C#", ".NET", "Ethereum", "JSON-RPC", "DoS hardening"],
    },
    ExperienceItem {
        period: "2026 — present",
        role: "Bug Bounty Researcher",
        org: "Ethereum Foundation — Bug Bounty Program",
        location: "Remote",
        summary: "Vulnerability research on Ethereum clients and protocol specifications; ranked 60th of 62 on the Consensus Layer leaderboard (250 pts) and 85th of 100 on the Execution Layer leaderboard (1,000 pts).",
        url: Some(EF_BOUNTY_URL),
        tags: &[
            "Vulnerability research",
            "Protocol security",
            "Coordinated disclosure",
        ],
    },
    ExperienceItem {
        period: "2024 — present",
        role: "Cybersecurity Pentester",
        org: "ENSIMAG — Grenoble INP, UGA",
        location: "Grenoble, France",
        summary: "Conduct authorised penetration tests on the school's internal infrastructure and student-facing services; report vulnerabilities and recommend remediations to harden the systems.",
        url: None,
        tags: &[
            "Pentesting",
            "Network security",
            "Web security",
            "Reporting",
        ],
    },
    ExperienceItem {
        period: "2024 — present",
        role: "Student IT-Fleet Manager",
        org: "ENSIMAG — Grenoble INP, UGA",
        location: "Grenoble, France",
        summary: "Administer and maintain the student computer fleet: hardware, OS deployment, inventory, troubleshooting and on-site user support.",
        url: None,
        tags: &["Linux", "Sysadmin", "Deployment", "Support"],
    },
    ExperienceItem {
        period: "Feb. 2026 — Jun. 2026",
        role: "R&D Research Intern",
        org: "Inria — Morpheo Team",
        location: "Grenoble, France",
        summary: "Built phase one of Kinoflex, a low-cost 4D volumetric capture platform: contributed to the core pipeline and to prototype validation.",
        url: None,
        tags: &["Imaging", "Computer vision", "Prototyping", "C", "Python"],
    },
    ExperienceItem {
        period: "May 2025 — Aug. 2025",
        role: "R&D Engineer Intern",
        org: "Pixminds",
        location: "Chambéry, France",
        summary: "Developed software turning ordinary photographs into complete 3D CAD models built on state-of-the-art AI 3D reconstruction, and researched LLM-driven parametric design producing editable CAD parameters for industrialisation.",
        url: None,
        tags: &["3D reconstruction", "Generative AI", "LLMs", "CAD"],
    },
];

/// Education & technical capabilities derived from Matteo Pecher's CV.
#[component]
pub fn SplitAboutSection() -> impl IntoView {
    let skill_cells = SKILLS
        .iter()
        .enumerate()
        .map(|(index, skill)| {
            let num = format!("{:02}", index + 1);
            view! {
                <article class="spec-cell">
                    <span class="spec-index">{num}</span>
                    <h3>{skill.label}</h3>
                    <p>{skill.summary}</p>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="about"
            eyebrow="Profile // Education & Skills"
            title="Matteo Pecher"
            intro=|| "Bug-bounty researcher in the Ethereum Foundation program and open-source contributor to the Nethermind Ethereum client. Specialised in computer science from low-level assembly up to blockchain protocol security — C and Rust enthusiast with hands-on vulnerability-research experience."
        >
            <div class="edu-card">
                <div class="edu-meta">
                    <span class="edu-period">"2023 — present"</span>
                    <span class="edu-badge">"ENSIMAG · M2 Cysec"</span>
                </div>
                <h3>
                    "Engineering Degree (M2) — Double degree in Cybersecurity (Cysec)"
                </h3>
                <p class="edu-school">"ENSIMAG (Grenoble INP) & Université Grenoble Alpes"</p>
                <p class="edu-note">
                    "ENSIMAG is ranked 1st computer-science engineering school in France (Le Figaro Étudiant & L'Étudiant 2025–2026) and in the Top 150 worldwide in Computer Science & Engineering (Shanghai Ranking, since 2019)."
                </p>
            </div>
            <div class="spec-matrix spec-matrix-2">{skill_cells}</div>
        </Section>
    }
}

/// Interactive timeline of research, open-source, security, and R&D roles.
#[component]
pub fn SplitExperienceSection() -> impl IntoView {
    let items = EXPERIENCES
        .iter()
        .map(|exp| {
            let title_view = match exp.url {
                Some(url) => view! {
                    <a class="exp-link" href=url target="_blank" rel="noopener noreferrer">
                        <span>{format!("{} · {}", exp.role, exp.org)}</span>
                        <span class="exp-arrow" aria-hidden="true">" ↗"</span>
                    </a>
                }
                .into_any(),
                None => view! {
                    <span>{format!("{} · {}", exp.role, exp.org)}</span>
                }
                .into_any(),
            };

            view! {
                <article class="exp-item">
                    <div class="exp-aside">
                        <span class="exp-period">{exp.period}</span>
                        <span class="exp-location">{exp.location}</span>
                    </div>
                    <div class="exp-body">
                        <h3>{title_view}</h3>
                        <p>{exp.summary}</p>
                        <ul class="exp-tags">
                            {exp.tags.iter().map(|tag| view! { <li>{*tag}</li> }).collect_view()}
                        </ul>
                    </div>
                </article>
            }
        })
        .collect_view();

    view! {
        <Section
            id="experience"
            eyebrow="Timeline // 2024 — 2026"
            title="Experience"
            intro=|| "Client protocol hardening, vulnerability research, authorised penetration testing, and R&D systems engineering."
        >
            <div class="exp-list">{items}</div>
        </Section>
    }
}
