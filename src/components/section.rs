use leptos::prelude::*;

// A titled section wrapper. `children` = whatever is nested inside the tags
// when you write <Section title="...">...</Section>.
#[component]
pub fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="section">
            <h2>{title}</h2>
            {children()}
        </section>
    }
}
