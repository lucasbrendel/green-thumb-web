#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate strum;
extern crate strum_macros;

mod logging;
mod routes;

use rocket_contrib::templates::Template;

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
    rockets().launch();
}
