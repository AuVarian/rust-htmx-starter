// src/routes/home.rs
use actix_web::HttpRequest;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct HomeTemplate {

}

pub async fn index(_req: HttpRequest) -> HomeTemplate {
    HomeTemplate {  }
}
