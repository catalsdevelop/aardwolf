use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;

pub fn head(out: &mut Write, catalog: &Catalog, title: &str)
-> io::Result<()> {
write!(out, "<head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    <title>")?;
i18n!(catalog, title).to_html(out)?;
write!(out, "</title>\n    <link rel=\"stylesheet\" href=\"/web/app.css\" />\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"/stylesheets/scratchpad.css\" />\n</head>\n")?;
Ok(())
}