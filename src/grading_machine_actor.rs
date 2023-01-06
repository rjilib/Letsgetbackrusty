use actix::{Actor, Addr, Context, Handler, Message};
use crate::fetch_new_crypto::{Data, Daum};
use crate::database::{Database, AddressCrypto, CryptoData};


#[derive(Debug)]
pub struct Grader {
    address_actor_db: Option<Addr<Database>>
}

pub struct TransferAddressDb {
    pub address_db: Addr<Database>
}

#[derive(Debug, Clone)]
pub struct MessageSymbol {
    pub symbol_list: Vec<Data>
}

#[derive(Debug, Clone)]
pub struct MessageAddress {
    pub address_list: Vec<Daum>
}

impl Actor for Grader {
    type Context = Context<Self>;

    fn started(&mut self , ctx: &mut Self::Context) { 
        tracing::info!("Starting Grader !")
    }
}

impl Default for Grader {
    fn default() -> Self {
        Self { 
            address_actor_db: None,
        }
    }
}

impl Message for TransferAddressDb {
    type Result = ();
}

impl Message for MessageSymbol {
    type Result = ();
}

impl Message for MessageAddress {
    type Result = ();
}

impl Handler<TransferAddressDb> for Grader {
    type Result = ();

    fn handle(&mut self, msg: TransferAddressDb, ctx: &mut Self::Context) -> Self::Result {
        self.address_actor_db = Some(msg.address_db);
    }
}

impl Handler<MessageSymbol> for Grader {
    type Result = ();

    fn handle(&mut self, msg: MessageSymbol, ctx: &mut Context<Self>) -> Self::Result {
        msg.symbol_list.into_iter()
            .for_each(|symbol| {
                self.address_actor_db.as_ref().unwrap().do_send(
                    CryptoData {
                        id: symbol.id,
                        name: symbol.name,
                        symbol: symbol.symbol,
                        cmc_rank: symbol.cmc_rank,
                        price: symbol.quote.usd.price,
                        volume_24: symbol.quote.usd.volume_24h,
                        percent_change_1: symbol.quote.usd.percent_change_1h,
                        percent_change_24: symbol.quote.usd.percent_change_24h,
                        percent_change_7: symbol.quote.usd.percent_change_7d,
                        market_cap: symbol.quote.usd.market_cap
                    }
                )
            }
        )
    }
}

impl Handler<MessageAddress> for Grader {
    type Result = ();

    fn handle(&mut self, msg: MessageAddress, ctx: &mut Context<Self>) -> Self::Result {         
        msg.address_list.into_iter()
            .for_each(|token| {
                self.address_actor_db.as_ref().unwrap().do_send(
                AddressCrypto {
                    rank: token.rank.unwrap(),
                    name: token.name,
                    symbol: token.symbol,
                    slug: token.platform.as_ref()
                        .and_then(|p| Some(p.slug.clone()))
                        .unwrap(),
                    token_address: token.platform.as_ref()
                        .and_then(|p| Some(p.token_address.clone()))
                        .unwrap()
                }
            );
        });
    }
}

pub fn start_grader() -> Addr<Grader> {
    Grader::default().start()
}