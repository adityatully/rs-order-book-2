use bounded_spsc_queue::Consumer;
use chrono::Utc;
use crate::{logger::types::Logs, shm::{balance_log_queue::BalanceLogQueue, holdings_log_queue::{self, HoldingLogQueue}, order_log_queue::OrderLogQueue}};
const PAYLOAD_SIZE : usize = 67 ;
pub struct LogReciever{
    pub order_log_shm_queue : OrderLogQueue,
    pub balance_log_shm_queue : BalanceLogQueue ,
    pub holding_log_shm_queue : HoldingLogQueue ,
    pub logs_recv_from_core : Consumer<Logs>
}

impl LogReciever{
    pub fn new(logs_recv_from_core : Consumer<Logs>)->Self{
        let order_log_shm_queue = OrderLogQueue::open("/tmp/Logs");
        let balance_log_shm_queue = BalanceLogQueue::open("/tmp/BalanceLogs");
        let holdings_log_queue = HoldingLogQueue::open("/tmp/HoldingLogs");
        if order_log_shm_queue.is_err(){
            eprintln!("failed to open the log queue");
        }
        if balance_log_shm_queue.is_err(){
            eprintln!("failed to open the log queue");
        }
        if holdings_log_queue.is_err(){
            eprintln!("failed to open the log queue");
        }

        Self{
            order_log_shm_queue : order_log_shm_queue.unwrap(),
            balance_log_shm_queue : balance_log_shm_queue.unwrap(),
            holding_log_shm_queue : holdings_log_queue.unwrap(),
            logs_recv_from_core  
        }
    }

    pub fn run(&mut self){
        loop {
            if let Some(log) = self.logs_recv_from_core.try_pop(){
                // first we serialise it into a log entry 
                match log{
                    Logs::BalanceLogs(balance_log)=>{
                        let _ = self.balance_log_shm_queue.enqueue(balance_log);
                    } ,
                    Logs::HoldingsLogs(holding_log)=>{
                        let _ = self.holding_log_shm_queue.enqueue(holding_log);
                    } , 
                    Logs::OrderLogs(order_log)=>{
                        let _ = self.order_log_shm_queue.enqueue(order_log);
                    }
                }
            }
        }
    }
}