use leptos::*;
use crate::pages::home::{Hero, Technologies, Projects};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Hero/>
        <Technologies/>
        <Projects/>
    }
}