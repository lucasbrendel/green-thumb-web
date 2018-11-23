#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde_rusqlite;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate log;
extern crate simplelog;

#[allow(dead_code)]
mod data;
mod logging;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}

fn main() {
    logging::logging_init();
    let _mgr = data::DataMgr::new(String::from("./db/green-thumb.db"));
    rocket::ignite().mount("/", routes![index, favicon]).launch();
}
