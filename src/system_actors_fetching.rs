use crate::actix::ContextFutureSpawner;
use crate::actix::ActorFutureExt;
use crate::actix::WrapFuture;
use actix::{Actor, Addr, Context, Handler, Message};

use crate::serialize_own_vec::SerdeVec;

use crate::fetch_new_crypto::{
    fetch_news_coinmarketcap,
    fetch_address_crypto,
    Daum,
};
use crate::grading_machine_actor::{
    Grader, 
    MessageSymbol, 
    MessageAddress
};

pub enum UpdateCryptoStatus {
    Updated,
    FailedToUpdate,
    Error,
}

#[derive(Debug, Clone)]
pub struct CrytpoOfTheDay {
    response: Option<bool>
}

pub struct FetchCryptoDaily {
    pub content_type: String,
    pub key: String,
    pub address_grader: Addr<Grader>
}

impl Default for CrytpoOfTheDay {
    fn default() -> Self {
        CrytpoOfTheDay { response: None }     
    }
}

impl Actor for CrytpoOfTheDay {
    type Context = Context<Self>;

    fn started(&mut self , ctx: &mut Self::Context) { 
        tracing::info!("Starting My Fetcher Actor !")
    }
}

impl Message for FetchCryptoDaily {
    type Result = ();
}

impl Handler<FetchCryptoDaily> for CrytpoOfTheDay {
    type Result = ();

    fn handle(&mut self, msg: FetchCryptoDaily, ctx: &mut Context<Self>) -> Self::Result {
        async move {
            //
            //
            // Async move on thread two funtion utils to
            // fetch data crypto on coinmarketcap API
            //
            // To resolve async problem within Actor Model Actix
            // Send message type MessageSymbol and MessageAddress
            // to the Grader address inside async thread.
            //
            if let Ok(fetch_new) = fetch_news_coinmarketcap(&msg.content_type, &msg.key)
                .await
                .and_then(|vec_crypto| Ok(vec_crypto.data) ) { 
                    msg.address_grader.send( 
                        MessageSymbol { 
                            symbol_list: fetch_new.clone() 
                        }
                    ).await;

                let listed_symbol = SerdeVec {
                    list: fetch_new
                        .into_iter()
                        .map(|c| c.symbol)
                        .collect()
                    };

                let fetch_address = fetch_address_crypto(
                    &msg.content_type,
                    &msg.key, 
                    &listed_symbol
                )
                .await
                .and_then(|address| Ok(address.data
                    .into_iter()
                    .filter(|daum| daum.platform != None)
                    .filter(|daum| daum.rank != None)
                    .collect::<Vec<Daum>>())
                );    
                match fetch_address {
                    Ok(result) => {
                        msg.address_grader.send( 
                            MessageAddress { 
                                address_list: result
                            }
                        ).await
                    }
                    Err(e) => {
                        tracing::error!("Error to fetch address {:?}", e);
                        panic!("Error");
                        
                    }
                }
                        
            } else {
                tracing::error!("Error to fetch news");
                panic!("Error");
            }
        }
        .into_actor(self)
        .map(|_res, _act, _ctx| tracing::info!("Reqwest API Coinmarketcap succed"))
        .spawn(ctx);
    }
}

pub fn start_fetching() -> Addr<CrytpoOfTheDay> {
    CrytpoOfTheDay::default().start(/* CrytpoOfTheDay */)
}