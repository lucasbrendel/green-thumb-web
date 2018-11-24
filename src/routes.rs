use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

/// This is the entrypoint
#[get("/")]
pub fn index() -> &'static str {
    // maud macro
    "Hello, World"
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
