use std::convert::Infallible;

use time_humanize::HumanTime;

use crate::dto::projects::ProjectTime;

pub fn humantime(time: &ProjectTime) -> Result<String, Infallible> {
    let human_time =
        HumanTime::from_duration_since_timestamp(time.as_offset().unix_timestamp() as u64)
            - HumanTime::now();

    Ok(human_time.to_string())
}

pub fn markdown(s: impl AsRef<str>) -> Result<String, Infallible> {
    let parser = pulldown_cmark::Parser::new(s.as_ref());
    let mut html = String::new();

    pulldown_cmark::html::push_html(&mut html, parser);

    Ok(html)
}
