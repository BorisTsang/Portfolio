use leptos::prelude::*;
use leptos_router::components::A;
use stylers::style;

#[component]
pub fn Hero() -> impl IntoView {
    let css = style! {
        .container {
            max-width: 1280px;
            margin: 0 auto;
            padding: 0 var(--spacing-lg);
        }

        .hero {
            min-height: 100vh;
            display: flex;
            align-items: center;
            background: radial-gradient(circle at top center, rgba(59, 130, 246, 0.1) 0%, rgba(255, 255, 255, 0.95) 70%);
            overflow: hidden;
        }

        .hero-content {
            max-width: 900px;
            text-align: center;
            margin: 0 auto;
            padding: var(--spacing-xl) var(--spacing-lg);
        }

        .hero-badge {
            display: inline-flex;
            align-items: center;
            gap: var(--spacing-sm);
            background: var(--color-surface-elevated);
            border: 1px solid var(--color-border);
            border-radius: var(--radius-xl);
            padding: var(--spacing-sm) var(--spacing-lg);
            font-size: 0.875rem;
            color: var(--color-text-secondary);
            margin-bottom: var(--spacing-xl);
            transition: transform 0.2s ease;
        }

        .hero-badge:hover {
            transform: scale(1.05);
        }

        .hero-title {
            font-size: clamp(2rem, 6vw, 4rem); /* Smaller base size for non-gradient text */
            font-weight: 800;
            line-height: 1.2;
            margin-bottom: var(--spacing-lg);
            color: var(--color-text-primary);
        }

        .hero-title .text-gradient {
            font-size: clamp(2.5rem, 7vw, 5rem); /* Larger size for gradient text */
            font-weight: 900; /* Slightly bolder for emphasis */
            background: linear-gradient(135deg, var(--color-gradient-start), var(--color-gradient-end));
            color: transparent;
            background-clip: text;
        }

        .hero-subtitle {
            font-size: clamp(1rem, 1.8vw, 1.25rem);
            color: var(--color-text-secondary);
            line-height: 1.7;
            max-width: 700px;
            margin: 0 auto var(--spacing-2xl);
        }

        .hero-actions {
            display: flex;
            gap: var(--spacing-xl);
            justify-content: center;
            flex-wrap: wrap;
        }
    };

    view! { class=css,
        <section id="hero" class="hero">
            <div class="container">
                <div class="hero-content">
                    <div class="hero-badge">
                        <span>"HongKonger"</span>
                    </div>
                    <h1 class="hero-title">
                        "Building "
                        <span class="text-gradient">"Secure "</span>
                        "& "
                        <span class="text-gradient">"Interactive "</span>
                        "Tech"
                    </h1>
                    <p class="hero-subtitle">
                        "I’m a young tech enthusiast creating secure embedded systems and engaging games."
                    </p>
                    <div class="hero-actions">
                        <A href="#projects" {..} class="btn btn-primary">"See My Work"</A>
                        <A href="#contact" {..} class="btn btn-secondary">"Get In Touch"</A>
                    </div>
                </div>
            </div>
        </section>
    }
}