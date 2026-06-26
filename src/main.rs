// Entry point. The real work lives in modules:
//   app        — the App component (page layout + resume data)
//   components — reusable building blocks (Header, Section, etc.)
//
// `mod x;` tells Rust "there's a file src/x.rs (or src/x/mod.rs), include it".
mod app;
mod components;

use app::App;

fn main() {
    // Readable panic messages in the browser console.
    console_error_panic_hook::set_once();
    // Mount the root component into <body> and start the app.
    leptos::mount::mount_to_body(App);
}
