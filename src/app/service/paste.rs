use crate::app::dao;
use crate::app::entity;
use crate::app::state;
use crate::app::template;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use serde_variant::to_variant_name;
use syntect::html::ClassStyle;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;

use askama::Template;
use base_62;
use html_escape;
use sha2::{Digest, Sha256};
use syntect::util::LinesWithEndings;

use crate::app::config::lang::Lang;
use validator::Validate;

#[get("/p/{key}")]
async fn query(
    web::Path(key): web::Path<String>,
    app_data: web::Data<state::AppState>,
) -> impl Responder {
    let mut paste_dao = dao::paste::Paste::new(app_data.connection.clone());
    if let Ok(paste_value) = paste_dao.query(&key) {
        let paste = template::Paste {
            html: &paste_value.html,
            plain: &paste_value.plain,
            poster: &paste_value.poster,
            lang: &paste_value.lang,
            time: &paste_value.time.to_rfc2822(),
        };
        let body = paste.render().unwrap();
        HttpResponse::Ok().content_type("text/html").body(body)
    } else {
        HttpResponse::Ok().content_type("text/html").body(
            template::Error {
                title: &format!("Key Missed {}", key),
                description: "Please Check Your Key",
            }
            .render()
            .unwrap(),
        )
    }
}

#[derive(Deserialize, Debug, Validate)]
struct CreatePasteDto {
    #[validate(length(min = 1))]
    poster: String,
    lang: Lang,
    #[validate(length(min = 1))]
    content: String,
    expire: usize,
}

#[post("/paste")]
async fn create(
    form: web::Form<CreatePasteDto>,
    app_data: web::Data<state::AppState>,
) -> impl Responder {
    let html_content = match &form.lang {
        Lang::PlainText => html_escape::encode_safe(&(&form).content).to_string(),
        _ => {
            let syntax_set = SyntaxSet::load_defaults_newlines();
            let paste_syntax = syntax_set
                .find_syntax_by_name(to_variant_name(&form.lang).unwrap())
                .unwrap();
            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                paste_syntax,
                &syntax_set,
                ClassStyle::Spaced,
            );
            for line in LinesWithEndings::from(&form.content) {
                html_generator.parse_html_for_line_which_includes_newline(line);
            }
            html_generator.finalize()
        }
    };

    let paste_entity = entity::Paste {
        lang: to_variant_name(&form.lang).unwrap().to_string(),
        html: html_content,
        plain: form.content.clone(),
        poster: form.poster.clone(),
        time: Utc::now(),
    };

    let mut hasher = Sha256::new();
    hasher.update(paste_entity.to_string());
    let key = base_62::encode(&hasher.finalize().to_vec());

    let mut paste_dao = dao::paste::Paste::new(app_data.connection.clone());
    paste_dao
        .create(
            &key,
            &paste_entity,
            if form.expire == 0 {
                None
            } else {
                Some(form.expire)
            },
        )
        .unwrap();
    HttpResponse::MovedPermanently()
        .set_header("Location", format!("/p/{}", &key))
        .finish()
}
