use leptos::prelude::*;
use serde::Deserialize;

// A repo as returned by the GitHub REST API. We only declare the fields we
// use — serde ignores everything else in the JSON automatically.
//   Option<String> = the field may be null (e.g. a repo with no description).
#[derive(Clone, Debug, Deserialize)]
struct Repo {
    name: String,
    description: Option<String>,
    html_url: String,
    #[serde(default)]
    stargazers_count: u32,
    language: Option<String>,
}

// Async function: fetch the user's most-recently-updated public repos.
// Returns an empty Vec on any failure so the UI degrades gracefully.
async fn fetch_repos() -> Vec<Repo> {
    let url = "https://api.github.com/users/RivasMario/repos?sort=updated&per_page=6";
    match gloo_net::http::Request::get(url).send().await {
        Ok(resp) => resp.json::<Vec<Repo>>().await.unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

// Live GitHub widget. This is the async-data lesson:
//   LocalResource = runs an async task in the browser and tracks its state.
//   <Suspense>    = shows a fallback until the resource resolves.
#[component]
pub fn GithubRepos() -> impl IntoView {
    // LocalResource (not Resource) because WASM fetch isn't Send/Sync.
    let repos = LocalResource::new(|| fetch_repos());

    view! {
        <Suspense fallback=move || view! { <p class="loading">"Loading repos\u{2026}"</p> }>
            {move || {
                // `.get()` returns Some(data) once loaded, None while pending.
                repos.get().map(|list| {
                    let list = list.to_vec();
                    if list.is_empty() {
                        view! { <p class="loading">"Couldn\u{2019}t load repos right now."</p> }
                            .into_any()
                    } else {
                        view! {
                            <div class="repos">
                                {list.into_iter().map(|r| view! {
                                    <a class="repo" href=r.html_url target="_blank">
                                        <div class="repo-name">{r.name}</div>
                                        <div class="repo-desc">
                                            {r.description.unwrap_or_else(|| "No description".to_string())}
                                        </div>
                                        <div class="repo-foot">
                                            {r.language.map(|l| view! { <span class="repo-lang">{l}</span> })}
                                            <span class="repo-stars">"\u{2605} "{r.stargazers_count}</span>
                                        </div>
                                    </a>
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}
