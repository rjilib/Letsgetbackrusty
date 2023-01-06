use actix::{Actor, Addr, Handler, Message};
use actix::{SyncArbiter, SyncContext};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use uuid::Uuid;
use crate::schema::{address, symbol};

// db executor actor
pub struct Database{
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Queryable, Debug)]
pub struct CryptoData {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub cmc_rank: i64,
    pub price: f64,
    pub volume_24: f64,
    pub percent_change_1: f64,
    pub percent_change_24: f64,
    pub percent_change_7: f64,
    pub market_cap: f64
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = symbol)]
pub struct NewCryptoData<'a>{
    pub id: Uuid,
    pub title: &'a str,
    pub symb: &'a str,
    pub cmc_rank: &'a str,
    pub price: &'a str,
    pub volume_24: &'a str,
    pub percent_change_1: &'a str,
    pub percent_change_24: &'a str,
    pub percent_change_7: &'a str,
    pub market_cap: &'a str
}

#[derive(Queryable, Debug)]
pub struct AddressCrypto {
    pub rank: i64, 
    pub name: String,
    pub symbol: String, 
    pub slug: String, 
    pub token_address: String 
}

#[derive(Insertable)]
#[diesel(table_name = address)]
pub struct NewAddressCrypto<'a> {
    pub id: Uuid,
    pub rank: &'a str,
    pub name: &'a str,
    pub symbol: &'a str,
    pub slug: &'a str,
    pub token_address: &'a str,
}

impl Actor for Database {
    type Context = SyncContext<Self>;

    fn started(&mut self , ctx: &mut Self::Context) {
        tracing::info!("Pool Database is On !")
    }
}

impl Message for CryptoData {
    type Result = ();
}

impl Message for AddressCrypto {
    type Result = ();
}

impl Handler<CryptoData> for Database {
    type Result = ();

    fn handle(&mut self, msg: CryptoData, ctx: &mut Self::Context) -> Self::Result {
        use self::symbol::dsl::*;
        
        let uuid = uuid::Uuid::new_v4();

        let new_crypto = NewCryptoData {
            id: uuid,
            title: &msg.name.as_str(),
            symb: &msg.symbol.as_str(),
            cmc_rank: &msg.cmc_rank.to_string(),
            price: &msg.price.to_string(),
            volume_24: &msg.volume_24.to_string(),
            percent_change_1: &msg.percent_change_1.to_string(),
            percent_change_24: &msg.percent_change_24.to_string(),
            percent_change_7: &msg.percent_change_7.to_string(),
            market_cap: &msg.market_cap.to_string()
        };

        let conn = &mut self.db_pool.get().unwrap();
        

        let insert_symbol = diesel::insert_into(symbol)
            .values(&new_crypto)
            .on_conflict(title)
            .do_update()
            .set(&new_crypto)
            //.do_nothing()
            .execute(conn);
  
        match insert_symbol {
            Ok(_) => tracing::info!(
                "Adding or Update'{}' as a new crypto symbol.",
                new_crypto.title,
                ),
            Err(e) => tracing::error!(
                "Failed to add '{}' as a new crypto symbol. {e}",
                new_crypto.title
                )
        }
    }
}

impl Handler<AddressCrypto> for Database {
    type Result = ();

    fn handle(&mut self, msg: AddressCrypto, ctx: &mut Self::Context) -> Self::Result {
        use self::address::dsl::*;

        let uuid = uuid::Uuid::new_v4();

        let new_addr = NewAddressCrypto {
            id: uuid,
            rank: &msg.rank.to_string(),
            name: msg.name.as_str(),
            symbol: msg.symbol.as_str(),
            slug: msg.slug.as_str(),
            token_address: msg.token_address.as_str()
        };

        let conn = &mut self.db_pool.get().unwrap();
        
        let insert_address = diesel::insert_into(address)
            .values(&new_addr)
            .on_conflict(token_address)
            .do_nothing()
            .execute(conn);

        match insert_address.unwrap() {
            1 => tracing::info!(
                "Adding '{}' ==> '{}'",
                new_addr.name,
                new_addr.token_address,
                ),
            0 => tracing::info!(
                "Already updated '{}'",
                new_addr.name,
                ),
            _ => tracing::error!(
                "Failed to add '{}' as a new crypto symbol.",
                new_addr.token_address
                )
        }
    }
}

pub fn start_db(database_url: String) -> Addr<Database> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool.");

    // => Uncomment this for first compilation
    //      Do SQL Unique
    //
    // diesel::sql_query("CREATE UNIQUE INDEX token_address ON address (token_address)")
    //     .execute(&mut pool.clone().get().unwrap()).unwrap();
    //
    // diesel::sql_query("CREATE UNIQUE INDEX title ON symbol (title)")
    //     .execute(&mut pool.clone().get().unwrap()).unwrap();

    SyncArbiter::start(2, move || Database{db_pool: pool.clone()})
}