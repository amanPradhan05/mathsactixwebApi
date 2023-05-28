use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
#[get("/add/{num1}/{num2}")]
async fn add(info:web::Path<(i32,i32)>)->impl Responder{
    let result=info.0+info.1;
    HttpResponse::Ok().body(format!("Result:{}",result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(add)
            
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}