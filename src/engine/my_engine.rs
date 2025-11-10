use std::collections::HashMap;
use crate::orderbook::order::Order;
use crate::orderbook::types::{Event , PubLishError , PublishSuccess};
use crate::orderbook::order_book::OrderBook;
use crate::publisher::event_publisher::{self, EventPublisher};
use tokio::sync::mpsc;

pub trait Engine{
    fn add_book(&mut self , symbol : &str);
    fn get_book(&self , symbol : &str)->Option<&OrderBook>;
    fn get_book_mut(&mut self, symbol: &str) -> Option<&mut OrderBook>;
    fn remove_book(&mut self , symbol : &str);
    fn get_book_count(&self)->usize;
    fn has_book(&self , symbol : &str)->bool;
}

pub struct MyEngine{
    // the engine will own all the orderbooks
    pub book_count : usize, 
    pub books : HashMap< String , OrderBook>,
    pub event_publisher : kanal::Sender<Event>
}

impl MyEngine{
    pub fn new(event_publisher : kanal::Sender<Event>)->Self {
        // initialise the publisher channel here 
        
            Self{
                book_count : 0 ,
                books : HashMap::new(),
                event_publisher  ,
            } 
            
        
    }

    pub fn start_engine(){
        // the queue struct (shared memory file will be initialised by the producer )
        // we need to initlaise a queue struct here and then start listening to it in an infinite loop
        // on reciveing the order we shud call the match function after serialising the order 
    }

}

impl Engine for MyEngine{
    fn add_book(&mut self , symbol : & str) {
        // we need to intialise a new chnanel 
        // add it to the map 
        // add its sender to the map initialise a new order book with its sender and the clone of he event sender 
        let (sx , rx ) = mpsc::channel::<Order>(512);
        let new_book = OrderBook::new(symbol);
        self.books.insert(String::from(symbol) , new_book);
        self.book_count = self.book_count.saturating_add(1);


    }
    fn get_book(&self , symbol : &str)->Option<&OrderBook> {
       self.books.get(symbol).map(|orderbook| orderbook)
    }
    fn get_book_mut(&mut self, symbol: &str) -> Option<&mut OrderBook> {
        self.books.get_mut(symbol)
    }
    fn get_book_count(&self)->usize {
        self.book_count
    }
    fn has_book(&self, symbol: &str) -> bool {
        self.books.contains_key(symbol)
    }

    // cleaning up logic reqd 
    fn remove_book(&mut self , symbol : &str) {
        if self.books.contains_key(symbol){
            self.books.remove(symbol);
            self.book_count = self.book_count.saturating_sub(1);
        }
    }
    

}