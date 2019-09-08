use actix_web::{web, App, HttpResponse, HttpServer, Responder, guard};
use actix_web::get;
use std::sync::Mutex;
mod te;
struct AppState {
    app_name: String,
}
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

fn indexname(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[rustfmt::skip]
fn main() {
//    let sys = actix_rt::System::new("example");
    HttpServer::new(|| {
        App::new()
            .service(index3)
//            .service(web::scope("/app")
//                .route("/index.html",web::get().to(index)))
            .route("/", web::get().to(index))

            .route("/again", web::get().to(index2))
            .data(AppState{
                app_name:String::from("Actix-web")
            })
            .route("/name",web::get().to(indexname))
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .route("/mm", web::to(|| HttpResponse::Ok()))
            .configure(te::config)
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();

//    let _=sys.run();
}