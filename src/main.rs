use env_logger::Env;
use std::net::TcpListener;
use dotenv;

use letsgetbackrusty::web::{start, AppState};
use letsgetbackrusty::system_actors_fetching::start_fetching;
use letsgetbackrusty::grading_machine_actor::{start_grader, TransferAddressDb};
use letsgetbackrusty::database::start_db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let api_key = dotenv::var("API_KEY").expect("Could not find API KEY").to_owned();
    let content_type = dotenv::var("CONTENT_TYPE").expect("Could not find CONTENT-TYPE").to_owned();
    let port = dotenv::var("PORT").expect("Could not find CONTENT-TYPE").to_owned();
    let db_url = dotenv::var("DATABASE_URL").expect("Could not find Url database").to_owned();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let add_fetcher = start_fetching();
    let add_grader = start_grader();
    let add_db = start_db(db_url);


    let state = AppState {
        address_daily: add_fetcher,
        content_type: content_type,
        api_key: api_key,
        address_grader: add_grader.clone()
    };

    add_grader.send(TransferAddressDb { address_db: add_db }).await;

    let address = format!("127.0.0.1:{}", port);
    let socket = TcpListener::bind(address)?;

    start(socket, state).await?;
    Ok(())
}
