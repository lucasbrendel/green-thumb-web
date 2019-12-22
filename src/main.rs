#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate rocket_contrib;
// extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
// extern crate serde_rusqlite;
extern crate log;
extern crate simplelog;
extern crate strum;
extern crate strum_macros;
#[macro_use]
extern crate diesel;
extern crate diesel_derive_enum;
extern crate dotenv;

#[allow(dead_code)]
pub mod data;
mod logging;
pub mod models;
pub mod plant;
mod routes;
pub mod schema;

use dotenv::dotenv;
use rocket_contrib::templates::Template;
use std::env;

pub fn rockets() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::index, routes::favicon, routes::static_file],
        )
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("wow", Box::new(routes::helper));
        }))
}

fn main() {
    logging::logging_init();
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _mgr = data::DataMgr::new(url);
    rockets().launch();
}
