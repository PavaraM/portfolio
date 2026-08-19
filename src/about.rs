use leptos::prelude::*;

struct Fact {
    key: &'static str,
    val: &'static str,
}

const FACTS: &[Fact] = &[
    Fact { key: "Location", val: "Sri Lanka (open to remote)" },
    Fact { key: "Machine", val: "OBSIDIAN · Ubuntu 24.04 · kernel 6.17" },
    Fact { key: "Current focus", val: "Docker · CI/CD · Linux internals" },
    Fact { key: "Currently", val: "AWS Cloud Practitioner · in progress" },
    Fact { key: "GitHub", val: "@PavaraM" },
    Fact { key: "Status", val: "Open to opportunities" },
];

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="section" id="about">
            <div class="container">
                <p class="section-label reveal">"Background"</p>
                <h2 class="reveal">"About"</h2>
                <div class="about-grid">
                    <div class="about-text reveal">
                        <p>
                            "I started my DevOps journey on "
                            <strong>"January 1, 2026"</strong>
                            ", with no formal degree — just a structured 12-month roadmap and a preference for learning by building real things."
                        </p>
                        <p>
                            "My work is grounded in "
                            <strong>"Linux systems, Bash automation, and containerization"</strong>
                            ". I care about the small engineering details: clean exit codes, structured logs, modular architecture, minimal surface area."
                        </p>
                        <p>
                            "I'm working toward a "
                            <strong>"remote junior DevOps or Platform Engineer role"</strong>
                            ", building infrastructure tools that reflect how I think about systems — simply, clearly, and with care."
                        </p>
                    </div>
                    <ul class="fact-list reveal" style="transition-delay:.1s">
                        {FACTS.iter().map(|f| {
                            view! {
                                <li class="fact-item">
                                    <span class="fact-key">{f.key}</span>
                                    <span class="fact-val">{f.val}</span>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            </div>
        </section>
    }
}
