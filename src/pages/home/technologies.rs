use leptos::prelude::*;

#[component]
pub fn Technologies() -> impl IntoView {
    view! {
        <section class="section" id="technologies">
            <div class="container">
                <h2 class="section-title fade-in-element">"Current technologies"</h2>
                <p class="fade-in-element" style="text-align: center; color: var(--color-text-secondary); margin-bottom: var(--spacing-2xl); font-size: 1.125rem;">
                    "I'm proficient in a range of modern technologies that empower me to build highly functional solutions. These are some of my main technologies."
                </p>
                <div class="tech-grid">
                    <div class="tech-card fade-in-left">
                        <div class="tech-icon">"🦀"</div>
                        <h3 class="tech-name">"Rust"</h3>
                        <p class="tech-description">"Systems programming language"</p>
                    </div>
                    <div class="tech-card fade-in-element">
                        <div class="tech-icon">"🕸️"</div>
                        <h3 class="tech-name">"WebAssembly"</h3>
                        <p class="tech-description">"High-performance web runtime"</p>
                    </div>
                    <div class="tech-card fade-in-right">
                        <div class="tech-icon">"⚡"</div>
                        <h3 class="tech-name">"Leptos"</h3>
                        <p class="tech-description">"Modern Rust web framework"</p>
                    </div>
                    <div class="tech-card fade-in-left">
                        <div class="tech-icon">"🎨"</div>
                        <h3 class="tech-name">"CSS"</h3>
                        <p class="tech-description">"Modern styling and design"</p>
                    </div>
                    <div class="tech-card fade-in-element">
                        <div class="tech-icon">"🔥"</div>
                        <h3 class="tech-name">"JavaScript"</h3>
                        <p class="tech-description">"Dynamic web interactions"</p>
                    </div>
                    <div class="tech-card fade-in-right">
                        <div class="tech-icon">"🐍"</div>
                        <h3 class="tech-name">"Python"</h3>
                        <p class="tech-description">"Backend and automation"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}