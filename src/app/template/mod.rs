use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub langs: Vec<Vec<&'a str>>,
}

#[derive(Template)]
#[template(path = "paste.html")]
pub struct Paste<'a> {
    pub html: &'a str,
    pub poster: &'a str,
    pub lang: &'a str,
    pub time: &'a str,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<'a> {
    pub title: &'a str,
    pub description: &'a str,
}
