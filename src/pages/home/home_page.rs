use leptos::prelude::*;
use crate::pages::home::{Hero, Technologies, Projects, Contact};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home">
            <Hero/>
            <Technologies/>
            <Projects/>
            <Contact/>
        </div>
    }
}