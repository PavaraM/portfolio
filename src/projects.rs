use leptos::prelude::*;

struct Project {
    name: &'static str,
    r#type: &'static str,
    desc: &'static str,
    tags: &'static [&'static str],
    stats: &'static [&'static str],
    url: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        name: "cicd-demo",
        r#type: "ci/cd",
        desc: "Flask API with a full CI/CD pipeline — ruff lint, pytest, and Trivy scans gate a multi-stage Docker build, then a health-gated blue/green deploy ships it to EC2 with zero downtime. Actions pinned to immutable SHAs.",
        tags: &["python", "flask", "docker", "github-actions", "trivy", "ec2"],
        stats: &["3 jobs · 5 gates", "SHA-pinned actions", "Blue/green · zero downtime"],
        url: "https://github.com/PavaraM/cicd-demo",
    },
    Project {
        name: "DevBox",
        r#type: "bash · infra",
        desc: "Infrastructure-as-Code for dev environments, on any Linux distro. Idempotent provisioning, Docker + Compose setup, SSH hardening, UFW firewall, deploy-user creation — driven by 10 composable profiles, 8 lifecycle hooks, and a zero-dependency curl|sh installer.",
        tags: &["bash", "linux", "docker", "ufw", "ssh", "iac"],
        stats: &["10 distros · 19 exit codes", "10 profiles · 8 hooks", "Idempotent · dry-run"],
        url: "https://github.com/PavaraM/devbox",
    },
    Project {
        name: "bash-logger",
        r#type: "library",
        desc: "Reusable structured logging library for Bash — DEBUG/INFO/WARN/ERROR levels, colored console output, timestamped file logs, retention-based auto-archival, and sudo-aware file ownership with no load-time side effects.",
        tags: &["bash", "logging", "library", "portable"],
        stats: &["Zero dependencies", "4 log levels", "Auto-archive"],
        url: "https://github.com/PavaraM/bash-logger",
    },
    Project {
        name: "fixfolder",
        r#type: "bash · utility",
        desc: "Sorts any directory into 17 categorized subfolders — images, documents, archives, code, config, and more. Recognizes 100+ extensions with safe move validation, an audit-trail log, and real-time progress summaries.",
        tags: &["bash", "filesystem", "cli", "linux"],
        stats: &["17 categories", "100+ extensions", "Audit-trail logs"],
        url: "https://github.com/PavaraM/Smart-File-Organizer",
    },
];

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section class="section" id="projects">
            <div class="container">
                <p class="section-label reveal">"Selected Work"</p>
                <h2 class="reveal">"Projects"</h2>
                <div class="projects-grid">
                    {PROJECTS.iter().enumerate().map(|(i, p)| {
                        let delay = format!("{}ms", i * 70);
                        let type_tag = (!p.r#type.is_empty()).then(|| {
                            view! { <span class="mono-tag">{p.r#type}</span> }
                        });
                        view! {
                            <a
                                href=p.url
                                target="_blank"
                                rel="noopener"
                                class="project-card reveal"
                                style:transition-delay=delay
                            >
                                <div class="project-top">
                                    <div class="project-name">
                                        {p.name}
                                        {type_tag}
                                    </div>
                                    <span class="card-arrow">"↗"</span>
                                </div>
                                <p class="project-desc">{p.desc}</p>
                                <div class="project-tags">
                                    {p.tags.iter().map(|&tag| {
                                        view! { <span class="tag">{tag}</span> }
                                    }).collect_view()}
                                </div>
                                <div class="project-stats">
                                    {p.stats.iter().map(|&stat| {
                                        view! { <span>{stat}</span> }
                                    }).collect_view()}
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
