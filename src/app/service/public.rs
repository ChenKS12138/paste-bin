use std::borrow::Cow;

use crate::app::assets::Assets;

use actix_web::{dev::Body, get, web, HttpResponse, Responder};
use mime_guess::from_path;

#[get("/public/{filepath:.*}")]
async fn index(web::Path(filepath): web::Path<String>) -> impl Responder {
    if let Some(content) = Assets::get(&filepath) {
        let body: Body = match content.data {
            Cow::Borrowed(bytes) => bytes.into(),
            Cow::Owned(bytes) => bytes.into(),
        };
        HttpResponse::Ok()
            .content_type(from_path(filepath).first_or_octet_stream().as_ref())
            .body(body)
    } else {
        HttpResponse::NotFound().body(format!("Not Found /public/{}", filepath))
    }
}
