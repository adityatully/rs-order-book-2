// jobs 
// will mantain userbalances acc to nuser ID 
// will expose functions such as check and lock funds for an order 
// expose functions to update balances after a trade or a fill
// it will constanlty recv data from thw queue as well as reply to the grpc requests 
// on reciving it performs checks and locks the funds for the order if valid 
// it passes the order to the engine to be processed 
// balance manager has 3 responsibilities  update balances from fills , reading from a channel 
// read from the SHM queue for the new order 
// The balanaces and holdings will be in a shared state for the grpc server and the balance manager 

use std::sync::Arc;

use crate::orderbook::types::{BalanceManagerError , MatchResult};
use crate::orderbook::order::Order;
pub trait BalanceManagerTrait{
    fn check_and_lock_funds(order : Order)->Result<() , BalanceManagerError>;
    fn update_funds_after_trade(order : Order)->Result<() , BalanceManagerError>;
}

pub struct SharedBalanceState{

}
pub struct MyBalanceManager{
    pub order_sender : crossbeam::channel::Sender<Order>,
    pub fill_recv : crossbeam::channel::Receiver<MatchResult>,
    pub state : Arc<SharedBalanceState>,
}

