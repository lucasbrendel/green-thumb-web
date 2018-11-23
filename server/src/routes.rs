use rocket;
use rocket::response::NamedFile;
use rocket::State;
use maud::{html, Markup};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// This is the entrypoint for our yew client side app.
#[get("/")]
fn index(db: State<Arc<sled::Tree>>) -> Markup {
    // maud macro
    html! {
        link rel="stylesheet" href="static/green-thumb.css" {}
        body {}
        // yew-generated javascript attaches to <body>
        script src=("static/green-thumb.js") {}
    }
}

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}

/// Serve static assets from the "static" folder.
#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

// TODO: remove this when we figure out how to change the native Rust
// WebAssembly's generated JavaScript code to point at "static/" prefix.
#[get("/ui.wasm")]
fn ugly_hack() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/ui.wasm")).ok()
}