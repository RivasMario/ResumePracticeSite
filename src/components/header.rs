use leptos::prelude::*;

// Top banner: name, target titles, location, contact links, and a small
// toolbar (dark/light toggle + print). Ends with a decorative PNW mountain
// skyline SVG.
//
// Privacy: phone number and CJIS/DoD specifics are intentionally omitted —
// this is a PUBLIC site. Only LinkedIn-safe info is shown.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="header-top">
                <div>
                    <h1>"Mario J. Rivas"</h1>
                    <p class="tagline">
                        "Cloud Support Engineer | Site Reliability | IT Security Management"
                    </p>
                    <p class="meta">
                        "Seattle, WA \u{2022} Pacific Northwest \u{2022} Active TS/SCI Clearance"
                    </p>
                </div>
                <Toolbar/>
            </div>

            <nav class="links">
                <a href="mailto:mariojrivas@outlook.com">"Email"</a>
                <a href="https://github.com/RivasMario" target="_blank">"GitHub"</a>
                <a href="https://www.linkedin.com/in/" target="_blank">"LinkedIn"</a>
            </nav>

            <Skyline/>
        </header>
    }
}

// Small button cluster. Shows the first piece of REAL interactivity:
// a signal (reactive state) driving the theme.
#[component]
fn Toolbar() -> impl IntoView {
    // `signal(true)` = reactive state, starts dark.
    // dark      -> read the value (a getter)
    // set_dark  -> change the value (a setter); any view reading `dark` updates.
    let (dark, set_dark) = signal(true);

    // `Effect` re-runs whenever the signals it reads change. Here: push the
    // theme onto <html data-theme="..."> so CSS can swap color variables.
    Effect::new(move |_| {
        let theme = if dark.get() { "dark" } else { "light" };
        if let Some(el) = document().document_element() {
            let _ = el.set_attribute("data-theme", theme);
        }
    });

    view! {
        <div class="toolbar">
            <button
                class="icon-btn"
                title="Toggle theme"
                on:click=move |_| set_dark.update(|d| *d = !*d)
            >
                // Reading `dark.get()` inside a closure makes this text reactive.
                {move || if dark.get() { "\u{2600}\u{FE0F}" } else { "\u{1F319}" }}
            </button>
            <button
                class="icon-btn"
                title="Print / Save as PDF"
                on:click=move |_| { let _ = window().print(); }
            >
                "\u{1F5A8}\u{FE0F}"
            </button>
        </div>
    }
}

// Decorative Pacific-Northwest mountain skyline (inline SVG).
#[component]
fn Skyline() -> impl IntoView {
    view! {
        <svg class="skyline" viewBox="0 0 1200 120" preserveAspectRatio="none" aria-hidden="true">
            // Back ridge (faint)
            <path class="ridge-back"
                d="M0,120 L0,70 L180,30 L340,75 L520,25 L700,80 L880,35 L1060,70 L1200,40 L1200,120 Z"/>
            // Front ridge (evergreen)
            <path class="ridge-front"
                d="M0,120 L0,95 L150,55 L300,95 L470,50 L640,100 L820,60 L1000,95 L1200,65 L1200,120 Z"/>
        </svg>
    }
}
