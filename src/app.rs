use leptos::prelude::*;
use leptos_meta::*;
// TODO: add routers

use crate::pages::{home::HomePage, projects::Projects};
use crate::components::{header::Header, contact::Contact, footer::Footer};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // TODO: add signal for title
    view! {
        <Title text="Boris Tsang"/>
        <Script>
            "function smoothScrollTo(targetId) {
                const element = document.getElementById(targetId);
                if (element) {
                    element.scrollIntoView({ behavior: 'smooth', block: 'start' });
                }
            }"
        </Script>

        <Header/>
        <HomePage/>
        <Contact/>
        <Footer/>
    }
}

