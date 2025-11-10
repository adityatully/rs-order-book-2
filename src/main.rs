use rust_orderbook_2::orderbook::order_manager::OrderManager;
use tokio::macros;



#[tokio::main]
async fn main(){
    let manager = OrderManager::new();
}


pub fn start_process(){
    // this is the main prcess function 
    // here we can initlaise mutiple engines and then call the funtin engine::start_engine() 
    // start engine wud spawn a thread which wud contantly listen from the alloted shared memory file 
    // or another option would be to spawn the thread in this fucntion which the start_engine function having 
    // only the inifnite loop 
    // this main fucntion will also spawn the publisher thread 
}