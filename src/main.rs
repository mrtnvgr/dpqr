use actix_files::Files;
use actix_web::{post, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashSet;

mod vk;

lazy_static! {
    static ref ANSWERS: HashSet<&'static str> = HashSet::from([
        "взаимопомощь и взаимоуважение",
        "единство народов россии",
        "историческая память",
        "добро и справедливость",
        "мечта",
        "созидательный труд",
        "жизнь и достоинство",
        "патриотизм",
        "дружба",
        "служение отечеству",
        "крепкая семья"
    ]);
}

#[post("/api/submit")]
async fn submit(rbody: String) -> HttpResponse {
    let answers: HashSet<&str> = rbody.split(",").filter(|x| !x.is_empty()).collect();

    if answers == *ANSWERS {
        let mut pass: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();
        pass = pass.to_lowercase();

        match vk::send_pass_to_vk(&pass).await {
            Ok(_) => HttpResponse::Ok().body(pass),
            Err(_) => HttpResponse::BadRequest().finish(),
        }
    } else {
        return HttpResponse::BadRequest().finish();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(submit)
            .service(Files::new("/", "www").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
