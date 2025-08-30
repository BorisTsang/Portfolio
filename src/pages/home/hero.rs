use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"Welcome to My Portfolio"</h1>
            <p>"Showcasing my skills and projects."</p>
        </section>
    }
}