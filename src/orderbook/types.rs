use std::sync::Arc;
use crate::{orderbook::order::Side, publisher::event_publisher::EventPublisher};
pub type OrderId = u64;
use thiserror::Error;
use serde::{Serialize, Deserialize };


#[derive(Debug , Clone , Copy)]
pub struct Fill{
    pub price : u64 , 
    pub quantity : u32 ,
    // taker orderid -> incoming order tht caused the match 
    pub taker_order_id : OrderId,
    // maker -> the order that was on the book that caused the match 
    pub maker_order_id : OrderId , 
    pub taker_side : Side ,
    pub maker_user_id : u64 ,
    pub taker_user_id : u64 , 
    pub symbol : u32
}

impl Fill{
    pub fn new(price:u64 , quantity:u32 , taker_order_id : OrderId , maker_order_id : OrderId , maker_user_id : u64 ,  taker_user_id : u64 ,  symbol : u32 , taker_side : Side) -> Self{
        Self{
             price  ,
             quantity , 
             taker_order_id ,
             maker_order_id , 
             maker_user_id , 
             taker_user_id , 
             symbol ,
             taker_side
        }
    }

    pub fn total_volume(&self)->u64{
        self.price * self.quantity as u64
    }
}
#[derive(Debug  , Clone)]
pub struct Fills{
    pub fills : Vec<Fill>
}

impl Fills{
    pub fn new()->Self{
        Self{
            fills : Vec::with_capacity(1000)
        }
    }

    pub fn add(&mut self , fill : Fill){
        self.fills.push(fill);
    }

}
// this can be given back to the Api using the pubsub
#[derive(Debug , Clone)]
pub struct MatchResult{
    /// The ID of the incoming order that initiated the match
    pub order_id : OrderId , 
    pub fills : Fills,
    pub remaining_qty : u32,
}

impl MatchResult{
    pub fn new(order_id: OrderId, initial_quantity: u32)->Self{
        Self { order_id , fills: Fills::new(), remaining_qty: initial_quantity }
    }
    pub fn add_transaction(&mut self , fill : Fill){
       self.remaining_qty =  self.remaining_qty.saturating_sub(fill.quantity);
       self.fills.add(fill);
    }
}

#[derive(Debug , Error)]
pub enum OrderBookError{
    // aff errors that can occour 
}


#[derive(Debug , Error)]
pub enum PubLishError{

}

pub enum PublishSuccess{
    
}
#[derive(Debug )]
pub enum BalanceManagerError{
    InsufficientFunds ,
    BalanceLockingFailed , 
    UserNotFound ,
    BalanceNotFound,
    InvalidSymbol 
}

pub struct BalanceInfo{
    
}
#[derive(Debug )]
pub enum ShmReaderError{
    QueueError
}
#[derive(Debug )]
pub enum QueueError{
    QueueFullError , 
    QueueEmptyError
}


#[derive(Debug)]
pub struct Event {
    market_update :  MarketUpdateAfterTrade
}

impl Event{
    pub fn new(market_update : MarketUpdateAfterTrade)->Self{
        Self { market_update }
    }
}
#[derive(Debug)]
pub struct MarketUpdateAfterTrade{
    pub symbol : u32 ,
    pub last_traded_price : u64 , 
    pub depth : (Vec<[String ; 3]> , Vec<[String ; 3]>),
    pub event_time : i64 ,
    pub trade_time : i64 , 
    pub match_result : MatchResult
}
impl MarketUpdateAfterTrade {
    pub fn new(symbol : u32 , last_traded_price : u64 ,  depth : (Vec<[String ; 3]> , Vec<[String ; 3]>), event_time : i64 ,trade_time : i64 , match_result : MatchResult)->Self{
        Self { symbol , last_traded_price, depth, event_time, trade_time ,  match_result }
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct TickerData {
    #[serde(rename = "e")]
    pub event: String, // "ticker"

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "E")]
    pub event_time: i64,

    #[serde(rename = "p")]
    pub price: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DepthData{
    #[serde(rename = "e")]
    pub event: String,         

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "E")]
    pub event_time: i64,

    #[serde(rename = "T")]
    pub trade_time: i64,

    #[serde(rename = "U")]
    pub first_id: i64,

    #[serde(rename = "u")]
    pub last_id: i64,

    #[serde(rename = "b")]
    pub bids: Vec<[String; 2]>,  

    #[serde(rename = "a")]
    pub asks: Vec<[String; 2]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeData {
    #[serde(rename = "e")]
    pub event: String,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "E")]
    pub event_time: i64,

    #[serde(rename = "T")]
    pub trade_time: i64,

    #[serde(rename = "t")]
    pub trade_id: i64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub quantity: String,

    #[serde(rename = "a")]
    pub buyer_order_id: String,

    #[serde(rename = "b")]
    pub seller_order_id: String,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}


// the trade data will be extracted by each fills by the manager 