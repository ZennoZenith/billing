use axum::response::Html;
use std::sync::OnceLock;
use tera::{Context, Tera};

use crate::{
    error::{Error, Result},
    web_config,
};

fn tera_instance() -> &'static Tera {
    static INSTANCE: OnceLock<Tera> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let mut tera = match Tera::new(&format!(
            "{}/**/*",
            web_config().TEMPLATE_FOLDER
        )) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {e}");
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    })
}

pub fn render(template_name: &str, context: &Context) -> Result<Html<String>> {
    tera_instance()
        .render(template_name, context)
        .map(Html)
        .map_err(Error::TeraRender)
}
