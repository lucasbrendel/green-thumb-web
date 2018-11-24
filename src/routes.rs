use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};
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
    out: &mut Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}
