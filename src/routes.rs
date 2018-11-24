use rocket;
use rocket::response::NamedFile;
use rocket::State;
use maud::{html, Markup};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// This is the entrypoint
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
pub fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}