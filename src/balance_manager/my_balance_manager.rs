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
use std::sync::atomic::{AtomicU64 , AtomicU32 };
use dashmap::DashMap;
use crate::orderbook::types::{BalanceManagerError , MatchResult};
use crate::orderbook::order::Order;
const MAX_USERS: usize = 10_000_000; // pre allocating for a max of 10 million users 
const MAX_SYMBOLS : usize = 100 ; 
const DEFAULT_BALANCE : u64 = 10000;
#[repr(C)]
#[repr(align(64))]  
pub struct UserBalance {
    pub user_id: AtomicU64,   // 8        
    pub available_balance: AtomicU64,      
    pub reserved_balance: AtomicU64,         
    pub total_traded_today: AtomicU64,  
    pub order_count_today: AtomicU32,   
    // 36 bytes , pad to 64 
    _padding: [u8; 28],  
}
impl Default for UserBalance{
    fn default()->Self{
        UserBalance { user_id: AtomicU64::new(0),
            available_balance: AtomicU64::new(DEFAULT_BALANCE), 
            reserved_balance: AtomicU64::new(0),
            total_traded_today: AtomicU64::new(0), 
            order_count_today: AtomicU32::new(0),
             _padding: [0 ; 28] 
            }
    }
}
pub struct UserHoldings{
    pub user_id: u64,     // 8 byte 
    pub user_holdings : [AtomicU32 ; MAX_SYMBOLS],
}
impl Default for UserHoldings{
    fn default() -> Self {
        UserHoldings {
            user_id: 0,
            user_holdings: unsafe { std::mem::zeroed() },  // Faster than from_fn
        }
    }
}
pub struct SharedBalanceState{
    pub balances : Arc<Box<[UserBalance ; MAX_USERS]>>,
    pub holdings : Arc<Box<[UserHoldings ; MAX_USERS]>>,
    pub user_id_to_index : Arc<DashMap<u64 , u32>>, // user_id to balance index 
    pub next_free_slot: AtomicU32,
    pub total_users: AtomicU32,

}
impl SharedBalanceState {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(Box::new(unsafe { std::mem::zeroed() })),
            holdings: Arc::new(Box::new(unsafe { std::mem::zeroed() })),
            user_id_to_index: Arc::new(DashMap::with_capacity(MAX_USERS)),
            next_free_slot: AtomicU32::new(0),
            total_users: AtomicU32::new(0),
        }
    }
}

impl Default for SharedBalanceState {
    fn default() -> Self {
        Self::new()
    }
}
pub struct MyBalanceManager{
    pub order_sender : crossbeam::channel::Sender<Order>,
    pub fill_recv : crossbeam::channel::Receiver<MatchResult>,
    pub state : Arc<SharedBalanceState>,
}
//pub trait BalanceManagerTrait{
//    fn check_and_lock_funds(order : Order)->Result<() , BalanceManagerError>;
//    fn update_funds_after_trade(order : Order)->Result<() , BalanceManagerError>;
//}

impl MyBalanceManager{
    pub fn new(order_sender : crossbeam::channel::Sender<Order> , fill_recv : crossbeam::channel::Receiver<MatchResult>)->(Self , Arc<SharedBalanceState>){
        let shared_state = Arc::new(SharedBalanceState::new());
        (Self { 
            order_sender, 
            fill_recv,
            state: Arc::clone(&shared_state)
         }
         , shared_state)
    }

    //// returned the state so that it can be passed to the grpc server 
    //pub fn check_and_lock_funds(&mut self , order : Order)->Result<() , BalanceManagerError>{
    //    // currently for liit orders , we get an order 
    //    
    //}
}