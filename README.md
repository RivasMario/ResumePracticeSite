# ResumePracticeSite

Personal resume website for Mario J. Rivas — built in **Rust + [Leptos](https://leptos.dev)**, compiled to WebAssembly. No JavaScript written by hand.

**Live:** https://rivasmario.github.io/ResumePracticeSite/

## Stack
- **Leptos 0.8** (CSR) — reactive UI in Rust → WASM
- **trunk** — build tool + dev server
- **gloo-net + serde** — live GitHub repo fetch from the public API

## Develop
```bash
# one-time setup
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# run with live reload at http://localhost:8080
trunk serve --open
```

## Build for production
```bash
trunk build --release --public-url /ResumePracticeSite/
# output in ./dist
```

## Structure
```
src/
  main.rs              entry point
  app.rs               page layout + resume data
  components/
    header.rs          banner, theme toggle, print, mountain skyline
    section.rs         titled section wrapper
    experience.rs      one job/project/education entry
    github.rs          live GitHub repos widget
style/main.css         PNW theme (dark/light via data-theme)
index.html             trunk entry document
.github/workflows/     auto-deploy to GitHub Pages on push to main
```

Deploys automatically via GitHub Actions on every push to `main`.
