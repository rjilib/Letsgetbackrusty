use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use serde::Deserialize;
use std::net::TcpListener;
use actix::Addr;

use crate::fetch_crypto::fecth_crypto_symbol;
use crate::grading_machine_actor::Grader;
use crate::system_actors_fetching::{
    CrytpoOfTheDay, 
    FetchCryptoDaily
};


#[derive(Deserialize, Debug, Default)]
struct SymbolBinance {
    symbol: String,
    devise: String
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub address_daily: Addr<CrytpoOfTheDay>,
    pub content_type: String,
    pub api_key: String,
    pub address_grader: Addr<Grader>
}

async fn crypto(form: web::Form<SymbolBinance>) -> HttpResponse {
    let response = fecth_crypto_symbol(
        &form.symbol, &form.devise).await;
    HttpResponse::Ok().body(format!("{:?}", response.unwrap()))
}

async fn fetch_news_crypto(state: web::Data<AppState>) -> HttpResponse {
    let response = state.address_daily.send(FetchCryptoDaily { 
        content_type: state.content_type.clone(), 
        key: state.api_key.clone(),
        address_grader: state.address_grader.clone()
    }).await;
    HttpResponse::Ok().body(format!("{:#?}", response.unwrap()))
}

async fn check() -> HttpResponse {
    HttpResponse::Ok().body("UP\n")
}


pub async fn start(socket: TcpListener, state: AppState) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .route("/check", web::get().to(check))
            .route("/crypto", web::get().to(crypto))
            .route("/news", web::get().to(fetch_news_crypto))
    })
    .listen(socket)?
    .run().await?;
    Ok(())
}