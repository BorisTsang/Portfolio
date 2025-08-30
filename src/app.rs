use leptos::prelude::*;
use leptos_meta::*;
/*
use crate::components::{
    contact::Contact,
    header::Header,
    projects::Projects,
};
*/

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // TODO: add signal for title
    view! {
        <Title text="Boris Tsang"/>
        <h1>Welcome!</h1>
    }
}