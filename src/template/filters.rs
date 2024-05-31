use std::convert::Infallible;

use time::{macros::format_description, OffsetDateTime};
use time_humanize::HumanTime;

use crate::dto::projects::ProjectTime;

pub fn humandtime(time: &ProjectTime) -> Result<String, Infallible> {
    let human_time =
        HumanTime::from_duration_since_timestamp(time.as_offset().unix_timestamp() as u64)
            - HumanTime::now();

    Ok(human_time.to_string())
}

pub fn humantime(time: &ProjectTime) -> Result<String, askama::Error> {
    let format_with_year = format_description!(
        "[day padding:none] [month repr:long] [year], [hour padding:none repr:24]:[minute] UTC"
    );
    let format_without_year = format_description!(
        "[day padding:none] [month repr:long], [hour padding:none repr:24]:[minute] UTC"
    );

    let format = if time.as_offset().year() == OffsetDateTime::now_utc().year() {
        format_without_year
    } else {
        format_with_year
    };

    time.as_offset()
        .format(format)
        .map_err(|error| askama::Error::Custom(Box::new(error)))
}

pub fn markdown(s: impl AsRef<str>) -> Result<String, Infallible> {
    let parser = pulldown_cmark::Parser::new(s.as_ref());
    let mut html = String::new();

    pulldown_cmark::html::push_html(&mut html, parser);

    Ok(html)
}
