use leptos::prelude::*;

// One experience / project / education entry.
//   role, company  — headline text
//   dates          — may be "" (hidden when empty, e.g. projects)
//   bullets        — a list; renders one <li> each, skipped if empty
#[component]
pub fn ExperienceItem(
    role: &'static str,
    company: &'static str,
    dates: &'static str,
    bullets: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="experience">
            <div class="experience-head">
                <span class="role">{role}</span>
                // Render the dates span ONLY when dates is non-empty.
                // `.then(|| ...)` returns Some(view) or None; None renders nothing.
                {(!dates.is_empty()).then(|| view! { <span class="dates">{dates}</span> })}
            </div>
            <div class="company">{company}</div>
            {(!bullets.is_empty()).then(|| view! {
                <ul class="bullets">
                    {bullets.into_iter()
                        .map(|b| view! { <li>{b}</li> })
                        .collect_view()}
                </ul>
            })}
        </div>
    }
}
