use bounded_spsc_queue::Consumer;
use chrono::Utc;
use crate::{logger::types::{Logs, SerialisedLogEntry}, shm::logger_queue::LogQueue};
const PAYLOAD_SIZE : usize = 67 ;
pub struct LogReciever{
    pub log_shm_queue : LogQueue,
    pub logs_recv_from_core : Consumer<Logs>
}

impl LogReciever{
    pub fn new(logs_recv_from_core : Consumer<Logs>)->Self{
        let log_shm_queue = LogQueue::open("/tmp/Logs");
        if log_shm_queue.is_err(){
            eprintln!("failed to open the log queue");
        }
        Self{
            log_shm_queue : log_shm_queue.unwrap(),
            logs_recv_from_core 
        }
    }

    pub fn run(&mut self){
        loop {
            if let Some(log) = self.logs_recv_from_core.try_pop(){
                // first we serialise it into a log entry 
                match log{
                    Logs::BalanceLogs(balance_log)=>{
                        match serde_json::to_vec(&balance_log){
                            Ok(data)=>{
                                let len = data.len();
                                let data_entry : [u8 ; PAYLOAD_SIZE] = data.try_into().expect("coundt convert vector into array");
                                let log_entry = SerialisedLogEntry{
                                    event_id : balance_log.event_id , 
                                    event_type : 0 ,
                                    timestamp : Utc::now().timestamp(),
                                    payload : data_entry,
                                    payload_len : len as u16
                                };
                                let _= self.log_shm_queue.enqueue(log_entry);
                            }
                            Err(_)=>{
                                eprint!("SERIALISATION ERROR")
                            }
                        }
                    } ,
                    Logs::HoldingsLogs(holding_log)=>{
                        match serde_json::to_vec(&holding_log){
                            Ok(data)=>{
                                let len = data.len();
                                let data_entry : [u8 ; PAYLOAD_SIZE] = data.try_into().expect("coundt convert vector into array");
                                let log_entry = SerialisedLogEntry{
                                    event_id : holding_log.event_id , 
                                    event_type : 1 ,
                                    timestamp : Utc::now().timestamp(),
                                    payload : data_entry,
                                    payload_len : len as u16
                                };
                                let _= self.log_shm_queue.enqueue(log_entry);
                            }
                            Err(_)=>{
                                eprint!("SERIALISATION ERROR")
                            }
                        }
                    } , 
                    Logs::OrderLogs(order_log)=>{
                        match serde_json::to_vec(&order_log){
                            Ok(data)=>{
                                let len = data.len();
                                let data_entry : [u8 ; PAYLOAD_SIZE] = data.try_into().expect("coundt convert vector into array");
                                let log_entry = SerialisedLogEntry{
                                    event_id : order_log.event_id , 
                                    event_type : 2 ,
                                    timestamp : Utc::now().timestamp(),
                                    payload : data_entry,
                                    payload_len : len as u16
                                };
                                let _= self.log_shm_queue.enqueue(log_entry);
                            }
                            Err(_)=>{
                                eprint!("SERIALISATION ERROR")
                            }
                        }
                    }
                }
            }
        }
    }
}