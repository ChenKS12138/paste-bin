use actix_web::{get, web, HttpResponse, Responder};
use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle},
};

#[get("/theme/{theme}")]
async fn index(web::Path(theme): web::Path<String>) -> impl Responder {
    let ts = ThemeSet::load_defaults();
    if let Some(theme) = &ts.themes.get(&theme) {
        let css = css_for_theme_with_class_style(theme, ClassStyle::Spaced);
        HttpResponse::Ok().body(css)
    } else {
        HttpResponse::NotFound().body(format!("Not Found /theme/{}", theme))
    }
}
