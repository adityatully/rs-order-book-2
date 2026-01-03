use bounded_spsc_queue::Consumer;
use crate::{logger::types::{BalanceLogWrapper, BaseLogs, HoldingLogWrapper, OrderLogWrapper, TradeLogs}, shm::{balance_log_queue::BalanceLogQueue, holdings_log_queue::{self, HoldingLogQueue}, order_log_queue::OrderLogQueue, trade_log_queue::TradeLogQueue}};
use std::time::{SystemTime, UNIX_EPOCH};
pub struct LogReciever{
    pub order_log_shm_queue : OrderLogQueue,
    pub balance_log_shm_queue : BalanceLogQueue ,
    pub holding_log_shm_queue : HoldingLogQueue ,
    pub trade_log_queue       : TradeLogQueue,
    pub logs_recv_from_core : Consumer<BaseLogs> , 
    pub logs_recv_from_publisher : Consumer<TradeLogs>

}

impl LogReciever{
    pub fn new(logs_recv_from_core : Consumer<BaseLogs> , logs_recv_from_publisher : Consumer<TradeLogs>)->Self{
        let order_log_shm_queue = OrderLogQueue::open("/tmp/OrderLogs");
        let balance_log_shm_queue = BalanceLogQueue::open("/tmp/BalanceLogs");
        let holdings_log_queue = HoldingLogQueue::open("/tmp/HoldingLogs");
        let trade_log_queue = TradeLogQueue::open("/tmp/TradeLogs");
        if order_log_shm_queue.is_err(){
            eprintln!("failed to open the order log queue");
        }
        if balance_log_shm_queue.is_err(){
            eprintln!("failed to open the balance log queue");
        }
        if holdings_log_queue.is_err(){
            eprintln!("failed to open the holdings log queue");
        }
        if trade_log_queue.is_err(){
            eprintln!("failed to open the trade log queue");
        }

        Self{
            order_log_shm_queue :   order_log_shm_queue.unwrap(),
            balance_log_shm_queue : balance_log_shm_queue.unwrap(),
            holding_log_shm_queue : holdings_log_queue.unwrap(),
            logs_recv_from_core  ,
            trade_log_queue : trade_log_queue.unwrap() , 
            logs_recv_from_publisher
        }
    }

    pub fn run(&mut self){
        loop {
            if let Some(log) = self.logs_recv_from_core.try_pop(){
                // we get the deltas , we need to wrap them
                match log{
                    BaseLogs::BalanceDelta(balance_delta)=>{
                        let _ = self.balance_log_shm_queue.enqueue(BalanceLogWrapper{
                            balance_delta : balance_delta ,
                            timestamp : SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64,
                            severity  : 0 
                        });
                    }
                    BaseLogs::HoldingDelta(holdings_delta)=>{
                        let _ = self.holding_log_shm_queue.enqueue(HoldingLogWrapper{
                            holding_delta : holdings_delta ,
                            timestamp : SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64 , 
                            severity : 0 
                        });
                    }
                    BaseLogs::OrderDelta(order_delta)=>{
                        let _ = self.order_log_shm_queue.enqueue(OrderLogWrapper{
                            order_delta : order_delta ,
                            timestamp : SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64,
                            severity  : 0
                        });
                    }
                }
            }

            if let Some(trade_log) = self.logs_recv_from_publisher.try_pop(){
                let _ = self.trade_log_queue.enqueue(trade_log);
            }
        }
    }
}