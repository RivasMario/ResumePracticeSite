// This file = the `components` module. It declares each child file as a
// sub-module and re-exports the components so other code can write
// `use crate::components::Header;` instead of `...::header::Header;`.

pub mod experience;
pub mod github;
pub mod header;
pub mod section;

pub use experience::ExperienceItem;
pub use github::GithubRepos;
pub use header::Header;
pub use section::Section;
