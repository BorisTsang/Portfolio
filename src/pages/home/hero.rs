use leptos::prelude::*;
use leptos_router::components::A;
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero" class="hero">
            <div class="container">
                <div class="hero-content">
                    <div class="hero-badge fade-in-element">
                        <span>"Based In USA"</span>
                    </div>
                    <h1 class="hero-title fade-in-element">
                        "Quality"<span class="text-gradient">"Design"</span>"&"
                        <span class="text-gradient">"WebAssembly"</span>"Development"
                        <span class="text-gradient">"Synergy"</span>
                    </h1>
                    <p class="hero-subtitle fade-in-element">
                        "Hi, I'm a developer who creates intuitive, visually stunning and highly functional web applications with modern technologies including Rust and WebAssembly."
                    </p>
                    <div class="hero-actions fade-in-element">
                        <A href="#projects" {..} class="btn btn-primary">"See My Work"</A>
                        <A href="#contact" {..} class="btn btn-secondary">"Get In Touch"</A>
                    </div>
                </div>
            </div>
        </section>
    }
}