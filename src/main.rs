#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

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
extern crate sled;
extern crate maud;

#[allow(dead_code)]
mod data;
mod logging;
mod routes;


fn main() {
    logging::logging_init();
    let _mgr = data::DataMgr::new(String::from("./db/green-thumb.db"));
    rocket::ignite().mount("/", routes![routes::index, routes::favicon, routes::static_file, routes::ugly_hack]).launch();
}