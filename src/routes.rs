use self::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use rocket::response::NamedFile;
use rocket_contrib::templates::{handlebars, Template};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct TemplateContext {
    parent: &'static str,
}

/// This is the entrypoint
#[get("/")]
pub fn index() -> Template {
    Template::render("index", &TemplateContext { parent: "layout" })
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

/// Took this from the Handlebars example on Rocket. This is probabaly not needed for anything.
pub fn helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::rockets;
    use rocket::http::Status;
    use rocket::local::{Client};

    // fn launch_and_get<'c>(route: &'c str) -> LocalResponse<'c> {
    //     Client::new(rockets()).unwrap().get(route).dispatch()
    // }

    #[test]
    fn test_index() {
        let client = Client::new(rockets()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_favicon() {
        let client = Client::new(rockets()).unwrap();
        let response = client.get("/favicon.ico").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_static() {
        let client = Client::new(rockets()).unwrap();
        let response = client.get("/static/favicon.ico").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
