use leptos::prelude::*;
use leptos_router::{components::{Router, Route, Routes}, path};
use leptos_meta::*;

use crate::pages::{notfound::NotFound, home::HomePage, projects::Projects};
use crate::components::{header::Header, footer::Footer};

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

        <Router>
            <div class="app">
                <Header/>
                <main class="main-content">
                    <Routes fallback=NotFound>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/projects") view=Projects/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}