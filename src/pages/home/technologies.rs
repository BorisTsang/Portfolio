use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Technologies() -> impl IntoView {
    let css = style! {
        .section {
            padding: var(--spacing-3xl) 0;
            background: var(--color-bg);
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 var(--spacing-md);
        }

        .title {
            font-size: clamp(2.5rem, 5vw, 4rem);
            font-weight: 700;
            color: var(--color-text-primary);
            margin-bottom: var(--spacing-sm);
            text-align: left;
        }

        .subtitle {
            font-size: 1rem;
            color: var(--color-text-secondary);
            margin-bottom: var(--spacing-2xl);
            max-width: 800px;
        }

        .tech-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); /* Slightly wider cards */
            grid-template-rows: auto;
            gap: var(--spacing-xl); /* More spacing between cards */
            align-items: stretch;
        }

        .tech-card {
            background: linear-gradient(135deg, rgba(255, 255, 255, 0.8), rgba(240, 248, 255, 0.8)); /* Light gradient for multi-color effect */
            border-radius: var(--radius-md);
            padding: var(--spacing-md);
            display: flex;
            align-items: center;
            gap: var(--spacing-md);
            box-shadow: var(--shadow-md);
            transition: transform 0.2s ease;
            height: 100%;
            border: 1px solid var(--color-border); /* Consistent light border */
        }

        .tech-card:hover {
            transform: translateY(-2px);
            box-shadow: var(--shadow-lg);
        }

        .tech-logo {
            width: 40px;
            height: 40px;
            object-fit: contain;
        }

        .tech-info {
            display: flex;
            flex-direction: column;
        }

        .tech-name {
            font-size: 1.125rem;
            font-weight: 600;
            color: var(--color-text-primary);
        }

        .tech-desc {
            font-size: 0.875rem;
            color: var(--color-text-tertiary);
        }

        .fade-in {
            animation: fadeIn 0.5s ease-in-out forwards;
            opacity: 0;
        }

        @keyframes fadeIn {
            from {
                opacity: 0;
            }
            to {
                opacity: 1;
            }
        }

        @media (max-width: 768px) {
            .tech-grid {
                grid-template-columns: 1fr; /* Stack cards on small screens */
            }

            .tech-card {
                padding: var(--spacing-sm); /* Reduce padding on mobile */
            }
        }
    };

    view! { class=css,
        <section class="section" id="technologies">
            <div class="container">
                <h2 class="title">"Current technologies"</h2>
                <p class="subtitle">"I'm proficient in a range of modern technologies that empower me to build highly functional solutions. These are some of my main technologies."</p>
                <div class="tech-grid">
                    // Primary Skills
                    <div class="tech-card">
                        <img src="https://cdn.simpleicons.org/rust" alt="Rust" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Rust"</span>
                            <span class="tech-desc">"Systems programming language"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg" alt="Python" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Python"</span>
                            <span class="tech-desc">"General-purpose programming"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/csharp/csharp-original.svg" alt="C#" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"C#"</span>
                            <span class="tech-desc">"Object-oriented language"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/html5/html5-original.svg" alt="HTML" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"HTML"</span>
                            <span class="tech-desc">"Markup language"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/css3/css3-original.svg" alt="CSS" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"CSS"</span>
                            <span class="tech-desc">"Styling language"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/javascript/javascript-original.svg" alt="JS" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"JavaScript"</span>
                            <span class="tech-desc">"Scripting language"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.simpleicons.org/leptos" alt="Leptos" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Leptos"</span>
                            <span class="tech-desc">"Rust web framework"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/nodejs/nodejs-original.svg" alt="Node.js" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Node.js"</span>
                            <span class="tech-desc">"JavaScript runtime"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/supabase/supabase-original.svg" alt="Supabase" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Supabase"</span>
                            <span class="tech-desc">"Backend-as-a-Service"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/sqlite/sqlite-original.svg" alt="SQL" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"SQL"</span>
                            <span class="tech-desc">"Database querying language"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/unity/unity-original.svg" alt="Unity" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Unity"</span>
                            <span class="tech-desc">"Game development engine"</span>
                        </div>
                    </div>
                    <div class="tech-card">
                        <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/linux/linux-original.svg" alt="Linux" class="tech-logo" />
                        <div class="tech-info">
                            <span class="tech-name">"Linux"</span>
                            <span class="tech-desc">"Basic knowledge"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}