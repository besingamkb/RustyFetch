use actix_web::{web, App, HttpServer, HttpResponse};
use tera::{Tera, Context};
use serde::Deserialize;

#[derive(Deserialize)]
struct YoutubeLinkFormData {
    youtube_link: String,
}
async fn index() -> HttpResponse {
    println!("Index");
    let tera: Tera = Tera::new("templates/**/**").unwrap();
    let rendered: String = tera.render("main.html", &Context::new()).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn download_video(req: web::Form<YoutubeLinkFormData>) -> HttpResponse {
    println!("download_video");
    println!("Youtube link is: {}", req.youtube_link);
    rustube::download_best_quality(&*req.youtube_link).await.unwrap();
    HttpResponse::Ok().body(format!("Youtube link is: {}!", req.youtube_link))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/",  web::get().to(index))
            .route("/download", web::post().to(download_video))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
