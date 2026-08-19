use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section class="section section-last" id="contact">
            <div class="container">
                <div class="contact-box reveal">
                    <div>
                        <p class="contact-title">
                            "Let's work "
                            <span>"together"</span>
                            "."
                        </p>
                        <p class="contact-sub">
                            "Open to remote DevOps, platform, or infrastructure roles. Feel free to reach out — I respond promptly."
                        </p>
                    </div>
                    <div class="contact-links">
                        <a
                            href="https://github.com/PavaraM"
                            target="_blank"
                            rel="noopener"
                            class="contact-link"
                        >
                            <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.167 6.839 9.49.5.09.682-.218.682-.484 0-.236-.009-.866-.013-1.7-2.782.603-3.369-1.342-3.369-1.342-.454-1.155-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.532 1.03 1.532 1.03.891 1.529 2.341 1.088 2.91.832.091-.646.349-1.086.635-1.337-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.03-2.682-.103-.254-.447-1.27.098-2.646 0 0 .84-.269 2.75 1.026A9.578 9.578 0 0 1 12 6.836a9.59 9.59 0 0 1 2.504.337c1.909-1.295 2.748-1.026 2.748-1.026.546 1.376.202 2.392.1 2.646.641.698 1.028 1.59 1.028 2.682 0 3.841-2.337 4.687-4.565 4.935.359.309.678.919.678 1.852 0 1.337-.012 2.415-.012 2.743 0 .269.18.579.688.481C19.138 20.163 22 16.418 22 12c0-5.523-4.477-10-10-10z"/>
                            </svg>
                            "github.com/PavaraM"
                        </a>
                        <a href="mailto:pavaramirihagalla@icloud.com" class="contact-link">
                            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                                <rect x="2" y="4" width="20" height="16" rx="2"/>
                                <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>
                            </svg>
                            "pavaramirihagalla@icloud.com"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
