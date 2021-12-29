use crate::app::config;
use crate::app::template;

use actix_web::{get, HttpResponse, Responder};
use askama::Template;
use serde_variant::to_variant_name;

#[get("/")]
async fn index() -> impl Responder {
    let index = template::Index {
        langs: config::lang::LANG_LIST
            .clone()
            .iter()
            .map(|(lang, description)| vec![to_variant_name(lang).unwrap(), *description])
            .collect(),
    };
    let body = index.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}
