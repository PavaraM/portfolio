use leptos::prelude::*;

const STACK: &[&str] = &["Linux", "Docker", "Bash", "Python", "Git", "GitHub Actions", "AWS", "Docker Compose"];

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero">
            <div class="container">
                <div class="hero-grid">
                    <div>
                        <p class="eyebrow">"DevOps Engineer · Sri Lanka"</p>
                        <h1>
                            "Building "
                            <em>"reliable"</em>
                            " infrastructure, one layer at a time."
                        </h1>
                        <p class="hero-desc">
                            "I'm Pavara — a self-taught DevOps engineer working through a 12-month roadmap (Linux → Docker → Kubernetes → CI/CD → Terraform → Cloud). I build infrastructure tooling that holds up under real use: idempotent, observable, and small enough to reason about."
                        </p>
                        <div class="hero-actions">
                            <a
                                href="https://github.com/PavaraM"
                                target="_blank"
                                rel="noopener"
                                class="btn btn-primary"
                            >
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.167 6.839 9.49.5.09.682-.218.682-.484 0-.236-.009-.866-.013-1.7-2.782.603-3.369-1.342-3.369-1.342-.454-1.155-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.532 1.03 1.532 1.03.891 1.529 2.341 1.088 2.91.832.091-.646.349-1.086.635-1.337-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.03-2.682-.103-.254-.447-1.27.098-2.646 0 0 .84-.269 2.75 1.026A9.578 9.578 0 0 1 12 6.836a9.59 9.59 0 0 1 2.504.337c1.909-1.295 2.748-1.026 2.748-1.026.546 1.376.202 2.392.1 2.646.641.698 1.028 1.59 1.028 2.682 0 3.841-2.337 4.687-4.565 4.935.359.309.678.919.678 1.852 0 1.337-.012 2.415-.012 2.743 0 .269.18.579.688.481C19.138 20.163 22 16.418 22 12c0-5.523-4.477-10-10-10z"/>
                                </svg>
                                "GitHub"
                            </a>
                            <a href="#contact" class="btn btn-ghost">"Get in touch →"</a>
                        </div>
                        <div class="stack-row">
                            <span class="stack-label">"Stack"</span>
                            <div id="stack-pills">
                                {STACK.iter().map(|&skill| {
                                    view! {
                                        <span class="stack-pill">{skill}</span>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>

                    <TerminalCard/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TerminalCard() -> impl IntoView {
    view! {
        <div class="term-card" aria-hidden="true">
            <div class="term-bar">
                <span class="term-dot"></span>
                <span class="term-dot"></span>
                <span class="term-dot"></span>
                <span class="term-title">"pavara@obsidian — ~"</span>
            </div>
            <div class="term-body">
                <div><span class="p">"$"</span> " devbox doctor"</div>
                <div><span class="c">"[INFO]"</span> " Family: debian"</div>
                <div><span class="c">"[INFO]"</span> " Distro: Ubuntu 24.04 LTS"</div>
                <div><span class="c">"[INFO]"</span> " SSH hardening: applied"</div>
                <div><span class="c">"[INFO]"</span> " Firewall (ufw): active"</div>
                <div><span class="c">"[INFO]"</span> " Docker engine: healthy"</div>
                <div><span class="s">"PASSED — 7/7 checks"</span></div>
                <div style="margin-top:14px"><span class="p">"$"</span> " fixfolder ~/Downloads"</div>
                <div><span class="s">"47 files organized ✓"</span></div>
                <div style="margin-top:14px">
                    <span class="p">"$"</span>
                    " "
                    <span class="term-cursor"></span>
                </div>
            </div>
        </div>
    }
}
